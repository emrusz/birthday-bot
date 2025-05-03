use crate::schema::member::dsl::member;
use crate::schema::member::{birthday, snowflake};
use crate::{Context, Error};
use chrono::NaiveDateTime;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateEmbed;

#[poise::command(
    slash_command,
    subcommands("register", "remove", "view"),
    rename = "birthday"
)]
pub async fn birthday_register(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Displays your birthday.
#[poise::command(slash_command)]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let mut connection = ctx.data().connection_pool.get().await?;
    let member_birthday: NaiveDateTime = member
        .select(birthday)
        .filter(snowflake.eq(ctx.author().id.to_string()))
        .first(&mut connection)
        .await?;

    // todo handle record not found with embed

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Success!")
                    .description("Your birthday is in the database.")
                    .field("Birthday", member_birthday.to_string(), true)
                    .color(serenity::Color::BLURPLE),
            )
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

/// Adds your birthday to the database.
#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "birthday"] date: String,
) -> Result<(), Error> {
    let datetime = date.parse::<NaiveDateTime>()?;

    let mut connection = ctx.data().connection_pool.get().await?;
    connection
        .transaction::<_, Error, _>(|connection| {
            async move {
                insert_into(member)
                    .values((
                        snowflake.eq(ctx.author().id.to_string()),
                        birthday.eq(datetime),
                    ))
                    .on_conflict(snowflake)
                    .do_update()
                    .set(birthday.eq(datetime))
                    .execute(connection)
                    .await?;

                // todo
                //  - associate with guild
                //  - constrain date input
                //  - timezone logic

                Ok(())
            }
            .scope_boxed()
        })
        .await?; // End member transaction

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Birthday Registered")
                    .description("Birthday successfully added to the database.")
                    .field("Member", ctx.author().to_string(), true)
                    .field("Birthday", date, true)
                    .color(serenity::Color::DARK_GREEN),
            )
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

/// Removes your birthday form the database.
#[poise::command(slash_command)]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error> {
    let mut connection = ctx.data().connection_pool.get().await?;

    delete(member)
        .filter(snowflake.eq(ctx.author().id.to_string()))
        .execute(&mut connection)
        .await?;

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Birthday Removed")
                    .description("Birthday successfully removed from the database.")
                    .field("Member", ctx.author().to_string(), true)
                    .color(serenity::Color::DARK_GREEN),
            )
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
