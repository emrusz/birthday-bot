use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GuildId;

pub mod commands;
pub mod models;
pub mod schema;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    connection_pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN.");
    let test_guild_id: Option<String> = std::env::var("TEST_GUILD_ID").ok();
    let gateway_intents = serenity::GatewayIntents::non_privileged();

    let pg_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL."),
    );
    let async_connection_pool = Pool::builder(pg_config)
        .build()
        .expect("Failed to build database pool.");

    let client_framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::select::select(),
                commands::guild::guild_register(),
                commands::member::birthday_register(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                if test_guild_id.is_some() {
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        GuildId::new(
                            test_guild_id
                                .expect("Could not unwrap TEST_GUILD_ID")
                                .parse()?,
                        ),
                    )
                    .await?;
                }
                ctx.set_activity(Some(serenity::gateway::ActivityData::custom(
                    "🎂 Celebrating birthdays!",
                )));
                Ok(Data {
                    connection_pool: async_connection_pool,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, gateway_intents)
        .framework(client_framework)
        .await;
    client.unwrap().start().await.unwrap();
}
