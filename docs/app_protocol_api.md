# Application Protocol API Documentation

## Overview

The `bapao_app_protocal` crate provides a high-level interface for handling application requests and responses in the Bapao communication system.

## Public API

### AppListener\<T\>

The main struct for handling incoming requests and routing them to appropriate handlers.

#### Type Parameters

- `T: Fn() -> TransUnitType` - A function type that returns a `TransUnitType`

#### Methods

##### `new() -> Self`

Creates a new `AppListener` instance.

**Example:**
```rust
use bapao_app_protocal::AppListener;

let mut listener = AppListener::new();
```

##### `add(&mut self, key: &'static str, callback: T)`

Registers a callback function for a specific route.

**Parameters:**
- `key: &'static str` - The route path to handle
- `callback: T` - The function to call when this route is requested

**Example:**
```rust
use bapao_app_protocal::{AppListener, TransUnitType};

fn handle_request() -> TransUnitType {
    TransUnitType::String("Hello, World!".to_string())
}

let mut listener = AppListener::new();
listener.add("/api/hello", handle_request);
```

##### `listen(&self) -> Future<()>`

Starts the listener and begins processing incoming requests asynchronously.

**Example:**
```rust
use bapao_app_protocal::AppListener;

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // Add your routes here
    listener.add("/api/status", || {
        TransUnitType::String("OK".to_string())
    });
    
    // Start listening for requests
    listener.listen().await;
}
```

## Re-exported Types

### TransUnitType

Re-exported from `bapao_trans_protocal::trans_content::TransUnitType`.

An enum representing the type of data that can be transmitted:

```rust
pub enum TransUnitType {
    String(String),    // Text data
    File(Vec<u8>),     // Binary file data
}
```

**Usage Examples:**

```rust
use bapao_app_protocal::TransUnitType;

// Return text response
fn text_handler() -> TransUnitType {
    TransUnitType::String("Response text".to_string())
}

// Return file response
fn file_handler() -> TransUnitType {
    let file_data = std::fs::read("path/to/file.jpg").unwrap();
    TransUnitType::File(file_data)
}
```

## Complete Example

Here's a complete example of setting up an application with multiple endpoints:

```rust
use bapao_app_protocal::{AppListener, TransUnitType};
use std::fs;

// Handler for status endpoint
fn status_handler() -> TransUnitType {
    TransUnitType::String("System is running".to_string())
}

// Handler for screenshot endpoint  
fn screenshot_handler() -> TransUnitType {
    match fs::read("/path/to/screenshot.jpg") {
        Ok(data) => TransUnitType::File(data),
        Err(_) => TransUnitType::String("Screenshot failed".to_string()),
    }
}

// Handler for system info endpoint
fn system_info_handler() -> TransUnitType {
    let info = format!(
        "{{\"hostname\": \"{}\", \"uptime\": \"{}\"}}",
        "localhost",
        "24h"
    );
    TransUnitType::String(info)
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    // Register endpoints
    listener.add("/api/status", status_handler);
    listener.add("/monitor/pic/shot", screenshot_handler);
    listener.add("/system/info", system_info_handler);
    
    println!("Starting Bapao application listener...");
    listener.listen().await;
}
```

## Request Flow

1. **Request Registration**: Use `add()` to register route handlers
2. **Listener Start**: Call `listen()` to start processing requests
3. **Request Processing**: The listener polls for new requests every 10 seconds
4. **Route Matching**: Incoming requests are matched against registered routes
5. **Handler Execution**: The appropriate callback function is executed
6. **Response Handling**: The response is automatically sent back through the transport layer

## Error Handling

The application protocol layer handles errors gracefully:

- Invalid routes result in no action (the request is ignored)
- Handler panics are caught by the transport layer
- Network errors are handled by the transport protocol layer

## Thread Safety

The `AppListener` is designed to be used in a single-threaded async context. All operations are async-aware and use Tokio for concurrency.