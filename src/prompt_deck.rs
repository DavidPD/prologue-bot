use std::{fs, path::Path, sync::Arc, vec};

use poise::{
    serenity_prelude::{autocomplete, CreateAutocompleteResponse},
    AutocompleteChoice, BoxFuture, SlashArgError,
};
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
        let mut data_write = ctx.data().write().await;

        let result = data_write.prompt_deck.draw_prompt();

        let response = match result {
            Some(prompt) => prompt,
            None => "You're all out of cards, use `/add_deck` to add more".into(),
        };

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
        #[autocomplete = "Self::test"]
        #[description = "deck name"]
        deck_name: Option<String>,
    ) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;
        let result = data_write.prompt_deck.start_session(name.as_str());

        match result {
            Ok(message) => ctx.say(message).await?,
            Err(message) => ctx.say(message).await?,
        };

        Ok(())
    }

    async fn test(ctx: Context<'_>, name: String) -> Vec<AutocompleteChoice<String>> {
        return vec![AutocompleteChoice {
            name: "test".into(),
            value: "test".into(),
        }];
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
    async fn add_deck(
        ctx: Context<'_>,
        #[description = "The name of the deck to add"] name: String,
    ) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;

        let file_name = format!("{name}.md");

        let mut deck_path = Path::new(data_write.deck_location.as_str()).to_path_buf();
        deck_path.push(file_name);

        let result = data_write.prompt_deck.add_deck(deck_path.to_str().unwrap());

        match result {
            Ok(_) => ctx.say(format!("Loaded deck {name}")).await?,
            Err(_) => {
                ctx.say(format!("There was an error loading deck {name}"))
                    .await?
            }
        };

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn export_prompt_answers(ctx: Context<'_>) -> Result<(), Error> {
        todo!();
    }

    pub fn get_commands() -> Vec<poise::Command<Arc<RwLock<Data>>, Error>> {
        vec![
            Self::list_decks(),
            Self::start_prompt_session(),
            Self::end_prompt_session(),
            Self::add_deck(),
            Self::draw(),
        ]
    }
}
