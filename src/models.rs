// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::guild)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild {
    pub id: i32,
    pub snowflake: String,
    pub birthday_role: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::member)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Member {
    pub id: i32,
    pub snowflake: String,
    pub birthday: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::member_guild)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MemberGuild {
    pub id: i32,
    pub member_id: i32,
    pub guild_id: i32,
}
