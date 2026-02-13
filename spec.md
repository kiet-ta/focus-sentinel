# Project Specification: FocusSentinel
**Version:** 1.0.0
**Type:** Browser Extension & Native Messaging Host
**Goal:** Enforce strict study focus by blocking all non-whitelisted websites during active sessions.

## 1. Architecture Overview
The system consists of two main components communicating via Standard I/O (stdio) using the Chrome Native Messaging protocol.

### A. Browser Extension (Frontend)
* **Tech Stack:** HTML, CSS, Vanilla JavaScript (Manifest V3).
* **Role:**
    * Monitor tab navigation (`webNavigation` API).
    * Provide a Popup UI for user configuration.
    * Send URL checks to the Native Host.
    * Execute redirects based on Native Host response.

### B. Native Host (Backend)
* **Tech Stack:** Rust (release build).
* **Role:**
    * Store configuration (Whitelist domains, Focus Mode status) locally.
    * Process URL validation logic.
    * Handle cross-platform file paths (Windows `%APPDATA%`, Linux `$XDG_CONFIG_HOME`).

## 2. Functional Requirements

### 2.1. Focus Mode Logic (Strict Whitelist)
* **Default State:** Focus Mode is OFF. All browsing is allowed.
* **Active State:** Focus Mode is ON.
    * User can only access domains in the `whitelist`.
    * Any other URL triggers a `BLOCK` response.
    * Blocked requests redirect to a local `blocked.html` page (packaged in extension).

### 2.2. Configuration (Popup UI)
* **Toggle Switch:** Start/Stop Focus Mode.
* **Domain Management:**
    * Input field to add domain (e.g., `stackoverflow.com`).
    * List view of whitelisted domains with a "Remove" button.
    * *Constraint:* Cannot edit whitelist while Focus Mode is ACTIVE.

### 2.3. Data Persistence
* Config is saved in a JSON file: `config.json`.
* **Location:**
    * **Windows:** `C:\Users\<User>\AppData\Roaming\FocusSentinel\config.json`
    * **Linux:** `~/.config/focussentinel/config.json`
* **Schema:**
    ```json
    {
      "is_active": false,
      "whitelist": ["google.com", "github.com", "localhost"]
    }
    ```

## 3. Communication Protocol (Native Messaging)
Messages are JSON objects sent over Stdin/Stdout.
**Important:** Each message must be preceded by the 4-byte length header (little-endian) as per Chrome Native Messaging spec.

### Request (Extension -> Rust)
Type 1: Update Config
```json
{ "type": "UPDATE_CONFIG", "payload": { "is_active": true, "whitelist": [...] } }

```

Type 2: Check URL

```json
{ "type": "CHECK_URL", "url": "[https://facebook.com/feed](https://facebook.com/feed)" }

```

### Response (Rust -> Extension)

Type 1: Acknowledge

```json
{ "status": "ok" }

```

Type 2: Check Result

```json
{ "action": "ALLOW" }
// OR
{ "action": "BLOCK", "redirect": "extension://.../blocked.html" }

```

## 4. Implementation Details

### 4.1. Rust Logic (Crate: `serde`, `serde_json`)

* **Loop:** The main function runs an infinite loop reading stdin.
* **Parsing:** Deserialize JSON.
* **Matching:** Check if `url.host` contains any whitelisted string.
* **Error Handling:** If reading fails (EOF), terminate gracefully.

### 4.2. Installation Scripts

* **`install_win.bat`**: Sets registry key `HKCU\Software\Google\Chrome\NativeMessagingHosts\com.focussentinel`.
* **`install_linux.sh`**: Copies manifest to `~/.config/google-chrome/NativeMessagingHosts/`.

## 5. Security & Permissions

* **Extension Permissions:** `nativeMessaging`, `tabs`, `webNavigation`.
* **Host Permissions:** `<all_urls>` (required to intercept navigation).