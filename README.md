# birthday-bot

A Discord bot to celebrate birthdays!

## Environment

`.env`

```dotenv
POSTGRES_USER=database_user
POSTGRES_PASSWORD=database_password
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost/birthday_bot
DISCORD_TOKEN=discord.token
```

## Database

```mermaid
erDiagram
    member {
        varchar(20) snowflake "not null"
        timestamptz birthday "not null"
        integer id PK "generated as identity"
    }

    guild {
        varchar(20) snowflake "not null"
        varchar(20) birthday_role "not null"
        integer id PK "generated as identity"
    }

    member_guild {
        integer member_id FK "not null"
        integer guild_id FK "not null"
        integer id PK "generated as identity"
    }

    member ||--|{ member_guild: joins
    guild ||--|{ member_guild: has
```