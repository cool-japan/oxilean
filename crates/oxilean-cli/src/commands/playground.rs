//! `oxilean playground` — serves the WASM playground static assets locally.
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

/// Configuration for the playground subcommand.
pub struct PlaygroundConfig {
    pub port: u16,
    pub dir: PathBuf,
    pub no_open: bool,
}

impl PlaygroundConfig {
    /// Parse configuration from CLI arguments (the slice after `"playground"`).
    pub fn from_args(args: &[String]) -> Self {
        let mut port = 8080u16;
        let mut dir: Option<PathBuf> = None;
        let mut no_open = false;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--port" if i + 1 < args.len() => {
                    port = args[i + 1].parse().unwrap_or(8080);
                    i += 2;
                }
                "--dir" if i + 1 < args.len() => {
                    dir = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                }
                "--no-open" => {
                    no_open = true;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        // Default: look for playground/dist relative to cwd or common repo paths.
        let dir = dir.unwrap_or_else(|| {
            let candidates = [
                PathBuf::from("crates/oxilean-wasm/playground/dist"),
                PathBuf::from("playground/dist"),
            ];
            for c in &candidates {
                if c.exists() {
                    return c.clone();
                }
            }
            PathBuf::from("crates/oxilean-wasm/playground/dist")
        });
        PlaygroundConfig { port, dir, no_open }
    }
}

/// Run the playground static-file server.
pub fn run_playground(config: &PlaygroundConfig) -> Result<(), Box<dyn std::error::Error>> {
    if !config.dir.exists() {
        eprintln!("[oxilean playground] The playground dist directory was not found:");
        eprintln!("  {:?}", config.dir);
        eprintln!();
        eprintln!("To build it, run:");
        eprintln!("  cd crates/oxilean-wasm/playground && bash build.sh");
        return Ok(());
    }
    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(&addr)?;
    let url = format!("http://{}", addr);
    println!("[oxilean playground] Serving {:?} at {}", config.dir, url);
    if !config.no_open {
        open_browser(&url);
    }
    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_request(s, &config.dir),
            Err(e) => eprintln!("[oxilean playground] Connection error: {}", e),
        }
    }
    Ok(())
}

/// Attempt to open the URL in the default browser (best-effort, non-blocking).
fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", url])
        .spawn();
    // Silence "unused variable" on platforms where none of the above cfg match.
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let _ = url;
}

/// Handle a single incoming TCP connection: parse the request, serve the file.
fn handle_request(mut stream: TcpStream, serve_dir: &Path) {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(l)) => l,
        _ => return,
    };

    let (method, path) = match parse_request_line(&request_line) {
        Some(x) => x,
        None => {
            let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n");
            return;
        }
    };

    if method != "GET" && method != "HEAD" {
        let _ = stream.write_all(b"HTTP/1.1 405 Method Not Allowed\r\n\r\n");
        return;
    }

    // Strip query string.
    let path = path.split('?').next().unwrap_or("/");
    // Treat bare "/" as index.html.
    let path = if path == "/" { "/index.html" } else { path };

    // Resolve path and guard against traversal.
    let file_path = match resolve_path(serve_dir, path) {
        Some(p) => p,
        None => {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n");
            return;
        }
    };

    match fs::read(&file_path) {
        Ok(body) => {
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let ct = content_type(ext);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                ct,
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            if method == "GET" {
                let _ = stream.write_all(&body);
            }
        }
        Err(_) => {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n");
        }
    }
}

/// Parse the HTTP request line into `(method, path)`.
pub fn parse_request_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.splitn(3, ' ');
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();
    Some((method, path))
}

/// Resolve a URL path to an on-disk path, guarding against path traversal.
///
/// Returns `None` if the resolved path is outside `base` or does not exist as a file.
pub fn resolve_path(base: &Path, url_path: &str) -> Option<PathBuf> {
    let decoded = percent_decode(url_path);
    let relative = decoded.trim_start_matches('/');
    let candidate = base.join(relative);
    let canonical_base = base.canonicalize().ok()?;
    let canonical_candidate = candidate.canonicalize().ok()?;
    if !canonical_candidate.starts_with(&canonical_base) {
        return None; // Traversal attempt — reject.
    }
    if canonical_candidate.is_file() {
        Some(canonical_candidate)
    } else {
        None
    }
}

