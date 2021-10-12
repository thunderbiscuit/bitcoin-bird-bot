use std::fs::File;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_secret: String,
    pub access_token: String,
    pub access_secret: String,
}

impl Config {
    pub fn load() -> Self {
        let mut file = File::open("./bot.conf").unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let config: Config = hocon::de::from_str(&buff).unwrap();
        return config;
    }
}
