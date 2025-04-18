-- Create the server table
CREATE TABLE IF NOT EXISTS server (
    name VARCHAR(50) NOT NULL,
    motd VARCHAR(200)
);

-- Insert server record using defaults
INSERT INTO server (name, motd) VALUES ("Quoridor", "Have a nice day!");

-- Create the user table
CREATE TABLE IF NOT EXISTS user (
    user_id INTEGER PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(240) NOT NULL,
    password_date DATE DEFAULT NOW NOT NULL,
    password_count INTEGER DEFAULT 0,
    validation_code INTEGER NOT NULL DEFAULT 0,
    is_admin BOOLEAN DEFAULT FALSE NOT NULL,
    is_valid BOOLEAN DEFAULT FALSE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL
);

-- Create indexes on the user table
CREATE UNIQUE INDEX user_name ON user(name);
CREATE UNIQUE INDEX user_email ON user(email);
