use chrono::{Duration, TimeZone, Utc};

use crate::trans_content::ReqContent;

/// Removes expired requests from the content list.
/// 
/// Filters out requests that are older than 30 minutes to prevent
/// processing of stale requests and to keep the communication channel clean.
/// 
/// # Parameters
/// 
/// * `contents` - Vector of request content to filter
/// 
/// # Returns
/// 
/// `Vec<ReqContent>` - Filtered vector containing only non-expired requests
/// 
/// # Expiration Logic
/// 
/// A request is considered expired if:
/// `current_time - request_timestamp > 30 minutes`
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_trans_protocal::{utils::trim_expired_data, trans_content::*};
/// 
/// let all_requests = vec![
///     // Mix of recent and old requests
/// ];
/// 
/// let active_requests = trim_expired_data(all_requests);
/// println!("Filtered to {} active requests", active_requests.len());
/// ```
/// 
/// # Performance
/// 
/// This function operates in O(n) time where n is the number of requests.
/// It's called automatically by the transport layer to maintain system hygiene.
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
