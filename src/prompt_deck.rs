use std::{fs, result, sync::Arc};

use tap::Tap;
use tokio::sync::RwLock;

pub mod prompt_deck_data;
pub mod prompt_deck_loader;

pub use prompt_deck_data::*;

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

    #[poise::command(slash_command)]
    async fn list_decks(ctx: Context<'_>) -> Result<(), Error> {
        // let decks =
        let location = ctx.data().read().await.deck_location.clone();

        let mut result: Vec<String> = vec![];
        let paths = fs::read_dir(location).unwrap();
        for path in paths {
            result.push(path?.file_name().into_string().unwrap());
        }

        let response = format!("decks: \n {}", result.join("\n"));
        ctx.say(response).await?;

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn start_prompt_session(
        ctx: Context<'_>,
        #[description = "A name for your prompt session (required)"] name: String,
    ) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;
        let result = data_write.prompt_deck.start_session(name.as_str());

        match result {
            Ok(message) => ctx.say(message).await?,
            Err(message) => ctx.say(message).await?,
        };

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn end_prompt_session(ctx: Context<'_>) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;

        let result = data_write.prompt_deck.end_session();

        match result {
            Ok(message) => {
                ctx.say(format!("session ended \"{}\"", message)).await?;
            }
            Err(_) => {
                ctx.say("There is no prompt session in progress").await?;
            }
        }

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn add_deck(ctx: Context<'_>) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn export_prompt_answers(ctx: Context<'_>) -> Result<(), Error> {
        todo!();
    }

    pub fn get_commands() -> Vec<poise::Command<Arc<RwLock<Data>>, Error>> {
        vec![
            Self::draw(),
            Self::list_decks(),
            Self::start_prompt_session(),
            Self::end_prompt_session(),
        ]
    }
}
