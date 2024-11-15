-- Your SQL goes here
create table users (
    id serial primary key,
    name varchar not null,
    surname varchar not null,
    age integer not null
)
