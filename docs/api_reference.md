# API Reference

## Table of Contents

- [bapao_app_protocal](#bapao_app_protocal)
- [bapao_trans_protocal](#bapao_trans_protocal)
- [Data Types](#data-types)
- [Error Types](#error-types)

## bapao_app_protocal

### AppListener\<T\>

High-level application request listener and router.

```rust
pub struct AppListener<T>
where
    T: Fn() -> TransUnitType,
{
    listener: HashMap<&'static str, T>,
}
```

#### Methods

##### `new()`

```rust
pub fn new() -> Self
```

**Description:** Creates a new `AppListener` instance with an empty route table.

**Returns:** `AppListener<T>` - New listener instance

**Example:**
```rust
use bapao_app_protocal::AppListener;

let mut listener = AppListener::new();
```

---

##### `add()`

```rust
pub fn add(&mut self, key: &'static str, callback: T)
```

**Description:** Registers a callback function for a specific route path.

**Parameters:**
- `key: &'static str` - The route path to handle (e.g., "/api/status")
- `callback: T` - Function that returns `TransUnitType`

**Example:**
```rust
use bapao_app_protocal::{AppListener, TransUnitType};

fn status_handler() -> TransUnitType {
    TransUnitType::String("OK".to_string())
}

let mut listener = AppListener::new();
listener.add("/api/status", status_handler);
```

---

##### `listen()`

```rust
pub async fn listen(&self)
```

**Description:** Starts the listener loop, polling for requests every 10 seconds and routing them to registered handlers.

**Returns:** `Future<()>` - Never returns (infinite loop)

**Example:**
```rust
#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/test", || TransUnitType::String("test".to_string()));
    listener.listen().await;  // Runs forever
}
```

---

## bapao_trans_protocal

### BtpListener

Low-level transport protocol listener for Gitee communication.

```rust
pub struct BtpListener {
    done: Vec<ResStringContent>,
    files: HashMap<String, Vec<u8>>,
}
```

#### Methods

##### `new()`

```rust
pub fn new() -> Self
```

**Description:** Creates a new `BtpListener` instance.

**Returns:** `BtpListener` - New listener instance

**Example:**
```rust
use bapao_trans_protocal::BtpListener;

let mut listener = BtpListener::new();
```

---

##### `accept()`

```rust
pub async fn accept(&mut self) -> Vec<TransUnit>
```

**Description:** Fetches and processes new requests from Gitee repository.

**Returns:** `Vec<TransUnit>` - List of pending requests to process

**Side Effects:**
- Sends any stashed responses to Gitee
- Filters out expired requests
- Groups requests by state

**Example:**
```rust
use bapao_trans_protocal::BtpListener;

#[tokio::main]
async fn main() {
    let mut listener = BtpListener::new();
    
    loop {
        let requests = listener.accept().await;
        
        if requests.is_empty() {
            println!("No new requests");
            continue;
        }
        
        for request in requests {
            let path = request.get();
            println!("Processing request: {}", path);
            
            // Process request and create response
            let response = request.set(TransUnitType::String("Processed".to_string()));
            listener.stash(response);
        }
    }
}
```

---

##### `stash()`

```rust
pub fn stash(&mut self, value: ResContentType)
```

**Description:** Stores a response for later transmission to Gitee.

**Parameters:**
- `value: ResContentType` - The response to store

**Behavior:**
- String responses are stored directly
- File responses are assigned a UUID filename and stored separately

**Example:**
```rust
use bapao_trans_protocal::{BtpListener, trans_content::*};

let mut listener = BtpListener::new();

// Stash string response
let string_response = ResContentType::String(ResStringContent {
    head: TransHead {
        id: "req_123".to_string(),
        content_type: Some("string".to_string()),
        state: "Done".to_string(),
        timestamp: 1234567890,
    },
    body: "Response text".to_string(),
});

listener.stash(string_response);
```

---

### TransUnit

Individual request/response transaction unit.

```rust
pub struct TransUnit {
    content: ReqContent,
}
```

#### Methods

##### `new()`

```rust
pub fn new(content: ReqContent) -> TransUnit
```

**Description:** Creates a new `TransUnit` from request content.

**Parameters:**
- `content: ReqContent` - The request content structure

**Returns:** `TransUnit` - New transaction unit

**Example:**
```rust
use bapao_trans_protocal::{trans_unit::TransUnit, trans_content::*};

let request = ReqContent {
    head: TransHead {
        id: "req_456".to_string(),
        content_type: None,
        state: "Pending".to_string(),
        timestamp: chrono::Utc::now().timestamp_millis(),
    },
    body: "/api/data".to_string(),
};

let unit = TransUnit::new(request);
```

---

##### `get()`

```rust
pub fn get(&self) -> &String
```

**Description:** Gets the request body content (typically a route path).

**Returns:** `&String` - Reference to the request body

**Example:**
```rust
let unit = TransUnit::new(request);
let route = unit.get();

match route.as_str() {
    "/api/status" => handle_status(),
    "/api/data" => handle_data(),
    _ => handle_unknown(),
}
```

---

##### `set()`

```rust
pub fn set(&self, content: TransUnitType) -> ResContentType
```

**Description:** Creates a response from content, preserving request metadata.

**Parameters:**
- `content: TransUnitType` - The response content

**Returns:** `ResContentType` - Formatted response ready for transmission

**Example:**
```rust
use bapao_trans_protocal::{trans_unit::TransUnit, trans_content::TransUnitType};

let unit = TransUnit::new(request);

// Create text response
let text_response = unit.set(TransUnitType::String("Success".to_string()));

// Create file response  
let file_data = std::fs::read("response.pdf").unwrap();
let file_response = unit.set(TransUnitType::File(file_data));
```

---

## Gitee Integration API

### Fetch Operations

#### `get_content()`

```rust
pub async fn get_content() -> Result<(Vec<ReqContent>, String), Box<dyn std::error::Error>>
```

**Description:** Fetches the current content from the configured Gitee repository file.

**Returns:** 
- `Vec<ReqContent>` - Parsed request content from the repository
- `String` - Current SHA hash of the file

**Errors:**
- Network connectivity issues
- Authentication failures  
- JSON parsing errors
- Base64 decoding errors

**Example:**
```rust
use bapao_trans_protocal::gitee::fetch::get_content;

#[tokio::main]
async fn main() {
    match get_content().await {
        Ok((requests, sha)) => {
            println!("Fetched {} requests, SHA: {}", requests.len(), sha);
            for req in requests {
                println!("Request {}: {}", req.head.id, req.body);
            }
        },
        Err(e) => eprintln!("Failed to fetch content: {}", e),
    }
}
```

---

#### `put_content()`

```rust
pub async fn put_content(content: String, sha: String) -> Result<(), Box<dyn std::error::Error>>
```

**Description:** Updates the repository file with new content.

**Parameters:**
- `content: String` - JSON string content to upload
- `sha: String` - Current SHA hash of the file (for conflict detection)

**Errors:**
- Network connectivity issues
- Authentication failures
- SHA conflicts (file was modified by another process)
- Repository write permission issues

**Example:**
```rust
use bapao_trans_protocal::gitee::fetch::put_content;

#[tokio::main]
async fn main() {
    let responses = vec![
        // Your response objects here
    ];
    
    let content = serde_json::to_string(&responses).unwrap();
    let sha = "current_file_sha_hash";
    
    match put_content(content, sha.to_string()).await {
        Ok(()) => println!("Content updated successfully"),
        Err(e) => eprintln!("Failed to update content: {}", e),
    }
}
```

---

#### `create_file()`

```rust
pub async fn create_file(file_name: &String, file_content: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>>
```

**Description:** Creates a new file in the Gitee repository with binary content.

**Parameters:**
- `file_name: &String` - Name of the file to create
- `file_content: &Vec<u8>` - Binary content of the file

**Errors:**
- Network connectivity issues
- Authentication failures
- File already exists
- Repository write permission issues

**Example:**
```rust
use bapao_trans_protocal::gitee::fetch::create_file;

#[tokio::main]
async fn main() {
    let image_data = std::fs::read("screenshot.png").unwrap();
    let filename = "image_123.png".to_string();
    
    match create_file(&filename, &image_data).await {
        Ok(()) => println!("File uploaded successfully"),
        Err(e) => eprintln!("Failed to upload file: {}", e),
    }
}
```

---

### Handler Operations

#### `group_by_state()`

```rust
pub fn group_by_state(content: Vec<ReqContent>) -> ContentGroupByState
```

**Description:** Groups requests by their processing state.

**Parameters:**
- `content: Vec<ReqContent>` - List of requests to group

**Returns:** `ContentGroupByState` - Grouped content structure

**Example:**
```rust
use bapao_trans_protocal::gitee::handler::group_by_state;

let all_requests = vec![/* requests from Gitee */];
let grouped = group_by_state(all_requests);

// Process only pending requests
for pending in grouped.pending {
    println!("Need to process: {}", pending.head.id);
}

// Handle completed requests
for done in grouped.done {
    println!("Already processed: {}", done.head.id);
}
```

---

### Utility Functions

#### `trim_expired_data()`

```rust
pub fn trim_expired_data(contents: Vec<ReqContent>) -> Vec<ReqContent>
```

**Description:** Removes requests older than 30 minutes from the content list.

**Parameters:**
- `contents: Vec<ReqContent>` - List of requests to filter

**Returns:** `Vec<ReqContent>` - Filtered list without expired requests

**Logic:** Compares request timestamp with current time minus 30 minutes

**Example:**
```rust
use bapao_trans_protocal::utils::trim_expired_data;

let all_requests = vec![/* mix of old and new requests */];
let active_requests = trim_expired_data(all_requests);

println!("Filtered out expired requests, {} remaining", active_requests.len());
```

---

## Data Types

### TransHead

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransHead {
    pub id: String,                    // Unique identifier
    pub content_type: Option<String>,  // "string" | "file" | None
    pub state: String,                 // "Pending" | "Done"
    pub timestamp: i64,                // Unix timestamp in milliseconds
}
```

### ReqContent

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqContent {
    pub head: TransHead,    // Request metadata
    pub body: String,       // Request content/route
}
```

### ResStringContent

```rust
pub type ResStringContent = ReqContent;
```

### ResFileContent

```rust
pub struct ResFileContent {
    pub head: TransHead,    // Response metadata
    pub body: Vec<u8>,      // Binary file content
}
```

### ResContentType

```rust
pub enum ResContentType {
    String(ResStringContent),    // Text response
    File(ResFileContent),        // Binary file response
}
```

### TransUnitType

```rust
pub enum TransUnitType {
    String(String),      // Text data for transmission
    File(Vec<u8>),       // Binary data for transmission
}
```

### ContentGroupByState

```rust
#[derive(Debug)]
pub struct ContentGroupByState {
    pub pending: Vec<ReqContent>,    // Requests awaiting processing
    pub done: Vec<ReqContent>,       // Completed requests
}
```

---

## Error Types

All async functions return `Result` types with error handling:

### Common Error Types

- `Box<dyn std::error::Error>` - Generic error for most operations
- `reqwest::Error` - HTTP/network related errors
- `serde_json::Error` - JSON serialization/deserialization errors
- `base64::DecodeError` - Base64 decoding errors

### Error Handling Patterns

```rust
// Pattern 1: Basic error handling
match some_async_operation().await {
    Ok(result) => {
        // Handle success
        println!("Operation succeeded: {:?}", result);
    },
    Err(e) => {
        // Handle error
        eprintln!("Operation failed: {}", e);
    }
}

// Pattern 2: Error propagation
async fn my_function() -> Result<(), Box<dyn std::error::Error>> {
    let (content, sha) = get_content().await?;  // ? operator propagates errors
    put_content("new content".to_string(), sha).await?;
    Ok(())
}

// Pattern 3: Error recovery
let content = match get_content().await {
    Ok((content, _sha)) => content,
    Err(e) => {
        eprintln!("Failed to get content: {}", e);
        vec![]  // Fallback to empty content
    }
};
```

---

## Usage Patterns

### Basic Request Handler

```rust
use bapao_app_protocal::{AppListener, TransUnitType};

fn echo_handler() -> TransUnitType {
    TransUnitType::String("Echo response".to_string())
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/echo", echo_handler);
    listener.listen().await;
}
```

### File Response Handler

```rust
use bapao_app_protocal::{AppListener, TransUnitType};
use std::fs;

fn file_handler() -> TransUnitType {
    match fs::read("/path/to/file.pdf") {
        Ok(data) => TransUnitType::File(data),
        Err(_) => TransUnitType::String("File not found".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/download/file", file_handler);
    listener.listen().await;
}
```

### Multiple Endpoint Handler

```rust
use bapao_app_protocal::{AppListener, TransUnitType};

fn status() -> TransUnitType {
    TransUnitType::String("{\"status\": \"running\"}".to_string())
}

fn version() -> TransUnitType {
    TransUnitType::String("{\"version\": \"1.0.0\"}".to_string())
}

fn health() -> TransUnitType {
    TransUnitType::String("{\"health\": \"ok\"}".to_string())
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    
    listener.add("/api/status", status);
    listener.add("/api/version", version);
    listener.add("/api/health", health);
    
    println!("API server starting...");
    listener.listen().await;
}
```

### Direct Transport Protocol Usage

```rust
use bapao_trans_protocal::{BtpListener, trans_content::TransUnitType};

#[tokio::main]
async fn main() {
    let mut transport = BtpListener::new();
    
    loop {
        let requests = transport.accept().await;
        
        for request in requests {
            let route = request.get();
            
            let response = match route.as_str() {
                "/ping" => TransUnitType::String("pong".to_string()),
                "/time" => {
                    let now = chrono::Utc::now().timestamp().to_string();
                    TransUnitType::String(now)
                },
                _ => TransUnitType::String("Unknown route".to_string()),
            };
            
            let formatted_response = request.set(response);
            transport.stash(formatted_response);
        }
    }
}
```

---

## Constants and Defaults

### Timing Constants

- **Polling Interval**: 10 seconds between Gitee checks
- **Request Expiration**: 30 minutes for request timeout
- **Sleep Duration**: 10 seconds between polling cycles

### Default Values

- **Default State**: "Pending" for new requests, "Done" for responses
- **Default Content Type**: "string" for text, "file" for binary
- **Default Message**: "response" for content updates, "send file" for file creation

---

## Dependencies

### Required Crates

```toml
# bapao_trans_protocal dependencies
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.13.0"
chrono = "0.4.19"
uuid = { version = "0.8", features = ["v4"] }

# app dependencies  
tokio = { version = "1.15.0", features = ["full"] }
image-base64 = "0.1.0"
```

### Feature Requirements

- `tokio` with "full" features for async runtime
- `reqwest` with "json" features for HTTP client
- `serde` with "derive" features for serialization
- `uuid` with "v4" features for unique ID generation