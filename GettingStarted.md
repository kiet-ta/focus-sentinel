# Getting Started with FocusSentinel

Welcome to FocusSentinel! This guide will help you set up the complete system, which consists of two components: the **Native Host (Rust Core)** and the **Browser Extension**.

---

## 🛠️ 1. Prerequisites

Before starting, ensure your system has the following tools installed:
- **Chromium-based Browser:** Google Chrome, Microsoft Edge, or Brave.
- **Rust Toolchain:** Used to compile the Native Host. Install it from [rustup.rs](https://rustup.rs/) (run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs` on Linux/Mac, or download the installer on Windows).

---

## 📥 2. Download the Source Code

Clone the repository using Git or download it as a ZIP file:

```bash
git clone https://github.com/kiet-ta/focus-sentinel.git
cd focus-sentinel
```

---

## ⚙️ 3. Compile the Native Host (Rust Backend)

The Native Host is the background application that runs on your operating system to manage and validate URLs.

1. Navigate into the `host` directory:
   ```bash
   cd host
   ```
2. Compile the project (A release build is required for optimal performance):
   ```bash
   cargo build --release
   ```
3. Once compiled, the executable will be generated at `target/release/focus_sentinel_host` (Linux/Mac) or `target/release/focus_sentinel_host.exe` (Windows). Do not move this file!

---

## 🧩 4. Install the Browser Extension

You need to load the extension into your browser.

1. Open your Chrome/Edge/Brave browser.
2. Navigate to the extensions page:
   - **Chrome / Brave:** `chrome://extensions/`
   - **Edge:** `edge://extensions/`
3. Enable **Developer mode** (usually a toggle in the top right corner).
4. Click **Load unpacked**.
5. Browse and select the `extension` folder located inside the `focus-sentinel` directory you just cloned.
6. Once loaded successfully, the FocusSentinel extension will appear. **Copy your Extension ID (e.g., `jbclhaqbgpcmilmjjpldpcknneegmfgk`).**

---

## 🔌 5. Connect Extension with Native Host

This is the most crucial step. You must declare the Extension ID in the native messaging manifest so the browser grants it permission to communicate with the Rust backend.

**Step 5.1: Update Extension ID**
1. Open the installation script for your operating system:
   - **Linux:** Open `install_linux.sh`
   - **Windows:** Open `install_win.bat`
2. Search for the string `"chrome-extension://jbclhaqbgpcmilmjjpldpcknneegmfgk/"`.
3. Replace the placeholder ID `jbclhaqbgpcmilmjjpldpcknneegmfgk` with your **actual Extension ID** copied in Step 4. Save the file.

**Step 5.2: Run the Registration Script**
- **Windows:** Double-click the `install_win.bat` file, or run it via CMD/PowerShell.
- **Linux:**
  ```bash
  # Return to the project root directory
  cd ..
  
  # Grant execution permissions and run
  chmod +x install_linux.sh
  ./install_linux.sh
  ```

---

## 🎉 6. How to Use

1. Click on the **FocusSentinel** icon in your browser toolbar to open the Popup.
2. Under **Allowed Domains**, add the websites you want to whitelist (e.g., `github.com`, `stackoverflow.com`). **Note: You cannot edit the whitelist while Focus Mode is ACTIVE.**
3. Toggle the **Focus Mode** switch to ON. The status will change to **ACTIVE**.
4. Try visiting a website not on your whitelist (e.g., `facebook.com`). You will be immediately redirected to the **Blocked Page** with a message to get back to work!

*If Focus Mode is active but you can still access all websites, the Extension might have failed to connect to the Native Host. Double-check Step 5 to ensure the Extension ID is correct.*
