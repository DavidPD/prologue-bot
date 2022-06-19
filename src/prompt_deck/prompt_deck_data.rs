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
    pub fn start_session() {
        todo!();
    }

    pub fn end_session(&mut self) {
        if self.current_session.is_some() {
            if self.last_session.is_some() {
                let last_session = self.last_session.take();
                match last_session {
                    Some(last_session) => {
                        let name = last_session.name.clone();
                        self.previous_sessions.insert(name, last_session);
                    }
                    None => {}
                }
            }

            self.last_session = self.current_session.take();
        }

        todo!();
    }
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
