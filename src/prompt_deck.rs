use std::{collections::HashMap, fmt::Display, fs, sync::Arc};

use tap::Tap;
use tokio::sync::RwLock;

pub mod prompt_deck_loader;

use crate::{
    bot_data::*,
    deck::{Deck, TCardType},
};

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
    async fn start_prompt_session(ctx: Context<'_>) -> Result<(), Error> {
        todo!();
    }

    #[poise::command(slash_command)]
    async fn end_prompt_session(ctx: Context<'_>) -> Result<(), Error> {
        let mut data_write = ctx.data().write().await;

        if data_write.prompt_deck.current_session.is_some() {
            if data_write.prompt_deck.last_session.is_some() {
                let last_session = data_write.prompt_deck.last_session.take();
                match last_session {
                    Some(last_session) => {
                        let name = last_session.name.clone();
                        data_write
                            .prompt_deck
                            .previous_sessions
                            .insert(name, last_session);
                    }
                    None => {}
                }
            }

            data_write.prompt_deck.last_session = data_write.prompt_deck.current_session.take();
        }

        todo!();
    }

    #[poise::command(slash_command)]
    async fn add_deck(ctx: Context<'_>) -> Result<(), Error> {
        todo!();
    }

    #[poise::command(slash_command)]
    async fn export_prompt_answers(ctx: Context<'_>) -> Result<(), Error> {
        todo!();
    }

    pub fn get_commands() -> Vec<poise::Command<Arc<RwLock<Data>>, Error>> {
        vec![Self::draw(), Self::list_decks()]
    }
}

#[derive(Default)]
pub struct PromptDeckData {
    pub number_of_cards_drawn: i32,
    pub deck: Deck<PromptCard>,
    pub current_session: Option<PromptDeckSession>,
    pub last_session: Option<PromptDeckSession>,
    pub previous_sessions: HashMap<String, PromptDeckSession>,
}

pub struct PromptDeckSession {
    pub name: String,
}

#[derive(Clone, Default)]
pub struct PromptCard {
    pub prompt: String,
}

impl Display for PromptCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.prompt.as_str())
            .expect("error writing output");
        Ok(())
    }
}

impl TCardType for PromptCard {}
