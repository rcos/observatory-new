# observatory-new

Rewrite of the RCOS observatory
in a way that hopefully makes a bit more sense.

This implementation is intended to be simpler than previous incarnations in
order to be easier to maintain in the long run.

It renders pages as plain HTML with minimal JavaScript and uses SQLite
as the database backend.

If you would like to help out please read [CONTRIBUTING.md](./CONTRIBUTING.md).

## Major Dependencies
- [Diesel](https://diesel.rs): Database ORM
- [Rocket](https://rocket.rs): Web framework
- [Askama](https://github.com/djc/askama): Templating engine
- [FullCalendar](https://fullcalendar.io/): For JavaScript calendars
- [Bootstrap](https://getbootstrap.com): CSS framework

## Building
First you need to install the `openssl` and `sqlite` development headers.
How to do this varies by system but on Linux the packages are usually named
something like `libssl-devel` and `sqlite-devel` or similar.

You will also need the Diesel CLI. It can be easily installed with:

```
$ cargo install diesel_cli --no-default-features --features sqlite
```

To set up the SQL database run the following command:
```
$ diesel migration run --database-url=observ.sqlite
```

Rocket requires Rust Nightly ([for now](https://github.com/SergioBenitez/Rocket/issues/19))
so you need to set [Rustup](https://rustup.rs) to use it by running the following in the
`observatory-new` folder.

```
$ rustup override set nightly
```

After that it's as simple as
```
$ cargo build
```

And to run do
```
$ cargo run
```

## Documentation
The code is primarily documented using in-code doc comments.
This can be viewed either by browsing the source or in a web browser with.
```
$ cargo doc --open
```
