use std::sync::Arc;

use llm_wrapper::llm::Model;

use shuttle_persist::PersistInstance;
use tokio::sync::Mutex;
pub struct AppState {
    pub model: Arc<Mutex<Box<dyn Model>>>,
    pub persist_instance: PersistInstance,
}

impl AppState {
    pub fn new(model: Arc<Mutex<Box<dyn Model>>>, persist_instance: PersistInstance) -> AppState {
        AppState {
            model,
            persist_instance,
        }
    }
}
