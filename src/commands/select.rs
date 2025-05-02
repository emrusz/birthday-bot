use crate::models::{Guild, Member};
use crate::schema::member::birthday;
use crate::schema::{guild, member};
use crate::{Context, Error};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    owners_only,
    hide_in_help,
    subcommands("all_members", "all_guilds", "member")
)]
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

#[poise::command(slash_command)]
pub async fn member(
    ctx: Context<'_>,
    #[description = "Member"] member: serenity::Member,
) -> Result<(), Error> {
    let mut connection = ctx.data().connection_pool.get().await?;
    let member_birthday: NaiveDateTime = member::table
        .select(birthday)
        .filter(member::snowflake.eq(member.user.id.to_string()))
        .first(&mut connection)
        .await?;

    ctx.say(format!("{:?}", member_birthday)).await?;
    Ok(())
}
