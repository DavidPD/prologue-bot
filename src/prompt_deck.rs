use poise::serenity_prelude as serenity;

pub mod prompt_deck_commands;

use crate::bot_data::*;

pub struct PromptDeck {}

impl PromptDeck {
    /// Draw a prompt card from the current deck.
    #[poise::command(slash_command)]
    async fn draw(ctx: Context<'_>) -> Result<(), Error> {
        let response = format!("It's the Ace of tests!");
        ctx.say(response).await?;
        Ok(())
    }

    pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
        vec![Self::draw()]
    }
}
