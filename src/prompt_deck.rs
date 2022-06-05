use poise::{_GetGenerics, serenity_prelude as serenity};

pub mod prompt_deck_commands;

use crate::poise_framework_types::*;

pub struct PromptDeck {}

impl PromptDeck {
    #[poise::command(slash_command)]
    async fn draw(
        ctx: Context<'_>,
        #[description = "Selected user"] user: Option<serenity::User>,
    ) -> Result<(), Error> {
        let u = user.as_ref().unwrap_or(ctx.author());
        let response = format!("{}'s account was created at {}", u.name, u.created_at());
        ctx.say(response).await?;
        Ok(())
    }

    pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
        vec![Self::draw()]
    }
}
