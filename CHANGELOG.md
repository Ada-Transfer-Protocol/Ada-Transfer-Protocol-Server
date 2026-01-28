# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2024-01-28
### Added
- **Universal Linux Installer:** `setup.sh` now supports automated installation on Ubuntu, Debian, RHEL, CentOS, Fedora, Arch, and Alpine.
- **SSH Welcome Screen (MOTD):** Professional dashboard displaying system status, IP, ports, and developer credits upon login.
- **Authentication:** Added `AUTH_API_URL` support for external Webhook-based authentication alongside internal SQLite auth.
- **Admin CLI:** Enhanced `adatp` tool with `--username` and `--password` flags for secure authenticated connections.
- **Uninstaller:** Added `tools/uninstall.sh` for complete system cleanup.
- **Documentation:** Comprehensive `README.md` and `PROTOCOL_SPEC.md` updates.

### Changed
- **Service Name:** Systemd service renamed to `adatp-server` (aliased commands updated).
- **Default Config:** `Cargo.lock` is now tracked for reproducible builds.
- **Dependency Management:** Support for offline/vendored builds (with automatic fallback to online).
- **SDK Links:** Added references to Official SDKs for JS, Node.js, Python, PHP, and C.

### Fixed
- **Installation:** Resolved persistent caching issues with installation scripts via version query params.
- **MOTD:** Fixed bash syntax errors in welcome screen generation (heredoc/variable expansion).
- **CLI:** Fixed default connection port from 8443 to 3000 to match server default.
