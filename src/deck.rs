use std::fmt::Display;

use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};
use tap::Tap;
pub trait TCardType
where
    Self: Display,
    Self: Copy,
{
}

pub struct Deck<CardType: TCardType> {
    all_cards: Vec<CardType>,
    draw_pile: Vec<CardType>,
    discard_pile: Vec<CardType>,

    rng: ThreadRng,
}

impl<CardType: TCardType> Deck<CardType> {
    pub fn new(all_cards: Vec<CardType>) -> Self {
        Self {
            all_cards,
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn draw_card(&mut self) -> Option<CardType> {
        let card = self.draw_pile.pop()?;
        self.discard_pile.push(card);

        Some(card)
    }

    pub fn shuffle_deck(&mut self) {
        self.discard_pile.clear();
        self.draw_pile = self.all_cards.clone().tap_mut(|v| v.shuffle(&mut self.rng));
    }
}

// TODO: Write tests for deck datatype
