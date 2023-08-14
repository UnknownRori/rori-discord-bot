use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;

#[derive(Serialize, Deserialize)]
pub struct AppState {
    persist_instance: PersistInstance,
}

impl AppState {
    pub fn new(persist_instance: PersistInstance) -> AppState {
        AppState { persist_instance }
    }
}
