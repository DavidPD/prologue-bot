use std::{collections::HashMap, fmt::Display};

use crate::deck::{Deck, TCardType};

#[derive(Default)]
pub struct PromptDeckData {
    pub number_of_cards_drawn: i32,
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
}

pub struct PromptDeckSession {
    pub name: String,
}

impl PromptDeckSession {
    pub fn new(name: &str) -> PromptDeckSession {
        PromptDeckSession { name: name.into() }
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
