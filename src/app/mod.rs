use self::app_state::AppState;

pub mod app_state;
pub mod bot;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, AppState, Error>;
