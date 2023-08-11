use serenity::utils::MessageBuilder;

use crate::app::{Context, Error};
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
#[poise::command(slash_command)]
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
