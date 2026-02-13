use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug, Clone)] // Clone needed for default
struct Config {
    is_active: bool,
    whitelist: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_active: false,
            whitelist: vec![
                "google.com".to_string(),
                "github.com".to_string(),
                "localhost".to_string(),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum Request {
    UpdateConfig { payload: Config },
    CheckUrl { url: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Response {
    Acknowledge { status: String },
    CheckResult { action: String, #[serde(skip_serializing_if = "Option::is_none")] redirect: Option<String> },
    Error { error: String },
}

// --- Config Management ---

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("focussentinel"); // Lowercase for Linux consistency
    if cfg!(target_os = "windows") {
        // Window usually uses Capitalized AppData dirs, but 'focussentinel' is fine.
        // Let's stick to the spec: AppData\Roaming\FocusSentinel
         path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
         path.push("FocusSentinel");
    }
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

fn load_config() -> Config {
    let path = get_config_path();
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str(&data) {
            return config;
        }
    }
    let default = Config::default();
    save_config(&default); // Save default if missing/corrupt
    default
}

fn save_config(config: &Config) {
    let path = get_config_path();
    if let sticky = serde_json::to_string_pretty(config) {
         let _ = fs::write(path, sticky);
    }
}

// --- Native Messaging I/O ---

const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

fn read_input<R: Read>(stdin: &mut R) -> io::Result<Option<Request>> {
    // 1. Read 4-byte length (Little Endian)
    let len = match stdin.read_u32::<LittleEndian>() {
        Ok(len) => len as usize,
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None), // Chrome closed pipe
        Err(e) => return Err(e),
    };

    if len > MAX_MESSAGE_SIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Message too large"));
    }

    // 2. Read 'len' bytes
    let mut buffer = vec![0u8; len];
    stdin.read_exact(&mut buffer)?;

    // 3. Parse JSON
    let req: Request = serde_json::from_slice(&buffer)?;
    Ok(Some(req))
}

fn write_output<W: Write>(stdout: &mut W, resp: &Response) -> io::Result<()> {
    let msg = serde_json::to_string(resp)?;
    let msg_bytes = msg.as_bytes();
    let len = msg_bytes.len() as u32;

    // 1. Write 4-byte length (Little Endian)
    stdout.write_u32::<LittleEndian>(len)?;
    // 2. Write JSON bytes
    stdout.write_all(msg_bytes)?;
    stdout.flush()?;
    Ok(())
}

// --- Logic ---

fn check_url(url_str: &str, config: &Config) -> Response {
    if !config.is_active {
        return Response::CheckResult { action: "ALLOW".to_string(), redirect: None };
    }

    // specific hack for chrome-extension pages or internal chrome pages to avoid blocking the block page itself
    if url_str.starts_with("chrome-extension://") || url_str.starts_with("chrome://") {
           return Response::CheckResult { action: "ALLOW".to_string(), redirect: None };
    }

    if let Ok(url) = url::Url::parse(url_str) {
        if let Some(host_str) = url.host_str() {
            for domain in &config.whitelist {
                // Check if host ends with domain (loose check for subdomains)
                // e.g. "mail.google.com" ends with "google.com"
                if host_str == domain || host_str.ends_with(&format!(".{}", domain)) {
                     return Response::CheckResult { action: "ALLOW".to_string(), redirect: None };
                }
            }
        }
    }

    Response::CheckResult {
        action: "BLOCK".to_string(),
        redirect: Some("blocked.html".to_string()), // Extension will resolve full path
    }
}

// --- Main ---

fn main() {
    let mut config = load_config(); // Load once at startup
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        match read_input(&mut stdin) {
            Ok(Some(req)) => {
                let resp = match req {
                    Request::UpdateConfig { payload } => {
                        config = payload.clone(); // Update in-memory
                        save_config(&payload);    // Persist to disk
                        Response::Acknowledge { status: "ok".to_string() }
                    }
                    Request::CheckUrl { url } => {
                        check_url(&url, &config) // Use in-memory config
                    }
                };
                if let Err(_) = write_output(&mut stdout, &resp) {
                    break; // Exit if stdout breaks
                }
            }
            Ok(None) => break, // EOF
            Err(_) => break, // Error or EOF (including message too large)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_check_url_whitelist() {
        let config = Config {
            is_active: true,
            whitelist: vec!["google.com".to_string(), "example.com".to_string()],
        };

        // Allowed
        let resp = check_url("https://google.com", &config);
        assert!(matches!(resp, Response::CheckResult { action, .. } if action == "ALLOW"));

        let resp = check_url("https://mail.google.com", &config);
        assert!(matches!(resp, Response::CheckResult { action, .. } if action == "ALLOW"));

        // Blocked
        let resp = check_url("https://facebook.com", &config);
        assert!(matches!(resp, Response::CheckResult { action, .. } if action == "BLOCK"));
        
        let resp = check_url("https://notgoogle.com", &config);
        assert!(matches!(resp, Response::CheckResult { action, .. } if action == "BLOCK"));
    }

    #[test]
    fn test_check_url_inactive() {
        let config = Config {
            is_active: false,
            whitelist: vec!["google.com".to_string()],
        };
        let resp = check_url("https://facebook.com", &config);
        assert!(matches!(resp, Response::CheckResult { action, .. } if action == "ALLOW"));
    }

    #[test]
    fn test_read_input_valid() {
        let req = Request::CheckUrl { url: "https://example.com".to_string() };
        let req_json = serde_json::to_string(&req).unwrap();
        let len = req_json.len() as u32;
        
        let mut input = Vec::new();
        input.write_u32::<LittleEndian>(len).unwrap();
        input.write_all(req_json.as_bytes()).unwrap();

        let mut cursor = Cursor::new(input);
        let result = read_input(&mut cursor).unwrap();
        assert!(result.is_some());
        match result.unwrap() {
            Request::CheckUrl { url } => assert_eq!(url, "https://example.com"),
            _ => panic!("Wrong request type"),
        }
    }

    #[test]
    fn test_read_input_too_large() {
        let len = (MAX_MESSAGE_SIZE + 1) as u32;
        let mut input = Vec::new();
        input.write_u32::<LittleEndian>(len).unwrap();
        
        let mut cursor = Cursor::new(input);
        let result = read_input(&mut cursor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }
}
