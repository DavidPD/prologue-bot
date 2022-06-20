use std::{collections::HashMap, fmt::Display};

use crate::{
    deck::{Deck, TCardType},
    prompt_deck_loader::PromptDeckLoader,
};

#[derive(Default)]
pub struct PromptDeckData {
    pub deck: Deck<PromptCard>,
    pub current_session: Option<PromptDeckSession>,
    pub last_session: Option<PromptDeckSession>,
    pub previous_sessions: HashMap<String, PromptDeckSession>,
}

impl PromptDeckData {
    pub fn start_session(&mut self, name: &str) -> Result<String, String> {
        if self.current_session.is_some() {
            return Err(
                "There is already a session in progress, use `/end_prompt_session` to end it"
                    .into(),
            );
        }

        self.current_session = Some(PromptDeckSession::new(name));

        Ok(format!("Started a new session \"{}\"", name))
    }

    pub fn end_session(&mut self) -> Result<String, ()> {
        match self.current_session.take() {
            Some(current_session) => {
                match self.last_session.take() {
                    Some(last_session) => {
                        let name = last_session.name.clone();
                        self.previous_sessions.insert(name.clone(), last_session);
                    }
                    None => (),
                };

                let name = current_session.name.clone();
                self.last_session = Some(current_session);

                Ok(name)
            }
            None => Err(()),
        }
    }

    pub fn add_deck(&mut self, deck_name: impl Into<String>) -> Result<String, String> {
        let deck_name = deck_name.into();
        self.current_session = match self.current_session.take() {
            Some(current_session) => {
                let new_deck = PromptDeckLoader::load_deck(deck_name.clone());
                self.deck.shuffle_in_deck(new_deck.unwrap());

                Some(current_session)
            }
            None => {
                return Err(
                    "There is no session in progress, start one with `/start_prompt_session`"
                        .into(),
                )
            }
        };

        Ok(deck_name)
    }

    /// Returns None when out of cards
    pub fn draw_prompt(&mut self) -> Option<String> {
        let card = self.deck.draw_card();

        card.map(|card| card.prompt)
    }
}

#[derive(Default)]
pub struct PromptDeckSession {
    pub name: String,
    pub deck: Deck<PromptCard>,
}

impl PromptDeckSession {
    pub fn new(name: &str) -> PromptDeckSession {
        PromptDeckSession {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn add_deck(&mut self, deck: Deck<PromptCard>) {
        self.deck.shuffle_in_deck(deck);
    }
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
