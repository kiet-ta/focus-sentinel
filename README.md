# FocusSentinel

![CI Status](https://github.com/kiet-ta/focus-sentinel/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

**FocusSentinel** is a robust, cross-platform productivity tool designed to enforce strict study and work habits by blocking distracting websites. It pairs a Manifest V3 browser extension with a Rust-based Native Messaging Host to ensure URL blocking is reliable, persistent, and not easily bypassed by clearing browser data.

## ✨ Features

- **Strict Whitelisting**: "Deny-by-default" approach. When Focus Mode is active, only explicitly allowed domains can be accessed.
- **Fail-Open Architecture**: Minimizes disruption. If the host temporarily disconnects or fails, browsing remains available rather than permanently locking you out.
- **Cross-Platform**: Natively compiled for both Linux and Windows environments.
- **Persistent Configuration**: State is saved on the OS layer (via JSON), surviving browser restarts and cache clears.
- **Minimal Overhead**: Communication over standard I/O pipes (Native Messaging API) ensures sub-10ms response times without slowing down browsing.

## 🏗️ Architecture

The system operates via two components:
1. **Frontend (Chrome Extension)**: Injects into your browser, intercepts navigation attempts using `webNavigation`, manages the Popup UI, and renders the block page.
2. **Backend (Native Host)**: Written in Rust, it persists configuration to the OS file system, processes URL evaluation logic asynchronously, and responds to the extension via Stdio streams.

See the [docs/](./docs) directory for deep-dive architectural design documents.

## 🚀 Getting Started

For a comprehensive, step-by-step installation guide, please refer to:
👉 **[GettingStarted.md](./GettingStarted.md)**

### Brief Prerequisites
- Google Chrome, Edge, Brave, or any Chromium-based browser
- [Rust & Cargo](https://rustup.rs/) (for compiling the backend)

## 💻 Development & Building

To manually compile and test the Rust backend:
```bash
cd host
cargo build --release
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:
1. Fork the project.
2. Create your feature branch (`git checkout -b feat/AmazingFeature`).
3. Commit your changes adhering to conventional commits (`git commit -m 'feat: add some AmazingFeature'`).
4. Push to the branch (`git push origin feat/AmazingFeature`).
5. Open a Pull Request.

Please ensure all tests pass (`cargo test`) before submitting.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
