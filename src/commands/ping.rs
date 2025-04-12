use crate::{Context, Error};

/// Replies with a pong.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!️ 🏓").await?;
    Ok(())
}
