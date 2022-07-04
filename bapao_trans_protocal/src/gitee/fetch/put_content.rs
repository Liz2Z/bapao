use super::{http, utils};
use base64;
use std::collections::HashMap;
use std::error::Error;

/// 将数据更新至 gitee 上的 io 文件
pub async fn put_content(content: String, sha: String) -> Result<(), Box<dyn Error>> {
    let config: HashMap<String, String> = utils::read_config()?;

    let url = String::from("https://gitee.com/api/v5/repos/")
        + config.get("user_name").unwrap()
        + "/"
        + config.get("repo").unwrap()
        + "/contents/"
        + config.get("file_path").unwrap();

    let mut data = HashMap::new();
    let content_str = base64::encode(content);

    let token: &str = config.get("access_token").unwrap();

    data.insert("access_token", token);
    data.insert("sha", &sha);
    data.insert("message", "response");
    data.insert("content", &content_str);

    let resp = http::put(&url, &data).await?;

    if resp.status() != 200 {
        let err_msg: String = resp
            .text()
            .await
            .unwrap_or_else(|err| String::from(err.to_string()));

        let err = Box::<dyn Error>::from(err_msg);

        return Err(err);
    }

    Ok(())
}
