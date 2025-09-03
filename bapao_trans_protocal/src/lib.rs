mod gitee;
pub mod trans_content;
pub mod trans_unit;
mod utils;

use gitee::{
    fetch::{self as gitee_fetch},
    handler::{self as gitee_handler},
};
use serde_json;
use std::collections::HashMap;
use trans_content::{ReqContent, ResContentType, ResStringContent};
use trans_unit::TransUnit;
use uuid::Uuid;

/// Transport protocol listener for Gitee-based communication.
/// 
/// `BtpListener` handles the low-level communication with Gitee repositories,
/// including fetching requests, managing responses, and handling file transfers.
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::BtpListener;
/// 
/// #[tokio::main]
/// async fn main() {
///     let mut listener = BtpListener::new();
///     
///     // Process requests
///     let requests = listener.accept().await;
///     for request in requests {
///         // Handle request and create response
///         let response = request.set(TransUnitType::String("OK".to_string()));
///         listener.stash(response);
///     }
/// }
/// ```
pub struct BtpListener {
    done: Vec<ResStringContent>,
    files: HashMap<String, Vec<u8>>,
}

impl BtpListener {
    /// Creates a new `BtpListener` instance.
    /// 
    /// Initializes empty storage for completed responses and file data.
    /// 
    /// # Returns
    /// 
    /// A new `BtpListener` ready to handle transport operations.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_trans_protocal::BtpListener;
    /// 
    /// let mut listener = BtpListener::new();
    /// ```
    pub fn new() -> Self {
        BtpListener {
            done: vec![],
            files: HashMap::new(),
        }
    }

    /// Fetches new requests from the Gitee repository and returns pending requests.
    /// 
    /// This method polls the configured Gitee repository, processes the content,
    /// and returns any pending requests that need to be handled. It also sends
    /// any previously stashed responses back to the repository.
    /// 
    /// # Returns
    /// 
    /// `Vec<TransUnit>` - A vector of pending requests to process
    /// 
    /// # Behavior
    /// 
    /// - Fetches content from Gitee repository
    /// - Filters out expired requests (older than 30 minutes)
    /// - Groups requests by state (Pending/Done)
    /// - Sends stashed responses to repository
    /// - Returns only pending requests for processing
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_trans_protocal::BtpListener;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut listener = BtpListener::new();
    ///     
    ///     loop {
    ///         let requests = listener.accept().await;
    ///         
    ///         for request in requests {
    ///             println!("Processing: {}", request.get());
    ///             // Handle request...
    ///         }
    ///         
    ///         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    ///     }
    /// }
    /// ```
    pub async fn accept(&mut self) -> Vec<TransUnit> {
        // 获取gitee数据
        let (trans_content, sha) = gitee_fetch::get_content().await.unwrap_or_else(|err| {
            eprintln!("获取gitee内容出错：");
            eprintln!("{:#?}", err);

            // FIXME
            (vec![], String::from(""))
        });

        // 将获取到的数据按照 (已处理\未处理) 进行分类
        // FIXME 如果是很久之前就发出的Pending 数据呢？是不是应该失效掉
        let grouped_content = gitee_handler::group_by_state(trans_content);

        if grouped_content.pending.len() == 0 && self.done.len() == 0 {
            println!("无数据需要传输！");
            return vec![];
        } else {
            println!(
                "接收到新的请求：{} 个。已处理的待响应请求：{} 个。",
                grouped_content.pending.len(),
                self.done.len()
            );
        }

        self._send(sha, grouped_content.done).await;

        grouped_content
            .pending
            .into_iter()
            .map(|content| TransUnit::new(content))
            .collect()
    }

    /// Temporarily stores a response without immediately sending it to Gitee.
    /// 
    /// Responses are queued and will be sent to the repository during the next
    /// `accept()` call. This allows batching multiple responses together for
    /// more efficient communication.
    /// 
    /// # Parameters
    /// 
    /// * `value` - The response content to store
    /// 
    /// # Behavior
    /// 
    /// - String responses are stored directly in the done queue
    /// - File responses are assigned a UUID filename and stored separately
    /// - Files will be uploaded to Gitee as separate files
    /// - String responses will be included in the main communication file
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use bapao_trans_protocal::{BtpListener, trans_content::*};
    /// 
    /// let mut listener = BtpListener::new();
    /// 
    /// // Stash a string response
    /// let response = ResContentType::String(ResStringContent {
    ///     head: TransHead {
    ///         id: "req_123".to_string(),
    ///         content_type: Some("string".to_string()),
    ///         state: "Done".to_string(),
    ///         timestamp: 1234567890,
    ///     },
    ///     body: "Response data".to_string(),
    /// });
    /// 
    /// listener.stash(response);
    /// ```
    pub fn stash(&mut self, value: ResContentType) -> () {
        match value {
            ResContentType::String(val) => {
                self.done.push(val);
            }

            ResContentType::File(val) => {
                let file_name = Uuid::new_v4().to_string();
                let file_content = val.body;
                self.files.insert(file_name.clone(), file_content);
                self.done.push(ResStringContent {
                    head: val.head,
                    body: file_name,
                });
            }
        }
    }

    async fn _send(&mut self, sha: String, trans_content_vec: Vec<ReqContent>) -> () {
        // 发送 文件 内容
        let file_map = &self.files;
        // file_map.into_iter().for_each(|(file_name, file_content)| {
        //     let _ = async {
        //         let _ = gitee_fetch::create_file(file_name, file_content).await;
        //     };
        // });

        for (file_name, file_content) in file_map.into_iter() {
            let _ = gitee_fetch::create_file(file_name, file_content).await;
        }

        let _ = &mut &self.files.clear();

        // FIXME 删除gitee中的失效文件
        // FIXME 不需要等待请求响应成功失败，只要发送出去就行，以提高系统效率

        // 发送io 内容
        let mut trimed_content = utils::trim_expired_data(trans_content_vec);

        // 将当前已经处理完毕的数据 与 之前存起来的数据合并
        trimed_content.append(&mut self.done);

        let content = serde_json::to_string(&trimed_content).unwrap_or_else(|err| {
            println!("生成 io 内容出错！");
            println!("Cause: {}", err);
            // 出错就只能空数组兜底了
            String::from("[]")
        });

        gitee_fetch::put_content(content, sha)
            .await
            .unwrap_or_else(|err| {
                println!("更新数据出错！");
                println!("Cause: {}", err);
            });
    }
}
