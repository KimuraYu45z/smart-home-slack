use ws::{connect, CloseCode};

pub async fn get_websocket_url(slack_token: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://slack.com/api/apps.connections.open")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", slack_token),
        )
        .send()
        .await?;

    todo!()
}

pub async fn start(websocket_url: String) {
    connect(websocket_url, |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    })
    .unwrap()
}
