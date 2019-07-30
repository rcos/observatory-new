-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- User's real name
    real_name TEXT NOT NULL,
    -- User's online chat handle
    handle TEXT NOT NULL UNIQUE,
    -- User's email address
    email TEXT NOT NULL UNIQUE,
    -- The hash of the user's password
    password_hash TEXT NOT NULL,
    --- The salt used for the password
    salt TEXT NOT NULL,
    --- The user's bio
    bio TEXT NOT NULL,
    -- Is the user active?
    active BOOLEAN NOT NULL DEFAULT 1,
    -- SQLite stores dates as UNIX time
    joined_on DATETIME NOT NULL DEFAULT (datetime('now','localtime')),
    -- Priveledge tier
    -- 0 normal member
    -- 1 mentor
    -- 2 coordinator
    -- 3 the special Admin user
    tier INTEGER NOT NULL DEFAULT 0,
    -- User's Matermost handle
    mmost TEXT NOT NULL UNIQUE
);

-- Create a special admin user that cannot be logged into
INSERT INTO users (id, real_name, handle, email, password_hash, salt, bio, active, joined_on, tier, mmost)
VALUES (0, "Admin", "admin", "admin@rcos.io", "", "", "The Admin account for Observatory", 1, 0, 3, "admin");