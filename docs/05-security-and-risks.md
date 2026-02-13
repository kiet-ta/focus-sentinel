# Security & Risk Analysis

## 1. Threat Model

### User vs. Admin
This tool is designed for **self-control**, not **adversarial control**.
*   **Assumption:** The user *wants* to be restricted but needs a guardrail.
*   **Reality:** A user with file system access can simply delete the `config.json` or kill the `focus_sentinel_host.exe` process.
*   **Mitigation:** The system raises the "friction" required to bypass it, but does not strictly prevent admin override.

### Malicious Websites
*   **Input Sanitization:** The Rust host receives arbitrary URL strings. It parses them using the robust `url` crate, which handles malicious encoding or weird paths safely.
*   **No Code Execution:** The host parses JSON and strings. It does not execute any remote code or shell commands.

## 2. Risk Analysis

| Risk Component | Scenario | Impact | Mitigation |
| :--- | :--- | :--- | :--- |
| **Native Host** | Process Crash | Extension stops blocking; User can browse freely. | Extension attempts auto-reconnect. Fail-open design prevents browser lockup. |
| **Config File** | Corruption | Host fails to parse JSON. | Host falls back to default "safe" config or just fails check (Fail-Open). |
| **Performance** | High Latency | Slow navigation if Rust host is slow. | Native Messaging over stdio is extremely fast (<10ms). Minimal overhead. |
| **Privacy** | URL Logging | Host logic could log history. | Current implementation stores **no logs**. Privacy-first design. |

## 3. Dependency auditing

### Rust Crates
*   `serde` / `serde_json`: Industry standard, well-audited.
*   `byteorder`: Simple utility, low risk.
*   `dirs`: OS path abstraction, low risk.

### Chrome Permissions
*   `<all_urls>` is a high-privilege permission.
*   **Justification:** Essential for the core functionality of a "Website Blocker".
*   **Review:** Verification processes (Chrome Web Store) typically scrutinize this heavily.

## 4. Failure Modes

### Fail-Open vs. Fail-Closed
FocusSentinel is designed to **Fail-Open**.
*   If the Host crashes, the extension disconnects.
*   Pending checks might time out or be ignored.
*   **Result:** The user can access the internet.
*   **Reasoning:** Breaking the user's browser (blocking everything) due to a bug is a worse user experience than temporarily allowing access.
