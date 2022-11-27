use std::sync::Arc;

use poise::serenity_prelude::{self as serenity, RwLock};

mod config;
use config::Config;

pub mod deck;
pub mod prompt_deck;
pub use prompt_deck::*;

pub mod bot_data;
pub use bot_data::*;

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Testing Testing");
    let config = Config::load();
    let token = config.token;
    let prompt_file_location = config.prompt_file_location;

    let mut commands = PromptDeck::get_commands();
    commands.push(register());

    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: commands,
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Arc::new(RwLock::new(Data {
                    deck_location: prompt_file_location,
                    ..Default::default()
                })))
            })
        });

    framework
        .run()
        .await
        .expect("There was a problem running the framework");
}
