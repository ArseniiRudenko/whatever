-- This file should undo anything in `up.sql`

alter table users
    drop constraint some_id_is_set;

drop index users_steam_id_uindex;

alter table users drop constraint users_pk;


alter table users
    drop column if exists  steam_id;

alter table users
    drop column if exists  id;

alter table users
    drop column if exists  email_verified;


alter table users
    alter column email set not null;

alter table users
    alter column password_hash set not null;


alter table users
    add constraint users_pk
        primary key (email);


