use std::{fs::File, io::Read};

use serde::Deserialize;
use serenity::{async_trait, framework::StandardFramework, model::prelude::*, prelude::*, Client};

struct Handler;

#[derive(Deserialize)]
struct Config {
    app_id: String,
    public_key: String,
    token: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().configure(|c| c.prefix("~"));

    let config = load_config();
    let token = config.token;

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't create new client");

    if let Err(why) = client.start().await {
        println!("An error occurred in client: {:?}", why);
    }
}

fn load_config() -> Config {
    let mut file = File::open("config.toml").expect("Could not open config file");

    let mut config_string = String::new();
    file.read_to_string(&mut config_string)
        .expect("error reading config file");

    let config: Config = toml::from_str(&config_string).expect("Error parsing config file");

    config
}
