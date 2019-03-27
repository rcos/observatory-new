# observatory-new

Rewrite of the RCOS observatory
in a way that hopefully makes a bit more sense.

This implementation is intended to be simpler than previous incarnations in
order to be easier to maintain in the long run.

It renders pages as plain HTML with minimal JavaScript and uses SQLite
as the database backend.

## Major Dependencies
- [Diesel](https://diesel.rs): Database ORM
- [Gotham](https://gotham.rs): Web framework based on [Hyper](https://hyper.rs/);
- [Askama](https://github.com/djc/askama): Templating engine

## Building
This is a pure Rust project so it's as simple as
```
$ cargo build
```

To run do
```
$ cargo run
```

## Docs
The code is documented using in-code doc comments.
This can be viewed either by browsing the source or in a web browser with.
```
$ cargo doc --open
```