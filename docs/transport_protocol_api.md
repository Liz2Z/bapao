# Transport Protocol API Documentation

## Overview

The `bapao_trans_protocal` crate provides the low-level transport layer for communicating with Gitee repositories. It handles data serialization, network communication, and protocol management.

## Core Components

### BtpListener

The main transport listener that handles communication with Gitee.

#### Constructor

##### `new() -> Self`

Creates a new `BtpListener` instance.

**Example:**
```rust
use bapao_trans_protocal::BtpListener;

let mut listener = BtpListener::new();
```

#### Methods

##### `accept(&mut self) -> Future<Vec<TransUnit>>`

Fetches new requests from the Gitee repository and returns them as transport units.

**Returns:** `Vec<TransUnit>` - A vector of pending requests

**Example:**
```rust
use bapao_trans_protocal::BtpListener;

#[tokio::main]
async fn main() {
    let mut listener = BtpListener::new();
    
    // Get pending requests
    let requests = listener.accept().await;
    
    for request in requests {
        println!("Received request: {}", request.get());
    }
}
```

**Behavior:**
- Polls Gitee repository for new data
- Filters out expired requests (older than 30 minutes)
- Groups requests by state (Pending/Done)
- Returns only pending requests for processing

##### `stash(&mut self, value: ResContentType)`

Temporarily stores a response without immediately sending it to Gitee.

**Parameters:**
- `value: ResContentType` - The response content to store

**Example:**
```rust
use bapao_trans_protocal::{BtpListener, trans_content::{ResContentType, ResStringContent, TransHead}};

let mut listener = BtpListener::new();

let response = ResContentType::String(ResStringContent {
    head: TransHead {
        id: "req_123".to_string(),
        content_type: Some("string".to_string()),
        state: "Done".to_string(),
        timestamp: 1234567890,
    },
    body: "Response data".to_string(),
});

listener.stash(response);
```

### TransUnit

Represents a single request/response transaction unit.

#### Constructor

##### `new(content: ReqContent) -> TransUnit`

Creates a new `TransUnit` from request content.

**Parameters:**
- `content: ReqContent` - The request content structure

**Example:**
```rust
use bapao_trans_protocal::{trans_unit::TransUnit, trans_content::{ReqContent, TransHead}};

let request = ReqContent {
    head: TransHead {
        id: "req_123".to_string(),
        content_type: Some("string".to_string()),
        state: "Pending".to_string(),
        timestamp: 1234567890,
    },
    body: "/api/status".to_string(),
};

let unit = TransUnit::new(request);
```

#### Methods

##### `get(&self) -> &String`

Gets the request body content.

**Returns:** `&String` - Reference to the request body

**Example:**
```rust
let unit = TransUnit::new(request_content);
let request_path = unit.get();
println!("Request path: {}", request_path);
```

##### `set(&self, content: TransUnitType) -> ResContentType`

Creates a response from the provided content, maintaining the original request metadata.

**Parameters:**
- `content: TransUnitType` - The response content

**Returns:** `ResContentType` - The formatted response

**Example:**
```rust
use bapao_trans_protocal::{trans_content::TransUnitType, trans_unit::TransUnit};

let unit = TransUnit::new(request_content);

// Create string response
let response = unit.set(TransUnitType::String("Hello".to_string()));

// Create file response
let file_data = std::fs::read("image.jpg").unwrap();
let file_response = unit.set(TransUnitType::File(file_data));
```

## Data Types

### TransHead

Metadata structure for all transport communications.

```rust
pub struct TransHead {
    pub id: String,                    // Unique request identifier
    pub content_type: Option<String>,  // "string" or "file"
    pub state: String,                 // "Pending" or "Done"  
    pub timestamp: i64,                // Unix timestamp in milliseconds
}
```

### ReqContent

Structure for incoming requests.

```rust
pub struct ReqContent {
    pub head: TransHead,    // Request metadata
    pub body: String,       // Request content (usually a route path)
}
```

### ResContentType

Enum for response content types.

```rust
pub enum ResContentType {
    String(ResStringContent),    // Text response
    File(ResFileContent),        // Binary file response
}
```

### TransUnitType

Enum for the actual data being transmitted.

```rust
pub enum TransUnitType {
    String(String),      // Text data
    File(Vec<u8>),       // Binary data
}
```

## Gitee Integration

### Fetch Operations

#### `get_content() -> Result<(Vec<ReqContent>, String), Box<dyn std::error::Error>>`

Fetches content from the configured Gitee repository.

**Returns:** 
- `Vec<ReqContent>` - List of requests from the repository
- `String` - SHA hash of the current file state

