use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, AppState, Error>;

#[derive(Serialize, Deserialize)]
pub struct AppState {
    persist_instance: PersistInstance,
}

impl AppState {
    pub fn new(persist_instance: PersistInstance) -> AppState {
        AppState { persist_instance }
    }
}
