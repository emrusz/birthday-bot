use crate::schema::guild::dsl::guild;
use crate::schema::guild::{birthday_role, snowflake};
use crate::{Context, Error};
use diesel::prelude::*;
use diesel::{delete, insert_into};
use diesel_async::RunQueryDsl;
use poise::serenity_prelude as serenity;

//todo:
//  - Update on reinsert
//  - Embeds for add and remove
//  - Correct command name

#[poise::command(
    slash_command,
    subcommands("register", "unregister"),
    required_permissions = "ADMINISTRATOR"
)]
pub async fn guild_comm(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Registers your guild with Birthday Bot
#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "You guild's birthday role"] role: serenity::RoleId,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let mut connection = ctx.data().connection_pool.get().await?;

    insert_into(guild)
        .values((
            snowflake.eq(guild_id.to_string()),
            birthday_role.eq(role.to_string()),
        ))
        .execute(&mut connection)
        .await?;

    ctx.say("added").await?;

    Ok(())
}

/// Removes your guild from Birthday Bot
#[poise::command(slash_command)]
pub async fn unregister(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let mut connection = ctx.data().connection_pool.get().await?;

    delete(guild)
        .filter(snowflake.eq(guild_id.to_string()))
        .execute(&mut connection)
        .await?;

    ctx.say("removed").await?;
    Ok(())
}
