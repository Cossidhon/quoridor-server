-- Create the user table
CREATE TABLE IF NOT EXISTS user (
    userid TEXT PRIMARY KEY NOT NULL,
    role TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);