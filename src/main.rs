use std::env;

use prompt_deck::prompt_deck_commands::PROMPTDECKGROUP_GROUP;
use serenity::{async_trait, framework::StandardFramework, model::prelude::*, prelude::*, Client};

mod config;
use config::Config;

pub mod deck;
pub mod prompt_deck;
pub use prompt_deck::*;

pub use prompt_deck::prompt_deck_commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // async fn message(&self, context: Context, msg: Message) {
    //     unimplemented!();
    // }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("p>"))
        .group(&PROMPTDECKGROUP_GROUP);

    let config = Config::load();
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
