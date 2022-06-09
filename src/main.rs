use std::sync::Arc;

use poise::serenity_prelude::{self as serenity, RwLock};

mod config;
use config::Config;

pub mod deck;
pub mod prompt_deck;
pub use prompt_deck::*;

pub mod bot_data;
pub use bot_data::*;

pub use prompt_deck::prompt_deck_commands;

#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or(ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
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

    framework.run().await.unwrap();
}
