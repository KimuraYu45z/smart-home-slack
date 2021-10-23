use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Serialize)]
pub struct Config {
    slack_token: String,
}

impl Config {
    pub fn read() -> Result<Config, Box<dyn Error>> {
        let path = format!("{}/.smart-home-slack/config.json", std::env::var("HOME")?);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader::<_, Config>(reader)?;

        Ok(config)
    }

    pub fn slack_token(&self) -> &str {
        &self.slack_token
    }
}
