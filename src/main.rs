// mod quotes;

use anyhow::Context as _;
use poise::{
    serenity_prelude::{self as serenity, Activity},
    PrefixFrameworkOptions,
};
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

// TODO : Extract this into different file
struct AppState {
    // pub quotes: Vec<crate::quotes::Quote>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { /*quotes: vec![]*/ }
    }
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, AppState, Error>;

/// Responds with "pong!"
#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}

/// Say something
#[poise::command(slash_command, prefix_command)]
async fn say(
    ctx: Context<'_>,
    #[description = "What kind of thing should i say?"] text: String,
) -> Result<(), Error> {
    ctx.say(text).await?;
    Ok(())
}

async fn inspire(ctx: Context<'_>) -> Result<(), Error> {
    // TODO : Implement this
    Ok(())
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<AppState, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), say()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("$".to_owned()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(Activity::playing("Improving my self"))
                    .await;

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(AppState {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
