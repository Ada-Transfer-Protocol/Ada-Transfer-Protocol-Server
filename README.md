# AdaTP - Adaptive Transport Protocol

AdaTP is a secure, real-time, low-latency binary communication protocol designed for modern applications requiring high-performance data exchange. It features built-in end-to-end encryption, multi-room support, and efficient file transfer capabilities.

## 🚀 Key Features

*   **Security First:** X25519 Elliptic Curve Diffie-Hellman Key Exchange + AES-256-GCM Encryption for all payloads.
*   **Packet-Based:** Binary protocol with minimal overhead (45-byte header).
*   **Real-Time:** Persistent TCP connections for instant message delivery and broadcasting.
*   **Multi-Room Chat:** Dynamic room management (Public/Private rooms).
*   **File Transfer:** Chunked, encrypted, and reliable file sharing.
*   **Cross-Platform:** Native SDKs for Node.js, Python, PHP, and C.

---

## 📂 Project Structure

```
adatp/
├── core/           # Core protocol implementation (Rust library)
│   ├── src/        # Shared logic, Packet definitions, Crypto wrappers
├── server/         # High-performance Rust Server
│   ├── src/        # Server logic (Room management, Client handling)
│   └── users.json  # User authentication database
├── sdks/           # Client Libraries
│   ├── nodejs/     # TypeScript/Node.js SDK
│   ├── python/     # Python 3 SDK
│   ├── php/        # PHP 8.x SDK
│   └── c/          # C11 SDK (compatible with C++)
└── docs/           # Protocol Specification (RFC-style)
```

---

## 🖥️ Server

The AdaTP server is built in Rust using `tokio` for asynchronous I/O. It handles client connections, authentication, room management, and packet routing.

### Prerequisites
*   Rust (latest stable) -> `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Running the Server

```bash
cd adatp/server

# Run in development mode
RUST_LOG=info cargo run

# Build for production
cargo build --release
./target/release/adatp-server
```

### Configuration (`users.json`)
The server uses a simple JSON file for authentication. Add users here to grant access.

```json
{
  "users": {
    "admin": { "password": "admin_password", "role": "admin" },
    "filebot": { "password": "secret_password", "role": "bot" }
  }
}
```

**Note:** The server must be restarted after modifying `users.json`.

---

## 📊 Monitoring & CLI

AdaTP Server includes a built-in REST API for real-time monitoring and a powerful Admin CLI for management.

### Monitoring API
*   **Port:** 3000 (Default)
*   **Security:** Protected by API Key header (`x-api-key`).
*   **Endpoints:**
    *   `GET /api/metrics` -> Returns active connections, traffic stats, and uptime.

### Admin CLI (`adatp-cli`)
Use the CLI to manage API keys and inspect the server.

```bash
# 1. List API Keys (Direct DB Access)
cargo run --bin adatp-cli -- auth list

# 2. Create New API Key
cargo run --bin adatp-cli -- auth create --description "Dashboard App"

# 3. View Real-Time Stats
cargo run --bin adatp-cli -- stats --key <YOUR_API_KEY>
```

---

## 📦 SDKs & Client Usage

All SDKs implement the full AdaTP protocol specification, including Handshake (X25519), Encryption (AES-GCM), and File Transfer.

| Feature | Node.js | Python | PHP | C |
| :--- | :---: | :---: | :---: | :---: |
| **Secure Connection** | ✅ | ✅ | ✅ | ✅ |
| **Authentication** | ✅ | ✅ | ✅ | ✅ |
| **Chat / Rooms** | ✅ | ✅ | ✅ | ✅ |
| **File Transfer** | ✅ | ✅ | ✅ | ✅ |

### 1. Node.js SDK
**Directory:** `sdks/nodejs`

```bash
# Setup
npm install
npm run build

# Run Chat Example
npx ts-node example.ts

# Run File Transfer Example
npx ts-node filetransfer_example.ts
```

### 2. Python SDK
**Directory:** `sdks/python`
**Requirements:** `cryptography` library.

```bash
# Setup
pip install cryptography

# Run Chat Example
PYTHONPATH=src python3 example_chat.py

# Run File Transfer Example (Fixes applied)
PYTHONPATH=src python3 -u filetransfer_example.py
```

### 3. PHP SDK
**Directory:** `sdks/php`
**Requirements:** `ext-openssl`, `ext-sockets`.

```bash
# Run Chat Example
php chat_example.php

# Run File Transfer Example
php filetransfer_example.php
```

### 4. C SDK
**Directory:** `sdks/c`
**Requirements:** OpenSSL (`libssl`, `libcrypto`).

```bash
# Compile File Transfer Example
gcc -I include -I/opt/homebrew/include -I/usr/local/include \
    src/client.c src/packet.c src/crypto.c filetransfer_example.c \
    -L/opt/homebrew/lib -L/usr/local/lib -lssl -lcrypto -o c_file_transfer

# Run
./c_file_transfer
```

---

## ⚙️ Core Working Principle

1.  **Handshake:** 
    *   Client generates ephemeral X25519 keys.
    *   Sends Public Key to Server (`HANDSHAKE_INIT`).
    *   Server responds with its Public Key (`HANDSHAKE_RESPONSE`).
    *   Both parties compute shared secret -> Derive Session Keys (HKDF).
    *   Client sends Encrypted Confirmation (`HANDSHAKE_COMPLETE`).

2.  **Session:**
    *   All subsequent packets are Encrypted (AES-256-GCM) and Authenticated (Auth Tag).
    *   **Header:** 45 Bytes (Magic, Ver, Flags, Length, Seq, Type, Timestamp, SessionID).
    *   **Payload:** Variable length encrypted data.

3.  **File Transfer Flow:**
    *   `FILE_INIT`: Sender broadcasts metadata (Filename, Size, UUID).
    *   `FILE_CHUNK`: Sender streams data in chunks (e.g., 16KB).
    *   `FILE_COMPLETE`: Signals end of transfer.

For detailed protocol definitions, see [docs/PROTOCOL_SPEC.md](docs/PROTOCOL_SPEC.md).

---

## 📜 License
MIT License.
