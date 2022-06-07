use crate::PromptDeckData;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<RwLock<Data>>, Error>;

#[derive(Default)]
pub struct Data {
    pub deck_location: String,
    pub prompt_deck: PromptDeckData,
}
