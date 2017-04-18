CREATE EXTENSION if not exists "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    nickname VARCHAR(120) UNIQUE not null,
    created_at TIMESTAMP default current_timestamp not null,
    updated_at TIMESTAMP default current_timestamp not null,
    email VARCHAR(120) UNIQUE not null,
    password_hash BYTEA not null
);
SELECT diesel_manage_updated_at('users');
