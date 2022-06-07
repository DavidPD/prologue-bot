use std::sync::Arc;

use tap::Tap;
use tokio::sync::RwLock;

use poise::serenity_prelude as serenity;

pub mod prompt_deck_commands;

use crate::bot_data::*;

pub struct PromptDeck {}

impl PromptDeck {
    /// Draw a prompt card from the current deck.
    #[poise::command(slash_command)]
    async fn draw(ctx: Context<'_>) -> Result<(), Error> {
        let count = {
            let mut data_write = ctx.data().write().await;
            data_write
                .prompt_deck
                .number_of_cards_drawn
                .tap(move |_| data_write.prompt_deck.number_of_cards_drawn += 1)
        };

        let response = format!("It's the Ace of tests! Which you've drawn {} times", count);
        ctx.say(response).await?;
        Ok(())
    }

    pub fn get_commands() -> Vec<poise::Command<Arc<RwLock<Data>>, Error>> {
        vec![Self::draw()]
    }
}

#[derive(Default)]
pub struct PromptDeckData {
    number_of_cards_drawn: i32,
}
