# Main Application Documentation

## Overview

The main application (`app/src/main.rs`) demonstrates a practical implementation of the Bapao communication system, specifically providing a screenshot capture service through the Gitee-based communication channel.

## Application Structure

### Entry Point

```rust
// app/src/main.rs
use bapao_app_protocal;
use shot_pic::shot_pic;

mod shot_pic;

#[tokio::main]
async fn main() {
    let mut btp_listener = bapao_app_protocal::AppListener::new();

    btp_listener.add("/monitor/pic/shot", shot_pic);

    btp_listener.listen().await;
}
```

### Key Components

1. **Application Listener**: Uses `bapao_app_protocal::AppListener` for high-level request handling
2. **Screenshot Module**: Implements screenshot capture functionality
3. **Async Runtime**: Uses Tokio for asynchronous operations

## Screenshot Service

### Implementation

```rust
// app/src/shot_pic.rs
extern crate image_base64;

use std::fs;
use std::process;
use bapao_app_protocal::TransUnitType;

pub fn shot_pic() -> TransUnitType {
    // Current implementation reads a static file
    TransUnitType::File(fs::read("/Users/xxx/Downloads/image.jpg").unwrap())
    
    // Commented out: Dynamic screenshot capture
    // if let Ok(mut child) = process::Command::new("fswebcam")
    //     .args(["-r", "1440*720", "/home/pi/image.jpg"])
    //     .spawn()
    // {
    //     child.wait().unwrap();
    //     TransUnitType::File(fs::read("/home/pi/image.jpg").unwrap())
    // } else {
    //     TransUnitType::String(String::from("_"))
    // }
}
```

### Usage

To trigger a screenshot capture, send a request to the Gitee repository:

**Request Format:**
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

**Response:**
The application will respond with the screenshot as a binary file, which will be uploaded to the Gitee repository with a UUID filename.

## Production Deployment

### Enhanced Screenshot Implementation

For production use, consider this improved implementation:

```rust
// src/screenshot.rs
use bapao_app_protocal::TransUnitType;
use std::{fs, process::Command, path::Path};
use serde_json::json;

pub fn capture_screenshot() -> TransUnitType {
    let temp_path = "/tmp/bapao_screenshot.png";
    
    // Cross-platform screenshot capture
    let capture_result = capture_by_platform(temp_path);
    
    match capture_result {
        Ok(()) => {
            match fs::read(temp_path) {
                Ok(image_data) => {
                    // Clean up temporary file
                    let _ = fs::remove_file(temp_path);
                    
                    // Return image data
                    TransUnitType::File(image_data)
                },
                Err(e) => {
                    let error = json!({
                        "error": "Failed to read screenshot",
                        "details": e.to_string()
                    });
                    TransUnitType::String(error.to_string())
                }
            }
        },
        Err(e) => {
            let error = json!({
                "error": "Screenshot capture failed",
                "details": e
            });
            TransUnitType::String(error.to_string())
        }
    }
}

fn capture_by_platform(output_path: &str) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        capture_linux(output_path)
    }
    
    #[cfg(target_os = "macos")]
    {
        capture_macos(output_path)
    }
    
    #[cfg(target_os = "windows")]
    {
        capture_windows(output_path)
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}

#[cfg(target_os = "linux")]
fn capture_linux(output_path: &str) -> Result<(), String> {
    // Try different Linux screenshot tools
    let tools = [
        ("scrot", vec![output_path]),
        ("gnome-screenshot", vec!["-f", output_path]),
        ("import", vec!["-window", "root", output_path]),
    ];
    
    for (tool, args) in &tools {
        if let Ok(status) = Command::new(tool).args(args).status() {
            if status.success() {
                return Ok(());
            }
        }
    }
    
    Err("No screenshot tool available".to_string())
}

#[cfg(target_os = "macos")]
fn capture_macos(output_path: &str) -> Result<(), String> {
    let status = Command::new("screencapture")
        .args([output_path])
        .status()
        .map_err(|e| e.to_string())?;
        
    if status.success() {
        Ok(())
    } else {
        Err("screencapture command failed".to_string())
    }
}

#[cfg(target_os = "windows")]
fn capture_windows(output_path: &str) -> Result<(), String> {
    // Use PowerShell for Windows screenshot
    let ps_script = format!(
        "Add-Type -AssemblyName System.Windows.Forms; \
         $screen = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds; \
         $bitmap = New-Object System.Drawing.Bitmap $screen.Width, $screen.Height; \
         $graphics = [System.Drawing.Graphics]::FromImage($bitmap); \
         $graphics.CopyFromScreen($screen.Location, [System.Drawing.Point]::Empty, $screen.Size); \
         $bitmap.Save('{}', [System.Drawing.Imaging.ImageFormat]::Png)",
        output_path
    );
    
    let status = Command::new("powershell")
        .args(["-Command", &ps_script])
        .status()
        .map_err(|e| e.to_string())?;
        
    if status.success() {
        Ok(())
    } else {
        Err("PowerShell screenshot failed".to_string())
    }
}
```

