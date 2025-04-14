use crate::models::{Guild, Member};
use crate::schema::{guild, member};
use crate::{Context, Error};
use diesel_async::RunQueryDsl;

#[poise::command(slash_command, subcommands("all_members", "all_guilds"))]
pub async fn select(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Selects all members from database.
#[poise::command(slash_command)]
pub async fn all_members(ctx: Context<'_>) -> Result<(), Error> {
    let mut conn = ctx.data().connection_pool.get().await?;
    let data: Vec<Member> = member::table.load(&mut conn).await?;

    ctx.say(format!("{:?}", data)).await?;
    Ok(())
}

/// Selects all guilds from database.
#[poise::command(slash_command)]
pub async fn all_guilds(ctx: Context<'_>) -> Result<(), Error> {
    let mut conn = ctx.data().connection_pool.get().await?;
    let data: Vec<Guild> = guild::table.load(&mut conn).await?;

    ctx.say(format!("{:?}", data)).await?;
    Ok(())
}
