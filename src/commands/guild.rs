use crate::schema::guild::dsl::guild;
use crate::schema::guild::{birthday_role, snowflake};
use crate::{Context, Error};
use diesel::prelude::*;
use diesel::{delete, insert_into};
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::CreateEmbed;
use poise::{CreateReply, serenity_prelude as serenity};

#[poise::command(
    slash_command,
    subcommands("register", "unregister"),
    required_permissions = "ADMINISTRATOR",
    rename = "guild"
)]
pub async fn guild_register(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Registers your guild with Birthday Bot.
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
        .on_conflict(snowflake)
        .do_update()
        .set(birthday_role.eq(role.to_string()))
        .execute(&mut connection)
        .await?;

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Guild Registered")
                    .description("Guild successfully added to the database.")
                    .field("Guild ID", format!("{}", guild_id.to_string()), true)
                    .field("Role ID", format!("{}", role.to_string()), true)
                    .color(serenity::Color::DARK_GREEN),
            )
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

/// Removes your guild from Birthday Bot.
#[poise::command(slash_command)]
pub async fn unregister(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let mut connection = ctx.data().connection_pool.get().await?;

    delete(guild)
        .filter(snowflake.eq(guild_id.to_string()))
        .execute(&mut connection)
        .await?;

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Guild Unregistered")
                    .description("Guild successfully removed from the database.")
                    .field("Guild ID", format!("{}", guild_id.to_string()), true)
                    .color(serenity::Color::DARK_GREEN),
            )
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