### Main Application with Enhanced Features

```rust
// src/main.rs
use bapao_app_protocal::{AppListener, TransUnitType};

mod screenshot;
mod system_info;
mod file_operations;

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // Screenshot endpoints
    listener.add("/monitor/pic/shot", screenshot::capture_screenshot);
    listener.add("/monitor/pic/shot/thumb", screenshot::capture_thumbnail);
    
    // System monitoring
    listener.add("/system/info", system_info::get_system_info);
    listener.add("/system/status", system_info::get_status);
    listener.add("/system/uptime", system_info::get_uptime);
    
    // File operations
    listener.add("/files/logs", file_operations::get_logs);
    listener.add("/files/config", file_operations::get_config);
    
    println!("Bapao application started with endpoints:");
    println!("  /monitor/pic/shot - Capture screenshot");
    println!("  /monitor/pic/shot/thumb - Capture thumbnail");
    println!("  /system/info - Get system information");
    println!("  /system/status - Get system status");
    println!("  /system/uptime - Get system uptime");
    println!("  /files/logs - Get application logs");
    println!("  /files/config - Get configuration info");
    println!();
    println!("Listening for requests...");
    
    listener.listen().await;
}
```

## Building and Running

### Development Build

```bash
# Build in debug mode
cargo build

# Run the application
cargo run
```

### Production Build

```bash
# Build optimized release version
cargo build --release

# Run the optimized binary
./target/release/app
```

### Cross-Platform Build

```bash
# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Build for Raspberry Pi
cargo build --release --target arm-unknown-linux-gnueabihf
```

## Deployment

### Systemd Service (Linux)

Create a systemd service file:

```ini
# /etc/systemd/system/bapao.service
[Unit]
Description=Bapao Communication Service
After=network.target

[Service]
Type=simple
User=bapao
WorkingDirectory=/opt/bapao
ExecStart=/opt/bapao/target/release/app
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl enable bapao.service
sudo systemctl start bapao.service

# Check status
sudo systemctl status bapao.service

# View logs
sudo journalctl -u bapao.service -f
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

# Install screenshot dependencies
RUN apt-get update && apt-get install -y \
    scrot \
    imagemagick \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app /usr/local/bin/bapao
COPY bapao.config.json /etc/bapao/

WORKDIR /etc/bapao
CMD ["bapao"]
```

```bash
# Build and run
docker build -t bapao-app .
docker run -d --name bapao bapao-app
```

## Monitoring and Logging

### Application Logging

```rust
// Enhanced main.rs with logging
use bapao_app_protocal::{AppListener, TransUnitType};
use log::{info, error, warn};

mod screenshot;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();
    
    info!("Starting Bapao application");
    
    let mut listener = AppListener::new();
    listener.add("/monitor/pic/shot", screenshot_with_logging);
    
    info!("Registered endpoints: /monitor/pic/shot");
    info!("Starting listener...");
    
    listener.listen().await;
}

fn screenshot_with_logging() -> TransUnitType {
    info!("Screenshot request received");
    
    match screenshot::capture_screenshot() {
        TransUnitType::File(data) => {
            info!("Screenshot captured successfully, size: {} bytes", data.len());
            TransUnitType::File(data)
        },
        TransUnitType::String(error) => {
            error!("Screenshot capture failed: {}", error);
            TransUnitType::String(error)
        }
    }
}
```

### Health Check Endpoint

```rust
fn health_check() -> TransUnitType {
    let health_status = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().timestamp(),
        "uptime": get_uptime_seconds(),
        "memory_usage": get_memory_usage(),
        "last_request": get_last_request_time(),
    });
    
    TransUnitType::String(health_status.to_string())
}

fn get_uptime_seconds() -> u64 {
    // Implementation depends on platform
    0 // Placeholder
}

fn get_memory_usage() -> f64 {
    // Implementation depends on platform  
    0.0 // Placeholder
}

fn get_last_request_time() -> i64 {
    // Track last request timestamp
    0 // Placeholder
}
```

## Security Considerations

### Secure File Access

```rust
use std::path::{Path, PathBuf};

fn secure_file_handler() -> TransUnitType {
    let allowed_directory = "/opt/bapao/files";
    let requested_file = "document.pdf"; // This would come from request parsing
    
    // Prevent directory traversal attacks
    let safe_path = Path::new(allowed_directory).join(requested_file);
    let canonical_path = match safe_path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            let error = json!({"error": "Invalid file path"});
            return TransUnitType::String(error.to_string());
        }
    };
    
    // Ensure the path is within allowed directory
    if !canonical_path.starts_with(allowed_directory) {
        let error = json!({"error": "Access denied"});
        return TransUnitType::String(error.to_string());
    }
    
    // Safe to read file
    match std::fs::read(&canonical_path) {
        Ok(data) => TransUnitType::File(data),
        Err(e) => {
            let error = json!({"error": e.to_string()});
            TransUnitType::String(error.to_string())
        }
    }
}
```

