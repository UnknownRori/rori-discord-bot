mod app;
mod commands;
mod llm;
mod quotes;

use app::{app_state::AppState, bot::DiscordBot, Error};
use shuttle_persist::PersistInstance;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> ShuttlePoise<AppState, Error> {
    let bot = DiscordBot::create(secret_store, persist).await?;
    Ok(bot.into())
}
