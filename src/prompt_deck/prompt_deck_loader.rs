use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::{deck::Deck, PromptCard};

pub struct PromptDeckLoader {}

impl PromptDeckLoader {
    pub fn load_deck(path: &str) -> Result<Deck<PromptCard>, String> {
        let reader = open_file(path)?;

        // magic: .collect() can transform an iterator of Result<T, E> into a Result<Vec<T>, E>!
        let mut lines = match reader.lines().collect::<Result<Vec<String>, io::Error>>() {
            Ok(lines) => lines.into_iter(),
            Err(_) => {
                return Err("Error reading file".into());
            }
        };

        let mut cards: Vec<PromptCard> = vec![];

        loop {
            let card_lines = lines.by_ref().take_while(|line| line.trim() != "---");

            let card_string = card_lines
                .collect::<Vec<String>>()
                .join("\n")
                .trim()
                .to_owned();

            if card_string.clone().len() == 0 {
                break;
            }

            cards.push(PromptCard {
                prompt: card_string,
            })
        }

        return Ok(Deck::<PromptCard>::new(cards));
    }
}

fn open_file(path: &str) -> Result<std::io::BufReader<std::fs::File>, &'static str> {
    let file = File::open(path);

    match file {
        Ok(file) => Ok(BufReader::new(file)),
        Err(_) => Err("Error finding prompt deck, use `/list_decks` to see all valid decks"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_deck() {
        let deck = PromptDeckLoader::load_deck("Assets/Prompt Files/test_deck.md")
            .expect("Error opening file");

        assert_eq!(deck.all_cards.iter().count(), 3);
        assert_eq!(deck.all_cards.first().unwrap().prompt.lines().count(), 1);
        assert_eq!(deck.all_cards.last().unwrap().prompt.lines().count(), 3);
    }
}
