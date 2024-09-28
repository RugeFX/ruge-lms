-- Add migration script here
CREATE TABLE IF NOT EXISTS foo (id serial, name varchar(255) unique, number integer);
