use base64;
use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;

use crate::trans_content::ReqContent;

use super::utils;

// Gitee返回完整版：
//
// GiteeResponse {
//     encoding: "base64",
//     type: "file",
//     size: 235,
//     name: "io",
//     path: "io",
//     content: "WwoImIAgQ==",
//     sha: "xxxxxxxxxxxxx",
//     url: "https://gitee.com/api/v5/repos/username/repo/contents/io",
//     html_url: "https://gitee.com/username/repo/blob/master/io",
//     download_url: "https://gitee.com/username/reporaw/master/io",
//     _links: GiteeResponseLink {
//         html: "https://gitee.com/username/repo/blob/master/io",
//         _self: "https://gitee.com/api/v5/repos/username/repo/contents/io",
//     },
// }

#[derive(Serialize, Deserialize, Debug)]
struct GiteeResponse {
    content: String,
    sha: String,
}

/// Fetches content from the configured Gitee repository file.
/// 
/// Retrieves the communication file from Gitee, decodes the base64 content,
/// and parses it as JSON to extract request data.
/// 
/// # Returns
/// 
/// `Result<(Vec<ReqContent>, String), Box<dyn std::error::Error>>`
/// 
/// On success:
/// * `Vec<ReqContent>` - Parsed request content from the repository
/// * `String` - Current SHA hash of the file (needed for updates)
/// 
/// # Errors
/// 
/// * Network connectivity issues
/// * Authentication failures (invalid access token)
/// * Repository or file not found
/// * Base64 decoding errors
/// * JSON parsing errors
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::gitee::fetch::get_content;
/// 
/// #[tokio::main]
/// async fn main() {
///     match get_content().await {
///         Ok((requests, sha)) => {
///             println!("Found {} requests, SHA: {}", requests.len(), sha);
///             for req in requests {
///                 println!("Request: {} -> {}", req.head.id, req.body);
///             }
///         },
///         Err(e) => eprintln!("Failed to fetch: {}", e),
///     }
/// }
/// ```
/// 
/// # Configuration Required
/// 
/// This function reads from `bapao.config.json` in the project root:
/// ```json
/// {
///   "access_token": "your_gitee_token",
///   "user_name": "username", 
///   "repo": "repository_name",
///   "file_path": "communication_file"
/// }
/// ```
pub async fn get_content() -> Result<(Vec<ReqContent>, String), Box<dyn std::error::Error>> {
    let config: HashMap<String, String> = utils::read_config()?;

    let url = String::from("https://gitee.com/api/v5/repos/")
        + config.get("user_name").unwrap()
        + "/"
        + config.get("repo").unwrap()
        + "/contents/"
        + config.get("file_path").unwrap()
        + "?access_token="
        + config.get("access_token").unwrap();

    let resp = reqwest::get(url).await?.json::<GiteeResponse>().await?;

    let decoded_content_bytes = base64::decode(resp.content)?;

    let decoded_content = bytes_to_str(decoded_content_bytes);

    let tran_content: Vec<ReqContent> = serde_json::from_str(&decoded_content)?;

    Ok((tran_content, resp.sha))
}

/// 将字节码转成字符串
/// TODO 感觉这个不需要
fn bytes_to_str(bytes: Vec<u8>) -> String {
    let mut strs = String::new();

    for &a in bytes.iter() {
        if let Some(byte_char) = char::from_u32(a.into()) {
            strs.push(byte_char);
        } else {
            // FIXME 这里会不会有BUG
        }
    }
    strs
}
