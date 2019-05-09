-- Your SQL goes here
CREATE TABLE relation_group_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- ID of the group
    group_id INTEGER NOT NULL,
    -- ID of the user that is a member of the group
    user_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);