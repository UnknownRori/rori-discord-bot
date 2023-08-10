pub struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, AppState, Error>;
