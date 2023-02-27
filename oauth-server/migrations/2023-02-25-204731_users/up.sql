-- Your SQL goes here

CREATE TABLE users (
    id serial NOT NULL,
    email character varying NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);