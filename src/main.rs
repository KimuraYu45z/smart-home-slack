extern crate tokio;
extern crate serde_json;
extern crate reqwest;
extern crate wake_on_lan;
extern crate websocket;


use std::fs::File;
use std::io::BufReader;



pub mod slack;
pub mod wol;
pub mod command;



#[tokio::main]
async fn main() {
    let file = File::open("C:/Users/rq527/.smart-home-slack/token.json").unwrap();
    let reader = BufReader::new(file);
    let token_json:serde_json::Value = serde_json::from_reader(reader).unwrap();
    let slack_token=token_json["xapp"].as_str().unwrap();


    let ws_url=slack::get_websocket_url(slack_token.to_string()).await;
    


    if ws_url["ok"].as_bool().unwrap(){
        println!("url: {}",ws_url["url"].as_str().unwrap());
        slack::ws_connect(ws_url["url"].as_str().unwrap());
    }
    



    // let mac: &str = "18:C0:4D:94:06:7F";

    // match wol::convert_mac_address(mac) {
    //     Ok(mac_address) => {
    //         println!("Mac address : {:?}", mac_address);
    //     }
    //     Err(_) => {
    //         println!("Invalid mac address.");
    //     }
    // }
}
