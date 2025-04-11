// @generated automatically by Diesel CLI.

diesel::table! {
    guild (id) {
        id -> Int4,
        #[max_length = 20]
        snowflake -> Varchar,
        #[max_length = 20]
        birthday_role -> Varchar,
    }
}

diesel::table! {
    member (id) {
        id -> Int4,
        #[max_length = 20]
        snowflake -> Varchar,
        birthday -> Date,
    }
}

diesel::table! {
    member_guild (id) {
        id -> Int4,
        member_id -> Int4,
        guild_id -> Int4,
    }
}

diesel::joinable!(member_guild -> guild (guild_id));
diesel::joinable!(member_guild -> member (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    guild,
    member,
    member_guild,
);
