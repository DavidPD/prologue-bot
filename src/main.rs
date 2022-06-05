use poise::serenity_prelude as serenity;

mod config;
use config::Config;

pub mod deck;
pub mod prompt_deck;
pub use prompt_deck::*;

pub mod poise_framework_types;
pub use poise_framework_types::*;

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

    let commands = PromptDeck::get_commands();

    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![register()],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, ready, _framework| {
            let guild_ids = ready.guilds.iter().map(|g| g.id);
            Box::pin(async move { Ok(Data {}) })
        });

    framework.run().await.unwrap();
}
