-- Your SQL goes here
CREATE TABLE meetings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
    -- The datetime in UNIX time
    datetime INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- The attendance code of the meeting
    code TEXT NOT NULL,
    -- The ID of the group the meeting was for
    group_id INTEGER NOT NULL DEFAULT 0
);