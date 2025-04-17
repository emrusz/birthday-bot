create table guild
(
    id            int primary key generated always as identity,
    snowflake varchar(20) not null unique,
    birthday_role varchar(20) not null
);

create table member
(
    id        int primary key generated always as identity,
    snowflake varchar(20) not null unique,
    birthday  timestamptz not null
);

create table member_guild
(
    id        int primary key generated always as identity,
    member_id int references member (id) on delete cascade not null,
    guild_id  int references guild (id) on delete cascade  not null
);
