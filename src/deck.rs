use std::fmt::Display;

use rand::prelude::SliceRandom;
use tap::Tap;
pub trait TCardType
where
    Self: Display,
    Self: Clone,
{
}

#[derive(Default)]
pub struct Deck<CardType: TCardType> {
    pub all_cards: Vec<CardType>,
    draw_pile: Vec<CardType>,
    discard_pile: Vec<CardType>,
}

impl<CardType: TCardType> Deck<CardType> {
    pub fn new(all_cards: Vec<CardType>) -> Self {
        Self {
            all_cards,
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    pub fn draw_card(&mut self) -> Option<CardType> {
        let card = self.draw_pile.pop()?;
        self.discard_pile.push(card.clone());

        Some(card.clone())
    }

    pub fn shuffle_deck(&mut self) {
        self.discard_pile.clear();
        self.draw_pile = self
            .all_cards
            .clone()
            .tap_mut(|v| v.shuffle(&mut rand::thread_rng()));
    }

    pub fn shuffle_in_deck(&mut self, deck: Deck<CardType>) {
        self.all_cards.append(deck.all_cards.clone().as_mut());
    }
}

// TODO: Write tests for deck datatype
