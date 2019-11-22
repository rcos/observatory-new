# How To

Explanations of basic tasks for development, since some of them are more
involved than they seem.

## Compile and run the Project

This is outlined in the [README](../README.md), basically install the proper
version of Rust and then run `cargo run` in the project folder. The server
itself will tell you where to access it from.

## Add a new Page

There are three pieces to ever page on the website: the template struct,
handler function, and HTML template.

The [Askama](https://github.com/djc/askama) and [Rocket](https://rocket.rs)
documentation offer a lot more information on what these are and how they work.

### Template Struct

Every sub-folder of `src/` as a `templates.rs` file containing the template
structs. These structs define the data that will be passed to the template when
they are rendered. The handler function returns these template structs with the
data filled in and the template is then rendered and sent to the client.

Each template struct has a number of macro flags above it, and these are important.

## Add a field to the Database

Adding a field to the database is a dangerous change, since existing databases
have to be migrated to the new version. Only do it when you have to!
