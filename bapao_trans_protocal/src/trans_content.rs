use serde::{Deserialize, Serialize};

/// 响应数据的类型

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransHead {
    pub id: String,
    /// 传输的数据类型：文件："file"；文本："string"。
    pub content_type: Option<String>,
    /// 处理进度
    pub state: String,
    /// 数据处理完成并返回的时间节点的时间戳
    pub timestamp: i64,
}

/// 传输协议接受请求的数据类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqContent {
    pub head: TransHead,
    pub body: String,
}

pub struct ResFileContent {
    pub head: TransHead,
    pub body: Vec<u8>,
}

pub type ResStringContent = ReqContent;

/// 传输协议响应内容类型
pub enum ResContentType {
    String(ResStringContent),
    File(ResFileContent),
}

/// 传输单元 类型
pub enum TransUnitType {
    String(String),
    File(Vec<u8>),
}
