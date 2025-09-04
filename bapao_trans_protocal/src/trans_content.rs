use serde::{Deserialize, Serialize};

/// Metadata header for all transport communications.
/// 
/// `TransHead` contains essential information about each request or response,
/// including unique identification, content type, processing state, and timing.
/// 
/// # Fields
/// 
/// * `id` - Unique identifier for the request/response pair
/// * `content_type` - Type of content: "string", "file", or None for requests
/// * `state` - Processing state: "Pending" for requests, "Done" for responses
/// * `timestamp` - Unix timestamp in milliseconds when the request was created
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::trans_content::TransHead;
/// 
/// let header = TransHead {
///     id: "req_001".to_string(),
///     content_type: Some("string".to_string()),
///     state: "Done".to_string(),
///     timestamp: chrono::Utc::now().timestamp_millis(),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransHead {
    pub id: String,
    /// Data type being transmitted: "file" for binary data, "string" for text data
    pub content_type: Option<String>,
    /// Processing state: "Pending" for new requests, "Done" for completed responses
    pub state: String,
    /// Unix timestamp in milliseconds when the request was created
    pub timestamp: i64,
}

/// Request content structure for incoming communications.
/// 
/// `ReqContent` represents a complete request from an external client,
/// including metadata and the actual request data.
/// 
/// # Fields
/// 
/// * `head` - Request metadata (ID, state, timestamp, etc.)
/// * `body` - Request content, typically a route path or command
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::trans_content::*;
/// 
/// let request = ReqContent {
///     head: TransHead {
///         id: "req_001".to_string(),
///         content_type: None,
///         state: "Pending".to_string(),
///         timestamp: chrono::Utc::now().timestamp_millis(),
///     },
///     body: "/api/status".to_string(),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqContent {
    pub head: TransHead,
    pub body: String,
}

/// Response content structure for binary file responses.
/// 
/// Used when responding with file data such as images, documents, or other binary content.
/// 
/// # Fields
/// 
/// * `head` - Response metadata
/// * `body` - Binary file content
pub struct ResFileContent {
    pub head: TransHead,
    pub body: Vec<u8>,
}

/// Response content structure for text responses.
/// 
/// This is an alias for `ReqContent` since text responses have the same structure
/// as requests (metadata + string body).
pub type ResStringContent = ReqContent;

/// Enum for different types of response content.
/// 
/// Represents the two types of responses that can be sent back to external clients.
/// 
/// # Variants
/// 
/// * `String(ResStringContent)` - Text-based response (JSON, plain text, etc.)
/// * `File(ResFileContent)` - Binary file response (images, documents, etc.)
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::trans_content::*;
/// 
/// // Text response
/// let text_response = ResContentType::String(ResStringContent {
///     head: TransHead { /* ... */ },
///     body: "Hello, World!".to_string(),
/// });
/// 
/// // File response
/// let file_data = std::fs::read("image.png").unwrap();
/// let file_response = ResContentType::File(ResFileContent {
///     head: TransHead { /* ... */ },
///     body: file_data,
/// });
/// ```
pub enum ResContentType {
    String(ResStringContent),
    File(ResFileContent),
}

/// Enum for the actual data being transmitted in requests and responses.
/// 
/// This is the simplified data type used by application handlers, before
/// being wrapped in the full protocol structure.
/// 
/// # Variants
/// 
/// * `String(String)` - Text data
/// * `File(Vec<u8>)` - Binary file data
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::trans_content::TransUnitType;
/// 
/// // Return text data
/// fn text_handler() -> TransUnitType {
///     TransUnitType::String("Response text".to_string())
/// }
/// 
/// // Return file data
/// fn file_handler() -> TransUnitType {
///     let file_data = std::fs::read("document.pdf").unwrap();
///     TransUnitType::File(file_data)
/// }
/// ```
pub enum TransUnitType {
    String(String),
    File(Vec<u8>),
}
