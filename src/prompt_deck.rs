use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
    sync::Arc,
    vec,
};

use poise::AutocompleteChoice;
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

        let response = {
            let result = data_write.prompt_deck.draw_prompt();

            match result {
                Some(prompt) => prompt,
                None => "You're all out of cards, use `/add_deck` to add more".into(),
            }
        };

        // ctx.say(response.clone()).await?;
        ctx.send(|reply| reply.embed(|embed| embed.title(response)))
            .await?;

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn list_prompt_decks(ctx: Context<'_>) -> Result<(), Error> {
        let location = ctx.data().read().await.deck_location.clone();

        let result = get_available_decks(location);

        match result {
            Ok(result) => {
                let response = format!("decks: \n {}", result.join("\n"));
                ctx.say(response).await?;
            }
            Err(message) => {
                ctx.say(message).await?;
            }
        }

        Ok(())
    }

    #[poise::command(slash_command)]
    async fn start_prompt_session(
        ctx: Context<'_>,
        #[autocomplete = "Self::autocomplete_deck_name"]
        #[description = "starting deck name"]
        starting_deck: String,
    ) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;
        let result = data_write.prompt_deck.start_session();

        match result {
            Ok(session_message) => {
                let file_name = format!("{starting_deck}.md");

                let mut deck_path = Path::new(data_write.deck_location.as_str()).to_path_buf();
                deck_path.push(file_name);

                data_write.prompt_deck.add_deck(
                    deck_path
                        .to_str()
                        .expect("There was a problem starting session"),
                )?;

                ctx.say(format!("{session_message} with deck {starting_deck}"))
                    .await?;
            }
            Err(message) => {
                ctx.say(message).await?;
            }
        };

        Ok(())
    }

    async fn autocomplete_deck_name(
        ctx: Context<'_>,
        _name: String,
    ) -> Vec<AutocompleteChoice<String>> {
        let location = { ctx.data().read().await.deck_location.clone() };

        match get_available_decks(location) {
            Ok(deck_names) => deck_names
                .iter()
                .map(|name| AutocompleteChoice {
                    name: name.clone(),
                    value: name.clone(),
                })
                .collect(),
            Err(_) => vec![],
        }
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
    async fn add_prompt_deck(
        ctx: Context<'_>,
        #[autocomplete = "Self::autocomplete_deck_name"]
        #[description = "The name of the deck to add"]
        name: String,
    ) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;

        let file_name = format!("{name}.md");

        let mut deck_path = Path::new(data_write.deck_location.as_str()).to_path_buf();
        deck_path.push(file_name);

        let result = data_write.prompt_deck.add_deck(
            deck_path
                .to_str()
                .expect("There was an error adding a deck"),
        );

        match result {
            Ok(_) => ctx.say(format!("Loaded deck {name}")).await?,
            Err(_) => {
                ctx.say(format!("There was an error loading deck {name}"))
                    .await?
            }
        };

        Ok(())
    }

    pub fn get_commands() -> Vec<poise::Command<Arc<RwLock<Data>>, Error>> {
        vec![
            Self::list_prompt_decks(),
            Self::start_prompt_session(),
            Self::end_prompt_session(),
            Self::add_prompt_deck(),
            Self::draw(),
        ]
    }
}

fn get_deck_name(path: Result<DirEntry, io::Error>) -> Option<String> {
    Some(path.ok()?.path().file_stem()?.to_str()?.to_owned())
}

fn get_available_decks(deck_location: String) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = vec![];
    let paths = fs::read_dir(deck_location).map_err(|_| "Error reading list of files")?;

    for path in paths {
        if let Some(name) = get_deck_name(path) {
            result.push(name);
        }
    }

    Ok(result)
}
