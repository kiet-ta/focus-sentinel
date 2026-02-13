# Data Flow & Communication Protocol

## 1. Protocol Overview

Communication between the Chrome Extension and the Rust Native Host occurs over **Standard I/O (stdio)** using the **Chrome Native Messaging** protocol.

### Message Format
Each message is a JSON object preceded by a 4-byte length header in **native byte order** (Little Endian on both Windows/Linux x86_64).

1.  **Length Header (4 bytes):** Unsigned 32-bit integer specifying the JSON byte length.
2.  **Payload (N bytes):** UTF-8 encoded JSON string.

```mermaid
sequenceDiagram
    participant Ext as Chrome Extension
    participant Host as Native Host (Rust)

    Note over Ext, Host: Handshake (Stdio Pipe Created)
    
    Ext->>Host: [Length: 4 bytes] + [JSON Payload]
    Note right of Host: Read 4 bytes (u32)<br/>Allocate buffer<br/>Read N bytes
    Host->>Host: Deserialize JSON
    Host->>Host: Process Logic
    Host-->>Ext: [Length: 4 bytes] + [JSON Response]
```

## 2. JSON Schema

### Requests (Extension -> Host)

The host accepts a discriminated union of request types.

#### Type 1: `UPDATE_CONFIG`
Updates the persisted configuration on the host.

```json
{
  "type": "UPDATE_CONFIG",
  "payload": {
    "is_active": true,
    "whitelist": ["google.com", "github.com"]
  }
}
```

#### Type 2: `CHECK_URL`
Asks the host to validate a URL against the current whitelist.

```json
{
  "type": "CHECK_URL",
  "url": "https://www.facebook.com/feed"
}
```

### Responses (Host -> Extension)

#### Type 1: Acknowledge
Sent after a successful config update.

```json
{
  "status": "ok"
}
```

#### Type 2: Check Result
Sent after evaluating a URL.

**Allowed:**
```json
{
  "action": "ALLOW"
}
```

**Blocked:**
```json
{
  "action": "BLOCK",
  "redirect": "blocked.html"
}
```

## 3. Asynchronous Data Flow

Since the Native Messaging API key components interaction is asynchronous, the extension maintains a state of pending validations (conceptually, though currently implemented as fire-and-forget with ordered responses).

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant BG as Background.js
    participant Rust as Native Host

    User->>Browser: Navigates to example.com
    Browser->>BG: onBeforeNavigate event
    BG->>Rust: sendNativeMessage({type: "CHECK_URL", url: "..."})
    
    par Browser Navigation
        Browser->>Browser: Continues loading...
    and Host Verification
        Rust->>Rust: Check whitelist
        Rust-->>BG: { action: "BLOCK", redirect: "..." }
    end
    
    alt Action is BLOCK
        BG->>Browser: chrome.tabs.update(tabId, {url: "blocked.html"})
        User->>Browser: Sees Blocked Page
    end
```
