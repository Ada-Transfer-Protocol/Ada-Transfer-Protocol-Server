# Ada Transfer Protocol (Server)

**High-Performance Real-Time Communication Server** powered by Rust.

This repository contains the reference implementation of the **AdaTP Server**, designed to handle massive concurrency for Voice, Video, and Signaling with minimal latency.

## 🚀 Features

*   **Ultra-Low Latency**: Built on `tokio` asynchronous runtime.
*   **Binary Protocol**: Custom framing for minimal overhead (Header + Payload).
*   **Room Management**: Dynamic room creation and isolation.
*   **Global & Private Signaling**: Direct routing of signaling messages (`INVITE`, `ACCEPT`).
*   **Persistence**: SQLite integration for user auth and logs.
*   **Cross-Platform**: Runs on Linux, macOS, and Windows.

---

## 🛠 Installation & Usage

### Prerequisites
*   Rust (Cargo) installed.

### Run Server

```bash
# Clone Repository
git clone https://github.com/Ada-Transfer-Protocol/Server.git
cd Server

# Run Development Server
cargo run --bin adatp-server
```

Server listens on `0.0.0.0:3000` by default.

### Build for Production
```bash
cargo build --release --bin adatp-server
./target/release/adatp-server
```

---

## 📚 Protocol Specification

For full binary details, see [PROTOCOL_SPEC.md](docs/PROTOCOL_SPEC.md).

## 📦 Client SDKs

*   **JavaScript / Web**: [Ada-Transfer-Protocol/SDK-JS](https://github.com/Ada-Transfer-Protocol/SDK-JS)

---

## 📂 Project Structure

*   **/server**: The Rust server implementation (`main.rs`, `api.rs`).
*   **/core**: Shared libraries and models.
*   **/tools**: CLI utilities and test scripts.
*   **/docs**: Protocol documentation.

## License
MIT
