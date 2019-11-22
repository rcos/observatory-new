# Introduction

This document attempts to explain the layout and structure of the project.

To begin with make sure that you have read the [../README.md]. That contains the
instructions for how to get setup to work on this project, as well as links to
the documentation of the major dependencies, which I suggest you at least read a
little of to understand what they are.

## Project Structure

There are a few major folders of the project, that I will go into more later:

- `src/` contains the Rust code that is the main part of the project.
- `templates/` contains the HTML template files.
- `static/` contains static assets like images and CSS that will be served directly.
- `migrations/` contains the SQL code that intitializes the database.
- `scripts/` contains a few helpful scripts.
- `docs/` contains, of course, the documentation including this file.

### src/

This folder contains all the Rust code of the project, and is the majority of
the project.

There are several sub-folders within this one that each handle different parts
of what Observatory does.
Each of these sub-folders follows roughly the same structure:

- `mod.rs` marks this folder as a Rust module, and exports the other files.
- `handlers.rs` Rocket handlers for each of the URL routes, and helper functions.
- `models.rs` database models that can be queried and inserted into the database.
- `templates.rs` structs holding the data for HTML templates which can be rendered.

This layout is fairly consistent between all the folders, and generally each
file is only responsible for one task, with the `handler.rs` files doing the
bulk of the actual logic.

### templates/

The templates folder has all the HTML that matches with the structs in
`templates.rs` files in the Rust code.

Templates are in the Jinja2 format and are rendered using the Askama library.
See the Askama documentation for more about that (linked in the README).

### static/

Files in the static folder are served as-is without any modifications done to
them. This is perfect for things like images, CSS, and JavaScript files which
shouldn't be modified before they're served to the user.

Anything in this folder can be found in the matching place under the `/static/`
URL route on the webserver.

### migrations/

This folder has SQL files that initialize the database. These files are broken
up into seperate migrations, which are applied sequentially. By breaking up the
database into migrations we are able to update older versions of the database to
new versions without losing data, so they're very important.

Any time you want to make a change to the database you have to create a new
migration that does it. This can either be done manually or with the Diesel CLI.
Check out the Diesel documentation for more on this (linked in the README).

### scripts/

This folder contains a few helpful scripts to do tasks like build the docker
container.

There is also the importer script which is itself a little Ruby project that
generates an SQL file based on the old Observatory database so that we can
import some data from that to the new Observatory.

### docs/

Contains a variety of documentation files in the Markdown format. No code or
anything in here, just *knowledge*.
