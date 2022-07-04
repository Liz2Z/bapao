use chrono::{Duration, TimeZone, Utc};

use crate::trans_content::ReqContent;
// use std::time::{Duration, SystemTime};

/// 剔除已完成且过期（超过30min的）的数据
pub fn trim_expired_data(contents: Vec<ReqContent>) -> Vec<ReqContent> {
    contents
        .into_iter()
        .filter(|item| {
            // start + exp > now  === 过期
            // start > now - exp  === 过期
            // now - exp < start  === 过期
            // limit = now - exp;
            // limit.lt(start)    === 过期

            // 过期时间
            let duration = Duration::minutes(30);

            let limit_time_stamp = Utc::now().checked_sub_signed(duration);

            let start_time_stamp = Utc.timestamp_millis(item.head.timestamp);

            return limit_time_stamp.lt(&Option::Some(start_time_stamp));
        })
        .collect()
}
