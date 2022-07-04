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

pub struct BtpListener {
    done: Vec<ResStringContent>,
    files: HashMap<String, Vec<u8>>,
}

impl BtpListener {
    pub fn new() -> Self {
        BtpListener {
            done: vec![],
            files: HashMap::new(),
        }
    }

    /// 获取数据
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

    /// 暂存数据，但是不发送
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
