//! # Bapao Screenshot Service Application
//! 
//! This application demonstrates the Bapao communication system by providing
//! a screenshot capture service accessible through Gitee repository communication.
//! 
//! ## Functionality
//! 
//! - Listens for screenshot requests on `/monitor/pic/shot`
//! - Captures screenshots and returns them as binary data
//! - Uses the Bapao application protocol for request handling
//! 
//! ## Usage
//! 
//! 1. Configure `bapao.config.json` with your Gitee repository details
//! 2. Run the application: `cargo run`
//! 3. Send requests by updating the configured Gitee repository file
//! 
//! ## Request Format
//! 
//! ```json
//! [{
//!   "head": {
//!     "id": "screenshot_001",
//!     "content_type": null,
//!     "state": "Pending",
//!     "timestamp": 1704067200000
//!   },
//!   "body": "/monitor/pic/shot"
//! }]
//! ```

use bapao_app_protocal;
use shot_pic::shot_pic;

mod shot_pic;

#[tokio::main]
async fn main() {
    println!("Starting Bapao Screenshot Service...");
    
    let mut btp_listener = bapao_app_protocal::AppListener::new();

    // Register the screenshot endpoint
    btp_listener.add("/monitor/pic/shot", shot_pic);

    println!("Registered endpoint: /monitor/pic/shot");
    println!("Listening for requests...");
    
    btp_listener.listen().await;
}
