use serenity::{async_trait, framework::StandardFramework, model::prelude::*, prelude::*, Client};

mod config;
use config::Config;

pub mod simple_random;
pub use simple_random::RANDOMIZERGROUP_GROUP;

pub mod deckbuilder;

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
        .configure(|c| c.prefix("~"))
        .group(&RANDOMIZERGROUP_GROUP);

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
