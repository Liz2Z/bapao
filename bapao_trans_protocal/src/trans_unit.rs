use crate::trans_content::{
    ReqContent, ResContentType, ResFileContent, ResStringContent, TransHead, TransUnitType,
};

/// Represents a single request/response transaction unit.
/// 
/// Each `TransUnit` encapsulates one request from an external client and provides
/// methods to access the request data and create properly formatted responses.
/// 
/// # Usage Flow
/// 
/// 1. Create from incoming `ReqContent` using `new()`
/// 2. Get request data using `get()` 
/// 3. Process the request in your application logic
/// 4. Create response using `set()` with your response data
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::{trans_unit::TransUnit, trans_content::*};
/// 
/// // Create from request
/// let request = ReqContent { /* ... */ };
/// let unit = TransUnit::new(request);
/// 
/// // Get request path
/// let path = unit.get();
/// 
/// // Create response
/// let response = unit.set(TransUnitType::String("Hello".to_string()));
/// ```
pub struct TransUnit {
    content: ReqContent,
}

impl TransUnit {
    /// Creates a new `TransUnit` from request content.
    /// 
    /// # Parameters
    /// 
    /// * `content` - The request content received from the transport layer
    /// 
    /// # Returns
    /// 
    /// A new `TransUnit` wrapping the request content
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_trans_protocal::{trans_unit::TransUnit, trans_content::*};
    /// 
    /// let request = ReqContent {
    ///     head: TransHead {
    ///         id: "req_123".to_string(),
    ///         content_type: None,
    ///         state: "Pending".to_string(),
    ///         timestamp: 1234567890,
    ///     },
    ///     body: "/api/status".to_string(),
    /// };
    /// 
    /// let unit = TransUnit::new(request);
    /// ```
    pub fn new(content: ReqContent) -> TransUnit {
        return TransUnit { content: content };
    }

    /// Gets the request body content.
    /// 
    /// Returns a reference to the request body, which typically contains
    /// the route path or command that the external client wants to execute.
    /// 
    /// # Returns
    /// 
    /// `&String` - Reference to the request body content
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let unit = TransUnit::new(request_content);
    /// let route = unit.get();
    /// 
    /// match route.as_str() {
    ///     "/api/status" => handle_status(),
    ///     "/api/data" => handle_data(),
    ///     _ => handle_unknown(),
    /// }
    /// ```
    pub fn get(&self) -> &String {
        return &self.content.body;
    }

    /// Creates a response from the provided content, preserving request metadata.
    /// 
    /// This method takes your response data and wraps it in the proper response
    /// format, copying the request ID and timestamp while updating the state to "Done".
    /// 
    /// # Parameters
    /// 
    /// * `content` - The response data to send back
    /// 
    /// # Returns
    /// 
    /// `ResContentType` - Properly formatted response ready for transmission
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_trans_protocal::{trans_unit::TransUnit, trans_content::TransUnitType};
    /// 
    /// let unit = TransUnit::new(request_content);
    /// 
    /// // Create text response
    /// let text_response = unit.set(TransUnitType::String("Success".to_string()));
    /// 
    /// // Create file response
    /// let file_data = std::fs::read("image.jpg").unwrap();
    /// let file_response = unit.set(TransUnitType::File(file_data));
    /// ```
    pub fn set(&self, content: TransUnitType) -> ResContentType {
        match content {
            TransUnitType::String(str) => ResContentType::String(ResStringContent {
                head: TransHead {
                    id: self.content.head.id.clone(),
                    state: String::from("Done"),
                    timestamp: self.content.head.timestamp,
                    content_type: Option::Some(String::from("string")),
                },
                body: str,
            }),
            TransUnitType::File(str) => ResContentType::File(ResFileContent {
                head: TransHead {
                    id: self.content.head.id.clone(),
                    state: String::from("Done"),
                    timestamp: self.content.head.timestamp,
                    content_type: Option::Some(String::from("file")),
                },
                body: str,
            }),
        }
    }
}
