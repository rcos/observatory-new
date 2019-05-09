-- Your SQL goes here
create TABLE news (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- Date and time of the news event
    happened_at DATETIME NOT NULL,
    -- Title of the news
    title TEXT NOT NULL,
    -- Description of the news
    description TEXT NOT NULL,
    -- The color displayed on the newsfeed
    color TEXT
);