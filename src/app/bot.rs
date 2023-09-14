use std::sync::Arc;

use ::serenity::prelude::GatewayIntents;
use anyhow::Context as _;
use llm_wrapper::llm::ModelParameters;
use poise::{serenity_prelude::Activity, Framework, PrefixFrameworkOptions};
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use songbird::SerenityInit;
use tokio::sync::Mutex;

use crate::app::app_state::AppState;
use crate::app::Error;
use crate::commands::{help, infer, inspire, ping, say};

pub struct DiscordBot;

impl DiscordBot {
    pub async fn create(
        secret_store: SecretStore,
        persist: PersistInstance,
    ) -> Result<Arc<Framework<AppState, Error>>, anyhow::Error> {
        // Get the discord token set in `Secrets.toml`
        let discord_token = secret_store
            .get("DISCORD_TOKEN")
            .context("'DISCORD_TOKEN' was not found")?;

        let model = llm_wrapper::llm::load_dynamic(
            Some(llm_wrapper::llm::ModelArchitecture::Gpt2),
            std::path::Path::new("models/gpt-2-345M/ggml-model-f16.bin"),
            llm_wrapper::llm::TokenizerSource::HuggingFaceTokenizerFile(
                std::path::Path::new("models/gpt-2-345M/tokenizer.json").to_path_buf(),
            ),
            ModelParameters::default(),
            llm_wrapper::llm::load_progress_callback_stdout,
        )
        .expect("Failed to load model...");
        let model = Arc::new(Mutex::new(model));

        let app_state = AppState::new(model, persist);

        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![ping(), say(), inspire(), infer(), help()],
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
            .client_settings(|client| client.register_songbird())
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

        Ok(framework)
    }
}
