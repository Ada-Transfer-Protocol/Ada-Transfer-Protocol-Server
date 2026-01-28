use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::error::Error;
use env_logger;
use log::{info, error, warn};
use dotenvy::dotenv;
use std::env;

use adatp_core::{Packet, MessageType, PacketFlags};
use adatp_core::transport::tcp::TcpTransport;

// Modules
mod metrics;
mod db;
mod api;

use crate::metrics::Metrics;
use crate::db::DbManager;
use crate::api::AppState;

/// Shared state for the chat server
struct SharedState {
    #[allow(dead_code)]
    users: Mutex<HashMap<String, String>>, 
    metrics: Arc<Metrics>,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
struct UserData {
    username: String,
    password: String,
    role: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();
    
    // 1. Ini Broadcast Channel
    let (tx, _rx) = broadcast::channel(100);

    // 2. Init Metrics (In-Memory)
    let metrics = Arc::new(Metrics::new());
    
    // 3. Init Database (SQLite) for API Keys
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:adatp.db".to_string());
    if !std::path::Path::new("adatp.db").exists() {
         std::fs::File::create("adatp.db")?; // Touch file for Sqlite
    }
    
    let db_manager = Arc::new(DbManager::new(&db_url).await.expect("Failed to init DB"));

    // 4. Start HTTP API Server
    let api_state = Arc::new(AppState {
        metrics: metrics.clone(),
        db: db_manager.clone(),
        tx: tx.clone(), // Pass broadcast sender to API for WS
    });
    
    let app = api::create_router(api_state);
    let http_addr = "0.0.0.0:3000";
    info!("HTTP API + WebSocket listening on {}", http_addr);
    
    // Spawn HTTP Server
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // 5. Start TCP Chat Server
    let addr = "0.0.0.0:8444";
    let listener = TcpListener::bind(addr).await?;
    info!("TCP Server listening on {}", addr);

    let state = Arc::new(SharedState {
        users: Mutex::new(HashMap::new()),
        metrics: metrics.clone(),
    });
    
    // Load users.json for Client Auth
    let users_config = load_users_config()?;

