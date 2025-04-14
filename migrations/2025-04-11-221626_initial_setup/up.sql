create table guild
(
    id            int primary key generated always as identity,
    snowflake     varchar(20) not null,
    birthday_role varchar(20) not null
);

create table member
(
    id        int primary key generated always as identity,
    snowflake varchar(20) not null,
    birthday  timestamptz not null
);

create table member_guild
(
    id        int primary key generated always as identity,
    member_id int references member (id) not null,
    guild_id  int references guild (id)  not null
);
