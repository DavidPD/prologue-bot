use crate::PromptDeckData;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<RwLock<Data>>, Error>;

#[derive(Default)]
pub struct Data {
    pub prompt_deck: PromptDeckData,
}
