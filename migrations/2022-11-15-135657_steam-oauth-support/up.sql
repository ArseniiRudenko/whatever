-- Your SQL goes here

alter table users drop constraint users_pk;

alter table users
    alter column email drop not null;

alter table users
    alter column password_hash drop not null;

alter table users
    add steam_id varchar;

alter table users
    add id integer generated always as identity;

alter table users
    add email_verified bool default false not null;

create unique index users_steam_id_uindex
    on users (steam_id);

alter table users
    add constraint users_pk
        primary key (id);

alter table users
   add constraint some_id_is_set
        check (email is not null or steam_id is not null );

