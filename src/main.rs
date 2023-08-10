mod quotes;

use ::serenity::utils::MessageBuilder;
use anyhow::Context as _;
use poise::{
    serenity_prelude::{self as serenity, Activity},
    PrefixFrameworkOptions,
};
use quotes::QuoteAPI;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

// TODO : Extract this into different file
struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {}
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

/// To inspire someone using real or historic person's quote
#[poise::command(slash_command)]
async fn inspire(ctx: Context<'_>) -> Result<(), Error> {
    match QuoteAPI::fetch().await {
        Ok(quote) => {
            let message = MessageBuilder::new()
                .push_quote_line(format!("\"{}\"", quote.content))
                .push_quote_line_safe(format!(" - {}", quote.author))
                .build();

            ctx.say(message).await?;
        }
        Err(err) => {
            tracing::error!("{:#?}", err);
            ctx.say("Failed to fetch quote").await?;
        }
    }

    Ok(())
}

#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttlePoise<AppState, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let mut app_state = AppState::default();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), say(), inspire()],
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
                Ok(app_state)
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
