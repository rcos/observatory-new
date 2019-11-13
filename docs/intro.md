# Introduction

This document attempts to explain the layout and structure of the project, as
well as some guides on how to do common tasks (some are more complicated than
they seem).

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
