# 📂 CGIT-CLI // SURGICAL_EXTRACTION_TOOL
> **PROTOCOL:** NEURAL_LINK_ESTABLISHED
> **VERSION:** 1.0.4

**cgit-cli** is a terminal-based GitHub utility written in **Rust**. It provides a futuristic, minimalist interface for precision data extraction, allowing you to navigate and download specific repository nodes without full clones.

---

## ⚡ SYSTEM_FEATURES

* **SURGICAL_PRECISION**: Direct navigation of remote GitHub trees.
* **ASYNC_ENGINE**: Non-blocking UI powered by `tokio` and `ratatui`.
* **EDEX_INTERFACE**: High-contrast wireframe UI with live telemetry streams.
* **BULK_MARKING**: Multi-select specific targets or use `[A]` for mass extraction.
* **MINIMALIST**: Optimized for tiling window managers and keyboard-centric workflows.

---

## 🛠 TECH_STACK

| COMPONENT       | SPECIFICATION                          |
| :-------------- | :------------------------------------- |
| **Language** | Rust 1.75+                             |
| **UI Framework**| Ratatui (Low-level TUI)                |
| **Async** | Tokio                                  |
| **API** | GitHub REST API v3                     |
| **Environment** | Linux (Optimized for CachyOS/Arch)     |

---

## ⌨️ CONTROL_MAPPINGS

| KEY       | ACTION                                  |
| :-------- | :-------------------------------------- |
| `ENTER`   | **OPEN** / Enter Directory              |
| `SPACE`   | **MARK** / Select Target                |
| `A`       | **ALL** / Toggle Select All Items       |
| `D`       | **EXTRACT** / Begin Download Sequence   |
| `ESC`     | **BACK** / Return to Previous Node      |
| `CTRL+C`  | **TERMINATE** / Kill Process            |

---

## 🚀 DEPLOYMENT

### From Source
Ensure your Rust toolchain is active before building.

```bash
# Clone the repository
git clone https://github.com/Yukari-dev/cgit-cli

# Navigate to source
cd cgit-cli

# Build release binary
cargo build --release

# Execute
./target/release/cgit-cli

📡 TELEMETRY_PROTOCOL

The right-hand ANALYTICS panel displays a live HEX-stream synchronized with the system clock. This indicates a stable connection to the GitHub API. If the stream stalls, check your network interface.
📝 LICENSE

MIT // YUKARI_DEV_SYSTEMS

    "Everything is data. The rest is just noise."
