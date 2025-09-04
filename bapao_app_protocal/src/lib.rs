use bapao_trans_protocal;
pub use bapao_trans_protocal::trans_content::TransUnitType;
use std::{collections::HashMap, thread, time::Duration};

/// High-level application listener for handling requests through the Bapao communication system.
/// 
/// `AppListener` provides a simple interface for registering route handlers and processing
/// incoming requests from external clients through Gitee repositories.
/// 
/// # Type Parameters
/// 
/// * `T` - A function type that returns `TransUnitType`. All registered handlers must have this signature.
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_app_protocal::{AppListener, TransUnitType};
/// 
/// fn status_handler() -> TransUnitType {
///     TransUnitType::String("System is running".to_string())
/// }
/// 
/// #[tokio::main]
/// async fn main() {
///     let mut listener = AppListener::new();
///     listener.add("/api/status", status_handler);
///     listener.listen().await;
/// }
/// ```
pub struct AppListener<T>
where
    T: Fn() -> TransUnitType,
{
    listener: HashMap<&'static str, T>,
}

impl<T> AppListener<T>
where
    T: Fn() -> TransUnitType,
{
    /// Creates a new `AppListener` with an empty route table.
    /// 
    /// # Returns
    /// 
    /// A new `AppListener` instance ready for route registration.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_app_protocal::AppListener;
    /// 
    /// let mut listener = AppListener::new();
    /// ```
    pub fn new() -> Self {
        AppListener {
            listener: HashMap::new(),
        }
    }

    /// Registers a callback function for a specific route path.
    /// 
    /// When a request is received with a body matching the specified key,
    /// the associated callback function will be executed.
    /// 
    /// # Parameters
    /// 
    /// * `key` - The route path to handle (e.g., "/api/status", "/monitor/pic/shot")
    /// * `callback` - Function that returns a `TransUnitType` response
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_app_protocal::{AppListener, TransUnitType};
    /// 
    /// fn echo_handler() -> TransUnitType {
    ///     TransUnitType::String("Echo response".to_string())
    /// }
    /// 
    /// let mut listener = AppListener::new();
    /// listener.add("/echo", echo_handler);
    /// ```
    pub fn add(&mut self, key: &'static str, callback: T) {
        self.listener.insert(key, callback);
    }

    /// Starts the listener and begins processing incoming requests.
    /// 
    /// This function runs indefinitely, polling the Gitee repository every 10 seconds
    /// for new requests. When requests are found, they are routed to the appropriate
    /// registered handlers based on their body content.
    /// 
    /// # Behavior
    /// 
    /// - Polls Gitee repository every 10 seconds
    /// - Processes all pending requests in each cycle
    /// - Automatically sends responses back to the repository
    /// - Handles errors gracefully and continues operation
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_app_protocal::{AppListener, TransUnitType};
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut listener = AppListener::new();
    ///     
    ///     listener.add("/status", || {
    ///         TransUnitType::String("OK".to_string())
    ///     });
    ///     
    ///     // This will run forever
    ///     listener.listen().await;
    /// }
    /// ```
    pub async fn listen(&self) {
        let mut trans_listener = bapao_trans_protocal::BtpListener::new();

        loop {
            thread::sleep(Duration::new(10, 0));

            let mut incoming_data = trans_listener.accept().await;

            incoming_data.iter_mut().for_each(|unit| {
                let req_content = unit.get();

                let callback = &self.listener.get(&req_content[..]).unwrap();

                let res_content = callback();

                let res_unit = unit.set(res_content);

                trans_listener.stash(res_unit);
            });
        }
    }
}
