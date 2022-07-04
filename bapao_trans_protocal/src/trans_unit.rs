use crate::trans_content::{
    ReqContent, ResContentType, ResFileContent, ResStringContent, TransHead, TransUnitType,
};

/// 每一个都是一个单独的请求。
///
/// 通过调用 `get` 方法拿到请求数据，调用 `set` 来设置返回数据
///
pub struct TransUnit {
    content: ReqContent,
}

impl TransUnit {
    pub fn new(content: ReqContent) -> TransUnit {
        return TransUnit { content: content };
    }

    pub fn get(&self) -> &String {
        return &self.content.body;
    }

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
