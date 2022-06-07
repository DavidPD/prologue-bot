use std::{fs::File, io::Read};

#[derive(serde::Deserialize)]
pub struct Config {
    pub app_id: String,
    pub public_key: String,
    pub token: String,
    pub prompt_file_location: String,
}

impl Config {
    pub fn load() -> Config {
        let mut file = File::open("config.toml").expect("Could not open config file");

        let mut config_string = String::new();
        file.read_to_string(&mut config_string)
            .expect("error reading config file");

        let config: Config = toml::from_str(&config_string).expect("Error parsing config file");

        config
    }
}
