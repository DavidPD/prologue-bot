use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{deck::Deck, PromptCard};

pub struct PromptDeckLoader {}

impl PromptDeckLoader {
    pub fn load_deck(path: &str) -> Deck<PromptCard> {
        let reader = BufReader::new(File::open(path).expect("cannot open deck file"));

        let mut lines = reader.lines().into_iter().map(|line| line.unwrap());

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

        return Deck::<PromptCard>::new(cards);
    }
}

#[test]
fn test_load_deck() {
    let deck = PromptDeckLoader::load_deck("Assets/Prompt Files/test_deck.md");

    assert_eq!(deck.all_cards.iter().count(), 3);
    assert_eq!(deck.all_cards.first().unwrap().prompt.lines().count(), 1);
    assert_eq!(deck.all_cards.last().unwrap().prompt.lines().count(), 3);
}
