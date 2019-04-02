-- Your SQL goes here
CREATE TABLE meetings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
    -- The datetime in UNIX time
    happened_on DATE NOT NULL DEFAULT (datetime('now','localtime')),
    -- The attendance code of the meeting
    code TEXT NOT NULL UNIQUE,
    -- The ID of the group the meeting was for
    group_id INTEGER NOT NULL DEFAULT 0,
    -- The ID of the user who hosted the meeting
    hosted_by INTEGER NOT NULL DEFAULT 0
);