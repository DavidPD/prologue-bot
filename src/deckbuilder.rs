pub trait TCardType: Display {}

pub struct Deck<CardType: TCardType> {}
