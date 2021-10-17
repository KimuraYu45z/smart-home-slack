extern crate tokio;
extern crate serde_json;
extern crate reqwest;
extern crate wake_on_lan;
extern crate websocket;


pub mod slack;
pub mod command;



#[tokio::main]
async fn main() {
    let token_json_path:&str="C:/Users/rq527/.smart-home-slack/token.json";

    let ws_json=slack::get_websocket_url(token_json_path).await.unwrap();
    let ws_url=ws_json["url"].as_str().unwrap();

    slack::slack_run(ws_url);


}