/// Minimal percent-decoding (handles `%XX` hex sequences, ASCII only).
fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().and_then(|c| c.to_digit(16));
            let h2 = chars.next().and_then(|c| c.to_digit(16));
            if let (Some(a), Some(b)) = (h1, h2) {
                result.push((((a << 4) | b) as u8) as char);
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Return the MIME `Content-Type` for a given file extension.
pub fn content_type(ext: &str) -> &'static str {
    match ext {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript",
        "css" => "text/css",
        "wasm" => "application/wasm",
        "json" => "application/json",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_content_type_html() {
        assert_eq!(content_type("html"), "text/html; charset=utf-8");
    }

    #[test]
    fn test_content_type_htm() {
        assert_eq!(content_type("htm"), "text/html; charset=utf-8");
    }

    #[test]
    fn test_content_type_wasm() {
        assert_eq!(content_type("wasm"), "application/wasm");
    }

    #[test]
    fn test_content_type_js() {
        assert_eq!(content_type("js"), "text/javascript");
    }

    #[test]
    fn test_content_type_mjs() {
        assert_eq!(content_type("mjs"), "text/javascript");
    }

    #[test]
    fn test_content_type_css() {
        assert_eq!(content_type("css"), "text/css");
    }

    #[test]
    fn test_content_type_json() {
        assert_eq!(content_type("json"), "application/json");
    }

    #[test]
    fn test_content_type_png() {
        assert_eq!(content_type("png"), "image/png");
    }

    #[test]
    fn test_content_type_svg() {
        assert_eq!(content_type("svg"), "image/svg+xml");
    }

    #[test]
    fn test_content_type_ico() {
        assert_eq!(content_type("ico"), "image/x-icon");
    }

    #[test]
    fn test_content_type_unknown() {
        assert_eq!(content_type("xyz"), "application/octet-stream");
    }

    #[test]
    fn test_content_type_empty() {
        assert_eq!(content_type(""), "application/octet-stream");
    }

    #[test]
    fn test_parse_request_line_ok() {
        let (m, p) = parse_request_line("GET /index.html HTTP/1.1").unwrap();
        assert_eq!(m, "GET");
        assert_eq!(p, "/index.html");
    }

    #[test]
    fn test_parse_request_line_root() {
        let (m, p) = parse_request_line("GET / HTTP/1.1").unwrap();
        assert_eq!(m, "GET");
        assert_eq!(p, "/");
    }

    #[test]
    fn test_parse_request_line_head() {
        let (m, p) = parse_request_line("HEAD /app.wasm HTTP/1.1").unwrap();
        assert_eq!(m, "HEAD");
        assert_eq!(p, "/app.wasm");
    }

    #[test]
    fn test_parse_request_line_empty() {
        assert!(parse_request_line("").is_none());
    }

    #[test]
    fn test_parse_request_line_missing_path() {
        assert!(parse_request_line("GET").is_none());
    }

    #[test]
    fn test_path_traversal_rejected() {
        let tmp = env::temp_dir().join("playground_test_traversal");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("index.html"), b"hello").unwrap();
        // Traversal attempt — should be rejected.
        let r = resolve_path(&tmp, "/../../../etc/passwd");
        assert!(r.is_none(), "Traversal should be rejected");
    }

    #[test]
    fn test_resolve_valid_file() {
        let tmp = env::temp_dir().join("playground_test_resolve");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("index.html"), b"hello").unwrap();
        let r = resolve_path(&tmp, "/index.html");
        assert!(r.is_some(), "Should resolve existing file");
    }

    #[test]
    fn test_resolve_missing_file() {
        let tmp = env::temp_dir().join("playground_test_missing");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let r = resolve_path(&tmp, "/nonexistent.html");
        assert!(r.is_none(), "Missing file should return None");
    }

    #[test]
    fn test_playground_config_defaults() {
        let cfg = PlaygroundConfig::from_args(&[]);
        assert_eq!(cfg.port, 8080);
        assert!(!cfg.no_open);
    }

    #[test]
    fn test_playground_config_port() {
        let args = vec!["--port".to_string(), "9090".to_string()];
        let cfg = PlaygroundConfig::from_args(&args);
        assert_eq!(cfg.port, 9090);
    }

    #[test]
    fn test_playground_config_no_open() {
        let args = vec!["--no-open".to_string()];
        let cfg = PlaygroundConfig::from_args(&args);
        assert!(cfg.no_open);
    }

    #[test]
    fn test_playground_config_dir() {
        let args = vec!["--dir".to_string(), "/tmp/myplayground".to_string()];
        let cfg = PlaygroundConfig::from_args(&args);
        assert_eq!(cfg.dir, std::path::PathBuf::from("/tmp/myplayground"));
    }

    #[test]
    fn test_playground_config_invalid_port_falls_back() {
        let args = vec!["--port".to_string(), "not-a-number".to_string()];
        let cfg = PlaygroundConfig::from_args(&args);
        assert_eq!(cfg.port, 8080, "Invalid port should fall back to 8080");
    }
}
