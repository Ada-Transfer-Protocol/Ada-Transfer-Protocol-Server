# AdaTP Server & CLI Guide

This guide provides instructions for setting up, configuring, and running the AdaTP Server (`server`) and the Command Line Interface (`tools/adatp-cli`).

## 📂 Project Structure Note

The project requires a specific directory structure. The **SDKs** and **Docs** are located in the project root, while the Rust core and server components are within the `adatp` workspace.

```text
AdaTP/              # Project Root
├── sdks/           # Client Libraries (Node.js, Python, etc.)
├── docs/           # Protocol Documentation
└── adatp/          # Rust Workspace
    ├── core/       # Protocol Core
    ├── server/     # Main Server Application
    └── tools/      # Utilities
        └── adatp-cli  # Admin & Test CLI
```

---

## 🚀 1. AdaTP Server

The server handles all client connections, packet routing, and room management. It is built with Rust and Tokio for high-performance asynchronous I/O.

### Requirements
*   Rust (latest stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Setup & Configuration

1.  **Navigate to the Server Directory:**
    ```bash
    cd adatp/server
    ```

2.  **Environment Variables:**
    Create a `.env` file from the example.
    ```bash
    cp .env.example .env
    ```

3.  **User Authentication (`users.json`):**
    The server uses a JSON file for basic authentication. Ensure this file exists in the `server` directory.
    ```json
    {
      "users": {
        "admin": { "password": "secure_password", "role": "admin" },
        "client1": { "password": "password123", "role": "user" }
      }
    }
    ```

### Running the Server

**Development Mode:**
```bash
RUST_LOG=info cargo run
```

**Production Build:**
```bash
cargo build --release
./target/release/adatp-server
```

**Ports:**
*   `TCP: 8444` - Main AdaTP protocol port.
*   `HTTP: 3000` - Metrics and API port.

---

---
## 🛠️ 2. Admin CLI & Test Tool

The project contains two CLI tools:
1.  **Admin CLI:** For managing the server (API Keys, Stats) - located in `adatp-server`.
2.  **Test Tool:** For testing the protocol connection - located in `tools/adatp-cli`.

### A. Admin CLI (Management)

Use this to manage API keys and view server statistics.

```bash
# List API Keys
cargo run -p adatp-server --bin adatp-cli -- auth list

# Create a new API key
cargo run -p adatp-server --bin adatp-cli -- auth create --description "New App"
```

### B. Test Tool (Connection & Protocol)

Use this to test valid handshake and encryption flow.

**From `adatp/` workspace root:**
```bash
# Connect to the local server on port 8444
cargo run -p adatp-cli -- -a 127.0.0.1:8444
```

### Expected Output (Test Tool)
1.  **Handshake Init:** Sends public key (X25519) to server.
2.  **Handshake Response:** Receives server public key.
3.  **Encrypted Session:** Derives session keys and establishes AES-256-GCM channel.
4.  **Message Test:** Sends an encrypted "Hello" message.
5.  **Echo:** Receives the decrypted echo from the server.

---



---

## 🔗 Related Resources

*   **Protocol Documentation:** See `../docs/` (relative to `adatp` folder).
*   **Client SDKs:** See `../sdks/` for Node.js, Python, PHP, and C implementations.
