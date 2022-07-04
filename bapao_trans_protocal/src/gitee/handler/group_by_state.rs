use crate::trans_content::ReqContent;

#[derive(Debug)]
pub struct ContentGroupByState {
    pub pending: Vec<ReqContent>,
    pub done: Vec<ReqContent>,
}

/// 将请求数据根据数据的状态（state）做分组
pub fn group_by_state(content: Vec<ReqContent>) -> ContentGroupByState {
    let mut content_group_by_state = ContentGroupByState {
        pending: vec![],
        done: vec![],
    };

    for item in content.into_iter() {
        let state = &item.head.state[..];

        match state {
            "Pending" => content_group_by_state.pending.push(item),
            _ => content_group_by_state.done.push(item),
        }
    }

    content_group_by_state
}