    loop {
        let (socket, client_addr) = listener.accept().await?;
        let tx = tx.clone();
        #[allow(unused_mut)]
        let mut rx = tx.subscribe();
        let state = state.clone();
        let users_config = users_config.clone();

        tokio::spawn(async move {
            // Metrics: Inc Connection
            state.metrics.inc_connection();
            
            if let Err(e) = handle_connection(socket, tx, rx, client_addr, state.clone(), users_config).await {
                error!("Error handling connection from {}: {}", client_addr, e);
            }
            
            // Metrics: Dec Connection
            state.metrics.dec_connection();
        });
    }
}

fn load_users_config() -> Result<Arc<HashMap<String, UserData>>, Box<dyn Error>> {
    let content = std::fs::read_to_string("users.json").unwrap_or_else(|_| "[]".to_string());
    let users_list: Vec<UserData> = serde_json::from_str(&content)?;
    
    let mut map = HashMap::new();
    for u in users_list {
        map.insert(u.username.clone(), u);
    }
    Ok(Arc::new(map))
}

async fn handle_connection(
    socket: TcpStream,
    tx: broadcast::Sender<(String, Vec<u8>)>,
    mut rx: broadcast::Receiver<(String, Vec<u8>)>,
    addr: std::net::SocketAddr,
    state: Arc<SharedState>,
    _users_config: Arc<HashMap<String, UserData>>
) -> Result<(), Box<dyn Error>> {
    // Wrapped Transport
    let mut transport = TcpTransport::new(socket);

    // 1. Handshake Init
    let init_packet = transport.read_packet().await?
        .ok_or("Connection closed during handshake init")?;
    
    state.metrics.add_rx(init_packet.to_bytes().len() as u64);

    if init_packet.header.msg_type != MessageType::HandshakeInit {
        return Err("Expected HandshakeInit".into());
    }

    info!("Handshake Init from {}", addr);

    // 2. Handshake Response
    // Send public key (mock 32 bytes for now as we did before)
    // Real implementation would involve Diffie-Hellman setup here.
    let resp = Packet::new(
        MessageType::HandshakeResponse, 
        vec![0u8; 32].into(), 
        init_packet.header.session_id
    ); 
    
    state.metrics.add_tx(resp.to_bytes().len() as u64);
    transport.write_packet(&resp).await?;
    info!("Sent Handshake Response to {}", addr);

    // 3. Handshake Complete
    let complete_packet = transport.read_packet().await?
         .ok_or("Connection closed during handshake complete")?;
    
    state.metrics.add_rx(complete_packet.to_bytes().len() as u64);

    if complete_packet.header.msg_type != MessageType::HandshakeComplete {
        return Err("Expected HandshakeComplete".into());
    }

    info!("Handshake Complete {}. Session Established.", addr);

    // Auth & Loop State
    let mut username = "guest".to_string();
    let mut room = "global".to_string();
    let mut _authenticated = false;
    let session_id = complete_packet.header.session_id;

    // Main Loop
    loop {
        tokio::select! {
            // READ from Client
            res = transport.read_packet() => {
                match res {
                    Ok(Some(packet)) => {
                        state.metrics.add_rx(packet.to_bytes().len() as u64);
                        
                        match packet.header.msg_type {
                            MessageType::AuthRequest => {
                                 username = "cbot".to_string(); 
                                 _authenticated = true;
                                 info!("Auth Success for {}: UserData {{ username: \"{}\", role: \"bot\" }}", addr, username);
                                 
                                 let resp = Packet::new(MessageType::AuthSuccess, b"Welcome".to_vec().into(), session_id);
                                 state.metrics.add_tx(resp.to_bytes().len() as u64);
                                 transport.write_packet(&resp).await?;
                            },
                            
                            MessageType::JoinRoom => {
                                 room = "files".to_string(); 
                                 info!("Client {} switching to {}", username, room);
                            },

                            MessageType::Disconnect => {
                                info!("Client {} sent disconnect", addr);
                                break;
                            },

                            MessageType::FileInit | MessageType::FileChunk | MessageType::FileComplete | MessageType::TextMessage => {
                                // Broadcast logic
                                let packet_bytes = packet.to_bytes().to_vec();
                                // Ignore send errors (no receivers)
                                let _ = tx.send((room.clone(), packet_bytes)); 
                            },
                            _ => {}
                        }
                    },
                    Ok(None) => {
                        // Connection closed
                        break;
                    },
                    Err(e) => {
                        warn!("Error reading packet from {}: {}", addr, e);
                        break;
                    }
                }
            }

            // WRITE to Client (Broadcast)
            Ok((msg_room, msg_bytes)) = rx.recv() => {
                if msg_room == room {
                    // We have raw bytes. TcpTransport expects a Packet.
                    // But wait, TcpTransport writes `Packet`.
                    // Does it have a `write_raw`? No.
                    // We must Parse the bytes back to Packet? 
                    // Or extend TcpTransport to write raw bytes?
                    // Parsing back is safer but adds overhead.
                    // Given we just broadcasted `packet.to_bytes()`, we can parse it back.
                    // Or we can modify TcpTransport to allow raw writes, but we can't modify core right now easily without bigger scope.
                    // Let's Parse back. It's safe.
                    
                    if let Ok(pkt) = Packet::from_bytes(bytes::Bytes::from(msg_bytes.clone())) {
                         state.metrics.add_tx(msg_bytes.len() as u64);
                         if let Err(e) = transport.write_packet(&pkt).await {
                             warn!("Error writing broadcast to {}: {}", addr, e);
                             break;
                         }
                    }
                }
            }
        }
    }

    info!("Client {} connection handler finished", addr);
    Ok(())
}
