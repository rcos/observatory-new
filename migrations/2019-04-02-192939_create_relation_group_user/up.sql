-- Your SQL goes here
CREATE TABLE relation_group_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- ID of the group
    group_id INTEGER NOT NULL,
    -- ID of the user that is a member of the group
    member_id INTEGER NOT NULL
);