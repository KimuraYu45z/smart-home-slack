use websocket::{ClientBuilder, Message};


pub async fn get_websocket_url(slack_token: String) -> serde_json::Value {
    let client = reqwest::Client::new();
    let response = client
        .post("https://slack.com/api/apps.connections.open")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", slack_token),
        )
        .send()
        .await.unwrap();
    
    let responsetext= response.text().await.unwrap();
    println!("{}",responsetext);
    let responsejson:serde_json::Value=serde_json::from_str(&responsetext).unwrap();
    
    return responsejson
}


pub fn ws_connect(websocket_url: &str)->(){
    let mut client = ClientBuilder::new(websocket_url)
        .unwrap()
        .connect_secure(None)
        .unwrap();
    let msg=client.recv_message().unwrap();
    println!("{:?}",msg);
}