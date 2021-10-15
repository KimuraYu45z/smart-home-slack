use ws::{connect, CloseCode};
use websocket::{ClientBuilder, OwnedMessage};

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
    let responsejson:serde_json::Value=serde_json::from_str(&responsetext).unwrap();
    return responsejson
}

pub async fn start(websocket_url: String) {
    connect(websocket_url, |out| {
        out.send("Hello WebSocket").unwrap();
        
        move |msg| {
            println!("receive");
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    })
    .unwrap()
}

pub fn start2(websocket_url: String)->OwnedMessage{
    let client = ClientBuilder::new(&websocket_url).unwrap().connect_insecure().unwrap();
    let (mut receiver, sender) = client.split().unwrap();
    let msg=receiver.recv_message().unwrap();
    return msg
}