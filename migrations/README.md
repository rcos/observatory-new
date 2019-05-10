# Migrations
In this folder are the migrations used to generate the database tables
and initialize them. They are plain SQL files in the respective dialect
that each database supports. More information about migrations can be
found on [Diesel's website](https://diesel.rs)

Migrations are organized according to which database they are for.
Each database folder contains a series of folders dated and named based on
the migration with an `up.sql` and a `down.sql` file to apply and revert
the migration.