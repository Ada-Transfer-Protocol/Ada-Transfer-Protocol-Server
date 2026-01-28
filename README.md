# Ada Transfer Protocol (Server)

![AdaTP](https://img.shields.io/badge/AdaTP-v2.0-blueviolet?style=for-the-badge) ![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?style=for-the-badge) ![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge) ![Uptime](https://img.shields.io/badge/Uptime-99.9%25-success?style=for-the-badge)

**AdaTP (Ada Transfer Protocol)** is a next-generation, high-performance real-time communication server built with Rust. It is designed to handle massive concurrency for Voice (VoIP), Signaling, and File Transfer with ultra-low latency.

Unlike traditional heavy protocols (SIP/WebRTC stacks), AdaTP uses a **lightweight binary framing protocol** over WebSocket/TCP, making it ideal for AI Agents, IoT devices, and High-Frequency Trading systems.

---

## 🏗 System Architecture

AdaTP is built on the **Tokio** asynchronous runtime, utilizing a **Message-Passing Actor Model** for state management.

*   **Networking Layer**: Uses `tokio-tungstenite` for WebSocket handling. Supports Binary frames directly (no Base64 overhead).
*   **State Management**: In-memory `ACID` compliant state maps protected by `RwLock` and `DashMap` for O(1) access times.
*   **Packet Routing**: Efficient broadcasting engine that routes audio packets (`0x0044`) without decoding/encoding (Zero-Copy forwarding).
*   **Persistence**: Uses `SQLite` (via SQLx) for user authentication and transaction logging.

---

## 💻 System Requirements

AdaTP is extremely efficient. It can run on a Raspberry Pi or a high-end server.

| Requirement | Minimum | Recommended (10k+ Users) |
| :--- | :--- | :--- |
| **OS** | Linux (Any), macOS, Windows | Ubuntu 22.04 / Debian 11 |
| **CPU** | 1 Core (Arm/x64) | 4+ Cores (High Frequency) |
| **RAM** | 512 MB | 8 GB+ |
| **Network** | 10 Mbps Up/Down | 1 Gbps+ (Low Jitter) |
| **Storage** | 100 MB free space | NVMe SSD (for DB logs) |

---

## 🚀 Installation & Deployment

### One-Line Automated Install (Universal Linux)

This script auto-detects your OS, installs dependencies (Rust, GCC, SSL), builds the server, and sets up a systemd service (`adatp-server`).

```bash
curl -sSL https://raw.githubusercontent.com/Ada-Transfer-Protocol/Server/main/tools/setup.sh | bash
```

### Manual Build (Dev Mode)

```bash
git clone https://github.com/Ada-Transfer-Protocol/Server.git
cd Server
cargo run --bin adatp-server
```

### Uninstall
To completely remove AdaTP from your system:
```bash
curl -sSL https://raw.githubusercontent.com/Ada-Transfer-Protocol/Server/main/tools/uninstall.sh | bash
```

---

## 🔐 Authentication & Security

AdaTP supports two authentication modes:

### 1. Internal Database (Default)
Users are managed via the internal SQLite database (`adatp.db`).

### 2. External API Integration (Webhook)
You can delegate authentication to your custom backend (e.g. PHP/Node.js/Python).

Add this to your `.env` file:
```env
AUTH_DRIVER=api
AUTH_API_URL=https://api.myapp.com/v1/verify_user
```

**Request (AdaTP -> Your API)**:
```json
POST /v1/verify_user
{
  "username": "alice",
  "password": "user_provided_password"
}
```

**Response (Your API -> AdaTP)**:
```json
// Success
{
  "authorized": true,
  "user_id": "uuid-5566",   // Used as PeerID
  "role": "admin"           // Optional
}

// Failure
{
  "authorized": false,
  "error": "Invalid password"
}
```

---

## ⚙️ Configuration (Environment Variables)

You can configure the server by setting environment variables or creating a `.env` file in the root directory.

| Variable | Default | Description |
| :--- | :--- | :--- |
| `HOST` | `0.0.0.0` | Bind address. Use `127.0.0.1` for local only. |
| `PORT` | `3000` | Listening port for WebSocket connections. |
| `DATABASE_URL` | `sqlite:adatp.db` | Path to the SQLite database file. |
| `RUST_LOG` | `info` | Log level: `error`, `warn`, `info`, `debug`, `trace`. |
| `AUTH_API_URL` | (none) | URL for external auth webhook. |

---

## 🛠 Management CLI (Ubuntu / Debian / RHEL)

After installation, use these global commands to manage the server:

### Service Control
```bash
# Check Server Status (Active/Inactive)
adatp-status

# View Live Logs (Real-time)
adatp-log

# Restart Server (Apply config changes)
adatp-restart

# Stop Server
adatp-stop
```

### Admin Console
Launch the interactive command-line interface to inspect the server:
```bash
# Connect to local server (Anonymous)
adatp

# Connect with Authentication
adatp -u admin -p mypassword

# Connect to remote server with credentials
adatp --address 20.0.0.31:3000 --username alice --password secret
```

---

## 📚 Client SDKs

Integrate AdaTP into your applications using our official SDKs.

| Language | Repository | Status |
| :--- | :--- | :--- |
| **JavaScript / Web** | [SDK-JS](https://github.com/Ada-Transfer-Protocol/SDK-JS) | ✅ Stable |
| **Node.js** | [SDK-NodeJS](https://github.com/Ada-Transfer-Protocol/SDK-NodeJS) | ✅ Stable |
| **Python** | [SDK-Python](https://github.com/Ada-Transfer-Protocol/SDK-Python) | ✅ Stable |
| **PHP** | [SDK-PHP](https://github.com/Ada-Transfer-Protocol/SDK-PHP) | ✅ Stable |
| **C / Embedded** | [SDK-C](https://github.com/Ada-Transfer-Protocol/SDK-C) | ✅ Stable |

### JavaScript SDK Features
The Web SDK supports `AdaTPPhone` (VoIP), `AdaTPChat` (Messaging), and `AdaTPConference` with a low-code config pattern.

---

## 📂 Project Structure

```
/adatp-server
├── /server           # Core Server Application
├── /core             # Shared Libraries (Used by Client & Server)
├── /tools            # DevOps & Utilities
│   ├── setup.sh      # Universal Installer Script
│   ├── uninstall.sh  # Uninstaller
│   ├── install_service.sh # Systemd Generator
│   └── /adatp-cli    # Rust-based Admin CLI tool
├── /docs             # Documentation
│   └── PROTOCOL_SPEC.md # Binary Protocol Specification (RFC-style)
```

## License
MIT License. Copyright © 2024 Ada Transfer Protocol Team.