**Example:**
```rust
use bapao_trans_protocal::gitee::fetch::get_content;

#[tokio::main]
async fn main() {
    match get_content().await {
        Ok((requests, sha)) => {
            println!("Found {} requests, SHA: {}", requests.len(), sha);
            for req in requests {
                println!("Request ID: {}, Body: {}", req.head.id, req.body);
            }
        },
        Err(e) => eprintln!("Error fetching content: {}", e),
    }
}
```

#### `put_content(content: String, sha: String) -> Result<(), Box<dyn Error>>`

Updates the repository file with new content.

**Parameters:**
- `content: String` - JSON string of the content to upload
- `sha: String` - Current SHA hash of the file

**Example:**
```rust
use bapao_trans_protocal::gitee::fetch::put_content;

#[tokio::main]
async fn main() {
    let content = r#"[{"head":{"id":"123","state":"Done","timestamp":1234567890},"body":"response"}]"#;
    let sha = "abc123def456";
    
    match put_content(content.to_string(), sha.to_string()).await {
        Ok(()) => println!("Content updated successfully"),
        Err(e) => eprintln!("Error updating content: {}", e),
    }
}
```

#### `create_file(file_name: &String, file_content: &Vec<u8>) -> Result<(), Box<dyn Error>>`

Creates a new file in the Gitee repository.

**Parameters:**
- `file_name: &String` - Name of the file to create
- `file_content: &Vec<u8>` - Binary content of the file

**Example:**
```rust
use bapao_trans_protocal::gitee::fetch::create_file;

#[tokio::main]
async fn main() {
    let file_content = std::fs::read("local_file.jpg").unwrap();
    let file_name = "uploaded_image.jpg".to_string();
    
    match create_file(&file_name, &file_content).await {
        Ok(()) => println!("File created successfully"),
        Err(e) => eprintln!("Error creating file: {}", e),
    }
}
```

### Handler Operations

#### `group_by_state(content: Vec<ReqContent>) -> ContentGroupByState`

Groups request content by their processing state.

**Parameters:**
- `content: Vec<ReqContent>` - List of requests to group

**Returns:** `ContentGroupByState` - Grouped content structure

```rust
pub struct ContentGroupByState {
    pub pending: Vec<ReqContent>,  // Requests awaiting processing
    pub done: Vec<ReqContent>,     // Completed requests
}
```

**Example:**
```rust
use bapao_trans_protocal::gitee::handler::group_by_state;

let requests = vec![/* your ReqContent items */];
let grouped = group_by_state(requests);

println!("Pending requests: {}", grouped.pending.len());
println!("Completed requests: {}", grouped.done.len());

// Process pending requests
for pending_req in grouped.pending {
    println!("Processing request: {}", pending_req.head.id);
}
```

## Utility Functions

### `trim_expired_data(contents: Vec<ReqContent>) -> Vec<ReqContent>`

Removes expired requests (older than 30 minutes) from the content list.

**Parameters:**
- `contents: Vec<ReqContent>` - List of requests to filter

**Returns:** `Vec<ReqContent>` - Filtered list without expired requests

**Example:**
```rust
use bapao_trans_protocal::utils::trim_expired_data;

let all_requests = vec![/* your ReqContent items */];
let active_requests = trim_expired_data(all_requests);

println!("Active requests: {}", active_requests.len());
```

## Configuration

The transport protocol reads configuration from `bapao.config.json`:

```json
{
  "access_token": "your_gitee_personal_access_token",
  "user_name": "your_gitee_username",
  "repo": "repository_name", 
  "file_path": "communication_file_name"
}
```

## Error Handling

All async functions return `Result` types with appropriate error handling:

- **Network Errors**: HTTP request failures, timeouts
- **Serialization Errors**: JSON parsing failures
- **Authentication Errors**: Invalid access tokens or permissions
- **Repository Errors**: File not found, repository access issues

**Example Error Handling:**
```rust
use bapao_trans_protocal::gitee::fetch::get_content;

#[tokio::main]
async fn main() {
    match get_content().await {
        Ok((content, sha)) => {
            // Process successful response
            println!("Retrieved {} items", content.len());
        },
        Err(e) => {
            eprintln!("Failed to get content: {}", e);
            // Implement fallback logic
        }
    }
}
```

## Performance Considerations

- The system polls Gitee every 10 seconds for new requests
- Expired data is automatically cleaned up to prevent memory leaks
- File uploads are handled asynchronously
- Large files are automatically base64 encoded for transmission

## Thread Safety

The transport protocol components are designed for single-threaded async usage with Tokio. Use appropriate synchronization if sharing across threads.