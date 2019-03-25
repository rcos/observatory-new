-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
    -- User's real name
    real_name TEXT NOT NULL,
    -- User's online chat handle
    handle TEXT NOT NULL,
    -- User's email address
    email TEXT NOT NULL,
    -- The hash of the user's password
    password_hash TEXT NOT NULL,
    -- Is the user active?
    active BOOLEAN NOT NULL DEFAULT 1,
    -- SQLite stores dates as UNIX time
    joined_on INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Priveledge tier
    -- 0 normal member
    -- 1 mentor
    -- 2 coordinator
    -- 3 admin (Wes Turner)
    tier INTEGER NOT NULL DEFAULT 0
);

-- Create a special admin user that cannot be logged into
INSERT INTO users (id, real_name, handle, email, password_hash, active, joined_on)
VALUES (0, "Admin", "admin", "admin", "", 0, 0);