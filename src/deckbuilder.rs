use std::{collections::VecDeque, fmt::Display, rc::Rc};

use rand::prelude::{SliceRandom, ThreadRng};
use tap::Tap;
pub trait TCardType
where
    Self: Display,
    Self: Copy,
{
}

pub struct Deck<'a, CardType: TCardType> {
    all_cards: Vec<&'a Rc<CardType>>,
    draw_pile: Vec<&'a Rc<CardType>>,
    discard_pile: Vec<&'a Rc<CardType>>,

    rng: ThreadRng,
}

impl<CardType: TCardType> Deck<'_, CardType> {
    pub fn draw_card(&mut self) -> Option<&Rc<CardType>> {
        let card = &self.draw_pile.pop()?;
        self.discard_pile.push(card);

        Some(&card)
    }

    pub fn shuffle_deck(&mut self) {
        self.discard_pile.clear();
        self.draw_pile = self.all_cards.clone().tap_mut(|v| v.shuffle(&mut self.rng));
    }
}
