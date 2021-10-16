
use std::fs::File;
use std::io::BufReader;
use websocket::{ClientBuilder, Message, ws::dataframe::DataFrame};


pub async fn get_websocket_url(token_json_path: &str) -> Result<serde_json::Value,()> {
    let file = File::open(token_json_path).unwrap();
    let reader = BufReader::new(file);
    let token_json:serde_json::Value = serde_json::from_reader(reader).unwrap();
    let slack_token=token_json["xapp"].as_str().unwrap();
    let client = reqwest::Client::new();
    let response = client
        .post("https://slack.com/api/apps.connections.open")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", slack_token),
        )
        .send()
        .await.unwrap();
    
    let response_text= response.text().await.unwrap();
    println!("{}",response_text);
    let response_json:serde_json::Value=serde_json::from_str(&response_text).unwrap();

    if response_json["ok"].as_bool().unwrap(){
        Ok(response_json)
    }else{
        Err(println!("Couln't get ws url."))
    }
}

pub fn slack_run(ws_url:&str){
    let mut client = ClientBuilder::new(ws_url)
        .unwrap()
        .connect_secure(None)
        .unwrap();
    let mut connection=true;
    while connection{
        match client.recv_message(){
            Ok(msg)=>{
                if msg.is_ping(){
                    println!("receive : {:?}",msg);
                    let mut reply=Message::from(msg);
                    reply.into_pong().unwrap();
                    client.send_message(&reply).unwrap();
                    println!("send : {:?}",reply);
                }
                else if msg.is_close(){
                    connection=false;
                }
                else{
                    let msg_:String=String::from_utf8(msg.take_payload()).unwrap();
                    println!("receive : {:?}",msg_);
                    let msg_json:serde_json::Value=serde_json::from_str(&msg_).unwrap();
                    if msg_json["envelope_id"].as_str()!=None{
                        let reply=Message::text(msg_);
                        client.send_message(&reply).unwrap();
                    }
                }
            }
            Err(error)=>{
                println!("{:?}",error);
                connection=false;
            }
        }
        
    }
}