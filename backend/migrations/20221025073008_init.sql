-- Add migration script here
CREATE TABLE todos
(
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false
);