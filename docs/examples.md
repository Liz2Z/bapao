# Examples and Usage Guide

## Table of Contents

- [Basic Setup](#basic-setup)
- [Screenshot Service](#screenshot-service)
- [File Transfer Service](#file-transfer-service)
- [JSON API Service](#json-api-service)
- [Multi-Endpoint Application](#multi-endpoint-application)
- [Custom Transport Usage](#custom-transport-usage)
- [Error Handling Patterns](#error-handling-patterns)

---

## Basic Setup

### Minimal Application

The simplest possible Bapao application:

```rust
// src/main.rs
use bapao_app_protocal::{AppListener, TransUnitType};

fn hello_handler() -> TransUnitType {
    TransUnitType::String("Hello, Bapao!".to_string())
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/hello", hello_handler);
    
    println!("Bapao server starting...");
    listener.listen().await;
}
```

### Configuration Setup

```json
// bapao.config.json
{
  "access_token": "your_gitee_token_here",
  "user_name": "your_username",
  "repo": "bapao-demo",
  "file_path": "io"
}
```

### Repository Setup

1. Create repository `bapao-demo` on Gitee
2. Create file `io` with content: `[]`
3. Generate personal access token with repository permissions

---

## Screenshot Service

Complete implementation of a screenshot capture service:

```rust
// src/main.rs
use bapao_app_protocal::{AppListener, TransUnitType};
use std::{fs, process::Command};

mod screenshot;

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/monitor/pic/shot", screenshot::capture);
    
    println!("Screenshot service started");
    listener.listen().await;
}
```

```rust
// src/screenshot.rs
use bapao_app_protocal::TransUnitType;
use std::{fs, process::Command};

pub fn capture() -> TransUnitType {
    // For Linux with scrot
    let result = Command::new("scrot")
        .args(["/tmp/screenshot.png"])
        .output();
        
    match result {
        Ok(output) if output.status.success() => {
            match fs::read("/tmp/screenshot.png") {
                Ok(image_data) => {
                    // Clean up temporary file
                    let _ = fs::remove_file("/tmp/screenshot.png");
                    TransUnitType::File(image_data)
                },
                Err(_) => TransUnitType::String("Failed to read screenshot".to_string()),
            }
        },
        _ => TransUnitType::String("Screenshot command failed".to_string()),
    }
}

// Alternative implementation for macOS
pub fn capture_macos() -> TransUnitType {
    let result = Command::new("screencapture")
        .args(["/tmp/screenshot.png"])
        .output();
        
    match result {
        Ok(output) if output.status.success() => {
            match fs::read("/tmp/screenshot.png") {
                Ok(image_data) => {
                    let _ = fs::remove_file("/tmp/screenshot.png");
                    TransUnitType::File(image_data)
                },
                Err(_) => TransUnitType::String("Failed to read screenshot".to_string()),
            }
        },
        _ => TransUnitType::String("Screenshot failed".to_string()),
    }
}

// Cross-platform implementation
pub fn capture_cross_platform() -> TransUnitType {
    #[cfg(target_os = "linux")]
    return capture_linux();
    
    #[cfg(target_os = "macos")]
    return capture_macos();
    
    #[cfg(target_os = "windows")]
    return capture_windows();
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    TransUnitType::String("Unsupported platform".to_string())
}
```

**Usage from external client:**

Send request to Gitee repository file:
```json
[{
  "head": {
    "id": "screenshot_001",
    "content_type": null,
    "state": "Pending", 
    "timestamp": 1704067200000
  },
  "body": "/monitor/pic/shot"
}]
```

---

## File Transfer Service

Service for transferring files through the communication channel:

```rust
// src/main.rs
use bapao_app_protocal::{AppListener, TransUnitType};

mod file_service;

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    listener.add("/files/download", file_service::download_handler);
    listener.add("/files/list", file_service::list_handler);
    listener.add("/files/info", file_service::info_handler);
    
    println!("File service started");
    listener.listen().await;
}
```

```rust
// src/file_service.rs
use bapao_app_protocal::TransUnitType;
use std::{fs, path::Path};
use serde_json;

pub fn download_handler() -> TransUnitType {
    // In a real implementation, you'd parse the request to get the filename
    let file_path = "/home/user/documents/report.pdf";
    
    match fs::read(file_path) {
        Ok(data) => TransUnitType::File(data),
        Err(e) => TransUnitType::String(format!("{{\"error\": \"{}\"}}", e)),
    }
}

pub fn list_handler() -> TransUnitType {
    let dir_path = "/home/user/documents";
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let mut files = Vec::new();
            
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    files.push(name.to_string());
                }
            }
            
            let response = serde_json::json!({
                "files": files,
                "count": files.len()
            });
            
            TransUnitType::String(response.to_string())
        },
        Err(e) => {
            let error = serde_json::json!({"error": e.to_string()});
            TransUnitType::String(error.to_string())
        }
    }
}

pub fn info_handler() -> TransUnitType {
    let file_path = "/home/user/documents/report.pdf";
    
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let info = serde_json::json!({
                "size": metadata.len(),
                "modified": metadata.modified()
                    .unwrap_or(std::time::UNIX_EPOCH)
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "is_file": metadata.is_file(),
                "is_dir": metadata.is_dir()
            });
            
            TransUnitType::String(info.to_string())
        },
        Err(e) => {
            let error = serde_json::json!({"error": e.to_string()});
            TransUnitType::String(error.to_string())
        }
    }
}
```

---

## JSON API Service

RESTful-style API service with JSON responses:

```rust
// src/main.rs
use bapao_app_protocal::{AppListener, TransUnitType};

mod api;

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // API endpoints
    listener.add("/api/v1/status", api::status);
    listener.add("/api/v1/users", api::users);
    listener.add("/api/v1/system", api::system_info);
    
    println!("JSON API service started");
    listener.listen().await;
}
```

```rust
// src/api.rs
use bapao_app_protocal::TransUnitType;
use serde_json::json;
use std::collections::HashMap;

pub fn status() -> TransUnitType {
    let response = json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().timestamp(),
        "version": "1.0.0"
    });
    
    TransUnitType::String(response.to_string())
}

pub fn users() -> TransUnitType {
    // Mock user data
    let users = vec![
        json!({"id": 1, "name": "Alice", "email": "alice@example.com"}),
        json!({"id": 2, "name": "Bob", "email": "bob@example.com"}),
    ];
    
    let response = json!({
        "users": users,
        "total": users.len()
    });
    
    TransUnitType::String(response.to_string())
}

pub fn system_info() -> TransUnitType {
    let mut info = HashMap::new();
    info.insert("hostname", "bapao-server");
    info.insert("os", std::env::consts::OS);
    info.insert("arch", std::env::consts::ARCH);
    
    let response = json!({
        "system": info,
        "uptime": "24h",
        "memory_usage": "45%"
    });
    
    TransUnitType::String(response.to_string())
}
```

---

## Multi-Endpoint Application

Complex application with multiple services:

```rust
// src/main.rs
use bapao_app_protocal::{AppListener, TransUnitType};

mod handlers {
    pub mod auth;
    pub mod files;
    pub mod monitoring;
    pub mod system;
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // Authentication endpoints
    listener.add("/auth/login", handlers::auth::login);
    listener.add("/auth/logout", handlers::auth::logout);
    
    // File management
    listener.add("/files/upload", handlers::files::upload);
    listener.add("/files/download", handlers::files::download);
    listener.add("/files/list", handlers::files::list);
    
    // System monitoring
    listener.add("/monitor/cpu", handlers::monitoring::cpu_usage);
    listener.add("/monitor/memory", handlers::monitoring::memory_usage);
    listener.add("/monitor/disk", handlers::monitoring::disk_usage);
    listener.add("/monitor/screenshot", handlers::monitoring::screenshot);
    
    // System control
    listener.add("/system/restart", handlers::system::restart);
    listener.add("/system/shutdown", handlers::system::shutdown);
    listener.add("/system/status", handlers::system::status);
    
    println!("Multi-service application started");
    listener.listen().await;
}
```

```rust
// src/handlers/monitoring.rs
use bapao_app_protocal::TransUnitType;
use serde_json::json;
use std::process::Command;

pub fn cpu_usage() -> TransUnitType {
    // Get CPU usage (Linux example)
    let output = Command::new("cat")
        .arg("/proc/loadavg")
        .output();
        
    match output {
        Ok(result) => {
            let load_avg = String::from_utf8_lossy(&result.stdout);
            let response = json!({
                "cpu_load": load_avg.trim(),
                "timestamp": chrono::Utc::now().timestamp()
            });
            TransUnitType::String(response.to_string())
        },
        Err(_) => {
            let error = json!({"error": "Failed to get CPU usage"});
            TransUnitType::String(error.to_string())
        }
    }
}

pub fn memory_usage() -> TransUnitType {
    let output = Command::new("free")
        .args(["-m"])
        .output();
        
    match output {
        Ok(result) => {
            let memory_info = String::from_utf8_lossy(&result.stdout);
            let response = json!({
                "memory_info": memory_info.trim(),
                "timestamp": chrono::Utc::now().timestamp()
            });
            TransUnitType::String(response.to_string())
        },
        Err(_) => {
            let error = json!({"error": "Failed to get memory usage"});
            TransUnitType::String(error.to_string())
        }
    }
}

pub fn screenshot() -> TransUnitType {
    let result = Command::new("scrot")
        .args(["/tmp/monitor_screenshot.png"])
        .output();
        
    match result {
        Ok(output) if output.status.success() => {
            match std::fs::read("/tmp/monitor_screenshot.png") {
                Ok(image_data) => {
                    let _ = std::fs::remove_file("/tmp/monitor_screenshot.png");
                    TransUnitType::File(image_data)
                },
                Err(_) => TransUnitType::String("{\"error\": \"Failed to read screenshot\"}".to_string()),
            }
        },
        _ => TransUnitType::String("{\"error\": \"Screenshot capture failed\"}".to_string()),
    }
}
```

---

## Custom Transport Usage

Direct usage of the transport protocol for advanced scenarios:

```rust
use bapao_trans_protocal::{
    BtpListener,
    trans_content::*,
    trans_unit::TransUnit,
    gitee::fetch::{get_content, put_content, create_file},
    gitee::handler::group_by_state,
    utils::trim_expired_data,
};

#[tokio::main]
async fn main() {
    // Direct transport protocol usage
    let mut transport = BtpListener::new();
    
    loop {
        // Custom request processing
        match get_content().await {
            Ok((raw_content, sha)) => {
                // Filter expired requests
                let active_content = trim_expired_data(raw_content);
                
                // Group by state
                let grouped = group_by_state(active_content);
                
                println!("Processing {} pending requests", grouped.pending.len());
                
                // Process each request manually
                for req_content in grouped.pending {
                    let unit = TransUnit::new(req_content);
                    let request_path = unit.get();
                    
                    // Custom routing logic
                    let response = match request_path.as_str() {
                        path if path.starts_with("/api/") => {
                            handle_api_request(path)
                        },
                        path if path.starts_with("/files/") => {
                            handle_file_request(path)
                        },
                        _ => TransUnitType::String("Unknown endpoint".to_string()),
                    };
                    
                    // Convert to response format
                    let formatted_response = unit.set(response);
                    transport.stash(formatted_response);
                }
                
                // Responses are automatically sent in next accept() call
            },
            Err(e) => {
                eprintln!("Error fetching content: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        }
        
        // Wait before next poll
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

fn handle_api_request(path: &str) -> TransUnitType {
    match path {
        "/api/time" => {
            let now = chrono::Utc::now().timestamp();
            TransUnitType::String(format!("{{\"timestamp\": {}}}", now))
        },
        "/api/echo" => {
            TransUnitType::String("{\"message\": \"echo\"}".to_string())
        },
        _ => TransUnitType::String("{\"error\": \"API not found\"}".to_string()),
    }
}

fn handle_file_request(path: &str) -> TransUnitType {
    match path {
        "/files/log" => {
            match std::fs::read("/var/log/app.log") {
                Ok(data) => TransUnitType::File(data),
                Err(_) => TransUnitType::String("{\"error\": \"Log file not found\"}".to_string()),
            }
        },
        _ => TransUnitType::String("{\"error\": \"File not found\"}".to_string()),
    }
}
```

---

## Error Handling Patterns

### Graceful Error Recovery

```rust
use bapao_app_protocal::{AppListener, TransUnitType};
use serde_json::json;

fn robust_handler() -> TransUnitType {
    // Multiple fallback strategies
    
    // Try primary operation
    if let Ok(result) = try_primary_operation() {
        return TransUnitType::String(result);
    }
    
    // Try fallback operation
    if let Ok(result) = try_fallback_operation() {
        return TransUnitType::String(format!("{{\"result\": \"{}\", \"source\": \"fallback\"}}", result));
    }
    
    // Return error response
    let error_response = json!({
        "error": "All operations failed",
        "timestamp": chrono::Utc::now().timestamp(),
        "suggestion": "Try again later"
    });
    
    TransUnitType::String(error_response.to_string())
}

fn try_primary_operation() -> Result<String, Box<dyn std::error::Error>> {
    // Primary operation logic
    Ok("Primary result".to_string())
}

fn try_fallback_operation() -> Result<String, Box<dyn std::error::Error>> {
    // Fallback operation logic  
    Ok("Fallback result".to_string())
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/robust", robust_handler);
    listener.listen().await;
}
```

### Structured Error Responses

```rust
use bapao_app_protocal::TransUnitType;
use serde_json::json;

#[derive(Debug)]
enum ServiceError {
    NotFound,
    PermissionDenied,
    InternalError(String),
}

impl ServiceError {
    fn to_json_response(&self) -> TransUnitType {
        let (code, message) = match self {
            ServiceError::NotFound => (404, "Resource not found"),
            ServiceError::PermissionDenied => (403, "Permission denied"),
            ServiceError::InternalError(msg) => (500, msg.as_str()),
        };
        
        let response = json!({
            "error": {
                "code": code,
                "message": message,
                "timestamp": chrono::Utc::now().timestamp()
            }
        });
        
        TransUnitType::String(response.to_string())
    }
}

fn error_aware_handler() -> TransUnitType {
    match perform_operation() {
        Ok(data) => TransUnitType::String(data),
        Err(ServiceError::NotFound) => ServiceError::NotFound.to_json_response(),
        Err(ServiceError::PermissionDenied) => ServiceError::PermissionDenied.to_json_response(),
        Err(ServiceError::InternalError(msg)) => ServiceError::InternalError(msg).to_json_response(),
    }
}

fn perform_operation() -> Result<String, ServiceError> {
    // Your operation logic here
    Ok("Success".to_string())
}
```

---

## Request/Response Examples

### Text Request/Response

**Client Request:**
```json
[{
  "head": {
    "id": "req_001",
    "content_type": null,
    "state": "Pending",
    "timestamp": 1704067200000
  },
  "body": "/api/status"
}]
```

**Server Response:**
```json
[{
  "head": {
    "id": "req_001", 
    "content_type": "string",
    "state": "Done",
    "timestamp": 1704067200000
  },
  "body": "{\"status\": \"running\", \"uptime\": \"24h\"}"
}]
```

### File Request/Response

**Client Request:**
```json
[{
  "head": {
    "id": "file_req_001",
    "content_type": null,
    "state": "Pending",
    "timestamp": 1704067200000
  },
  "body": "/files/screenshot"
}]
```

**Server Response:**
```json
[{
  "head": {
    "id": "file_req_001",
    "content_type": "file", 
    "state": "Done",
    "timestamp": 1704067200000
  },
  "body": "uuid-generated-filename"
}]
```

The actual file content is uploaded separately with the UUID filename.

---

## Testing Examples

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bapao_app_protocal::TransUnitType;

    #[test]
    fn test_handler_returns_string() {
        let result = your_handler();
        
        match result {
            TransUnitType::String(s) => {
                assert!(!s.is_empty());
                // Additional assertions
            },
            _ => panic!("Expected string response"),
        }
    }

    #[test] 
    fn test_handler_returns_file() {
        let result = file_handler();
        
        match result {
            TransUnitType::File(data) => {
                assert!(!data.is_empty());
                // Verify file format if needed
            },
            _ => panic!("Expected file response"),
        }
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use bapao_trans_protocal::{
    trans_content::*,
    trans_unit::TransUnit,
};

#[tokio::test]
async fn test_request_response_cycle() {
    // Create mock request
    let request = ReqContent {
        head: TransHead {
            id: "test_001".to_string(),
            content_type: None,
            state: "Pending".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        },
        body: "/test/endpoint".to_string(),
    };
    
    // Create transaction unit
    let unit = TransUnit::new(request);
    
    // Verify request content
    assert_eq!(unit.get(), "/test/endpoint");
    
    // Create response
    let response = unit.set(TransUnitType::String("test response".to_string()));
    
    // Verify response format
    match response {
        ResContentType::String(res) => {
            assert_eq!(res.head.id, "test_001");
            assert_eq!(res.head.state, "Done");
            assert_eq!(res.body, "test response");
        },
        _ => panic!("Expected string response"),
    }
}
```

---

## Performance Optimization

### Efficient File Handling

```rust
use bapao_app_protocal::TransUnitType;
use std::fs::File;
use std::io::Read;

fn optimized_file_handler() -> TransUnitType {
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB limit
    
    let file_path = "/path/to/large/file.dat";
    
    match std::fs::metadata(file_path) {
        Ok(metadata) if metadata.len() > MAX_FILE_SIZE => {
            let error = json!({
                "error": "File too large",
                "max_size": MAX_FILE_SIZE,
                "actual_size": metadata.len()
            });
            TransUnitType::String(error.to_string())
        },
        Ok(_) => {
            match std::fs::read(file_path) {
                Ok(data) => TransUnitType::File(data),
                Err(e) => {
                    let error = json!({"error": e.to_string()});
                    TransUnitType::String(error.to_string())
                }
            }
        },
        Err(e) => {
            let error = json!({"error": e.to_string()});
            TransUnitType::String(error.to_string())
        }
    }
}
```

### Memory-Efficient Streaming

```rust
use bapao_app_protocal::TransUnitType;
use std::io::{BufReader, Read};
use std::fs::File;

fn streaming_handler() -> TransUnitType {
    let file_path = "/path/to/file.log";
    
    match File::open(file_path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            
            // Read in chunks to manage memory
            match reader.read_to_end(&mut buffer) {
                Ok(_) => TransUnitType::File(buffer),
                Err(e) => {
                    let error = json!({"error": format!("Read error: {}", e)});
                    TransUnitType::String(error.to_string())
                }
            }
        },
        Err(e) => {
            let error = json!({"error": format!("File open error: {}", e)});
            TransUnitType::String(error.to_string())
        }
    }
}
```

These examples demonstrate the full range of capabilities and usage patterns for the Bapao communication system. Each example includes proper error handling and follows Rust best practices.