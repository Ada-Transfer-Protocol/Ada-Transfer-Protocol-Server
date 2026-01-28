use axum::{
    routing::get,
    Router,
    Json,
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    middleware::{self, Next},
};
use axum::extract::Request;
use std::sync::Arc;
use serde_json::json;
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use bytes::Bytes;

use crate::metrics::Metrics;
use crate::db::DbManager;
use adatp_core::{Packet, MessageType}; 

pub struct AppState {
    pub metrics: Arc<Metrics>,
    pub db: Arc<DbManager>,
    pub tx: broadcast::Sender<(String, Vec<u8>)>,
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    if request.uri().path() == "/ws" {
        return next.run(request).await;
    }

    let api_key = headers
        .get("x-api-key")
        .and_then(|val| val.to_str().ok());

    match api_key {
        Some(key) => {
            match state.db.validate_key(key).await {
                Ok(true) => next.run(request).await,
                _ => (StatusCode::UNAUTHORIZED, "Invalid or Inactive API Key").into_response(),
            }
        }
        None => (StatusCode::UNAUTHORIZED, "Missing x-api-key header").into_response(),
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .route("/status", get(status_handler))
        .route("/metrics", get(metrics_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));
        
    Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(ws_handler))
        .nest("/api", api_routes)
        .with_state(state)
}

async fn root_handler() -> &'static str {
    "AdaTP Server is running! 🚀\nWS Endpoint: /ws"
}

async fn status_handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "adatp-server" }))
}

async fn metrics_handler(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let snapshot = state.metrics.snapshot();
    Json(json!(snapshot))
}

// --- WebSocket Logic ---

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    state.metrics.inc_connection();

    // State Tracking
    let mut room = "global".to_string();
    let mut connected_session_id = None; // Store the UUID of the client

    let (ws_tx, mut ws_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(100);
    
    // 1. Write Loop
    let write_task = tokio::spawn(async move {
        while let Some(data) = ws_rx.recv().await {
            if sender.send(Message::Binary(data)).await.is_err() {
                break;
            }
        }
    });

    // 2. Main Logic Loop
    let mut broadcast_rx = state.tx.subscribe();
    
    loop {
        tokio::select! {
            // A. Incoming from WebSocket (Client)
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Binary(data))) => {
                        state.metrics.add_rx(data.len() as u64);
                        if let Ok(packet) = Packet::from_bytes(Bytes::from(data.clone())) {
                            
                            // Capture ID from first valid packet
                            if connected_session_id.is_none() {
                                connected_session_id = Some(packet.header.session_id);
                            }

                            match packet.header.msg_type {
                                MessageType::JoinRoom => {
                                    // Parse Room Name from Payload
                                    if let Ok(new_room) = std::str::from_utf8(&packet.payload) {
                                        room = new_room.to_string();
                                        println!("Client joined room: {}", room);
                                    } else {
                                        // Demo Fallback if payload empty/invalid
                                        // room = "conf".to_string(); 
                                        println!("JoinRoom failed: invalid payload");
                                    }
                                },
                                MessageType::AuthRequest => {
                                    // Respond with Success
                                    let resp = Packet::new(MessageType::AuthSuccess, Bytes::from("Access Granted"), packet.header.session_id);
                                    let _ = ws_tx.send(resp.to_bytes().to_vec()).await;
                                },
                                MessageType::TextMessage | MessageType::FileInit | MessageType::FileChunk | MessageType::FileComplete | MessageType::VoiceData | MessageType::VideoData => {
                                     // Broadcast to Room
                                     let _ = state.tx.send((room.clone(), data));
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break, // Disconnect
                    _ => {}
                }
            }

            // B. Incoming from Broadcast
            Ok((msg_room, msg_bytes)) = broadcast_rx.recv() => {
                if msg_room == room {
                    // Don't echo back to sender? (Echo cancellation logic is better handled on client for now as we don't parse sender ID here efficiently every time)
                    state.metrics.add_tx(msg_bytes.len() as u64);
                    if ws_tx.send(msg_bytes).await.is_err() {
                        break;
                    }
                }
            }
        }
    }

    // --- DISCONNECT HANDLER (REALTIME EXIT) ---
    if let Some(session_id) = connected_session_id {
        // Create a 'PresenceUpdate' packet with payload "LEAVE"
        // And send it to the room so others know this ID is gone.
        let leave_packet = Packet::new(MessageType::PresenceUpdate, Bytes::from("LEAVE"), session_id);
        let _ = state.tx.send((room, leave_packet.to_bytes().to_vec()));
    }

    write_task.abort();
    state.metrics.dec_connection();
}