### Request Validation

```rust
fn validated_handler() -> TransUnitType {
    // In a real implementation, you'd have access to the request
    // This is a conceptual example of validation patterns
    
    let request_size_limit = 1024 * 1024; // 1MB
    let rate_limit_per_minute = 60;
    
    // Validate request size, rate limits, authentication, etc.
    // Return appropriate error responses for invalid requests
    
    TransUnitType::String("Validated response".to_string())
}
```

## Troubleshooting

### Common Issues

1. **Application won't start**
   - Check configuration file exists and is valid
   - Verify Gitee access token permissions
   - Ensure repository and file exist

2. **Screenshots not working**
   - Install required screenshot tools (`scrot`, `gnome-screenshot`, etc.)
   - Check file permissions for temporary directories
   - Verify image processing dependencies

3. **No responses received**
   - Check Gitee repository permissions
   - Verify network connectivity
   - Monitor application logs for errors

### Debug Mode

Run with debug logging enabled:

```bash
RUST_LOG=debug cargo run
```

### Verbose Logging

```bash
RUST_LOG=trace cargo run
```

## Performance Tuning

### Memory Optimization

```rust
// Optimized main.rs for memory efficiency
use bapao_app_protocal::{AppListener, TransUnitType};

#[tokio::main]
async fn main() {
    // Set memory limits
    const MAX_RESPONSE_SIZE: usize = 50 * 1024 * 1024; // 50MB
    
    let mut listener = AppListener::new();
    
    listener.add("/monitor/pic/shot", || {
        let result = capture_screenshot();
        
        match &result {
            TransUnitType::File(data) if data.len() > MAX_RESPONSE_SIZE => {
                let error = json!({
                    "error": "Response too large",
                    "max_size": MAX_RESPONSE_SIZE,
                    "actual_size": data.len()
                });
                TransUnitType::String(error.to_string())
            },
            _ => result,
        }
    });
    
    listener.listen().await;
}
```

### CPU Optimization

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

// Cache frequently accessed data
struct AppState {
    screenshot_cache: Option<Vec<u8>>,
    cache_timestamp: i64,
}

impl AppState {
    fn new() -> Self {
        AppState {
            screenshot_cache: None,
            cache_timestamp: 0,
        }
    }
    
    fn is_cache_valid(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        now - self.cache_timestamp < 60 // 60 second cache
    }
}

// Usage with shared state
#[tokio::main]
async fn main() {
    let app_state = Arc::new(Mutex::new(AppState::new()));
    
    let mut listener = AppListener::new();
    
    // Note: This is conceptual - actual implementation would need
    // different approach since AppListener doesn't support closures with captures
    listener.add("/monitor/pic/shot", move || {
        cached_screenshot_handler(&app_state)
    });
    
    listener.listen().await;
}
```

## Extending the Application

### Adding New Endpoints

1. **Create Handler Function**

```rust
// src/handlers/new_feature.rs
use bapao_app_protocal::TransUnitType;

pub fn new_feature_handler() -> TransUnitType {
    // Your implementation here
    TransUnitType::String("New feature response".to_string())
}
```

2. **Register in Main**

```rust
// src/main.rs
mod handlers;

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // Existing endpoints
    listener.add("/monitor/pic/shot", shot_pic::shot_pic);
    
    // New endpoint
    listener.add("/api/new-feature", handlers::new_feature::new_feature_handler);
    
    listener.listen().await;
}
```

### Modular Architecture

```rust
// src/main.rs
use bapao_app_protocal::AppListener;

mod modules {
    pub mod monitoring;
    pub mod files;
    pub mod system;
    pub mod api;
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // Register all modules
    modules::monitoring::register_routes(&mut listener);
    modules::files::register_routes(&mut listener);
    modules::system::register_routes(&mut listener);
    modules::api::register_routes(&mut listener);
    
    println!("All modules registered, starting listener...");
    listener.listen().await;
}
```

```rust
// src/modules/monitoring.rs
use bapao_app_protocal::{AppListener, TransUnitType};

pub fn register_routes<T>(listener: &mut AppListener<T>) 
where 
    T: Fn() -> TransUnitType 
{
    listener.add("/monitor/screenshot", capture_screenshot);
    listener.add("/monitor/cpu", get_cpu_usage);
    listener.add("/monitor/memory", get_memory_usage);
}

pub fn capture_screenshot() -> TransUnitType {
    // Implementation
    TransUnitType::String("Screenshot captured".to_string())
}

pub fn get_cpu_usage() -> TransUnitType {
    // Implementation
    TransUnitType::String("{\"cpu_usage\": \"45%\"}".to_string())
}

pub fn get_memory_usage() -> TransUnitType {
    // Implementation  
    TransUnitType::String("{\"memory_usage\": \"60%\"}".to_string())
}
```

This main application serves as a foundation that can be extended with additional functionality while maintaining the core communication protocol.