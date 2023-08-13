mod app;
mod commands;
mod quotes;

use ::serenity::prelude::GatewayIntents;
use anyhow::Context as _;
use poise::{serenity_prelude::Activity, PrefixFrameworkOptions};
use shuttle_persist::PersistInstance;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

use app::{AppState, Error};
use commands::{help, inspire, ping, say};

// TODO : Maybe refactor this
#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> ShuttlePoise<AppState, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let app_state = AppState::new(persist);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), say(), inspire(), help()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                )),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(
            GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::non_privileged(),
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(Activity::playing("with Remi")).await;

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(app_state)
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
