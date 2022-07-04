use bapao_app_protocal;
use shot_pic::shot_pic;

mod shot_pic;

// use std::process;

#[tokio::main]
async fn main() {
    let mut btp_listener = bapao_app_protocal::AppListener::new();

    btp_listener.add("/monitor/pic/shot", shot_pic);

    btp_listener.listen().await;
}
