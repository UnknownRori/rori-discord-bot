use std::sync::Arc;

use serenity::utils::MessageBuilder;

use crate::app::{Context, Error};
use crate::llm::infer_text;
use crate::quotes::QuoteAPI;

/// Responds with "pong!"
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}

/// Say something
#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "What kind of thing should i say?"] text: String,
) -> Result<(), Error> {
    ctx.say(text).await?;
    Ok(())
}

/// To inspire someone using real or historic person's quote
#[poise::command(slash_command, prefix_command)]
pub async fn inspire(ctx: Context<'_>) -> Result<(), Error> {
    match QuoteAPI::fetch().await {
        Ok(quote) => {
            let message = MessageBuilder::new()
                .push_quote_line(format!("_\"{}\"_", quote.content))
                .push_quote_line("")
                .push_quote_line(format!(" \\- *{}*", quote.author))
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

#[poise::command(slash_command, prefix_command)]
pub async fn infer(
    ctx: Context<'_>,
    #[description = "Prompt a text that should be infered"] prompt: String,
) -> Result<(), Error> {
    // ? INFO : May god have mercy on me
    ctx.defer().await?;

    match &ctx.data().model {
        Some(model) => {
            let model = Arc::clone(model);
            if let Ok(model) = model.try_lock() {
                ctx.say(infer_text(model.as_ref(), prompt).await).await?;
            } else {
                ctx.say("Model in use...").await?;
            }
            Ok::<(), Error>(())
        }
        None => {
            ctx.say("Model is currently not included in Prouduction Server")
                .await?;
            Ok(())
        }
    }?;

    Ok(())
}

/// Show what you can do with Rori Bot
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type ~help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
