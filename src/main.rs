extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;
extern crate wake_on_lan;
extern crate websocket;

pub mod config;
pub mod slack;

use config::Config;

#[tokio::main]
async fn main() {
    let config = match Config::read() {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };
    let ws_url = slack::get_websocket_url(config.slack_token())
        .await
        .unwrap();

    slack::slack_run(&ws_url);
}
