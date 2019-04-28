# observatory-new

Rewrite of the RCOS observatory in a way that hopefully makes a bit more sense.
Since we have kinda lost track of what number we're on at this point I am
codenaming this version Observatory-New (I think it's the 5th version though).

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
First you need to install the `openssl` development headers.
How to do this varies by system but on Linux the packages are usually named
something like `libssl-devel` or similar.

Rocket requires Rust Nightly ([for now](https://github.com/SergioBenitez/Rocket/issues/19))
so you need to set [Rustup](https://rustup.rs) to use it by running the following in the
`observatory-new` folder.

```
$ rustup override set nightly
```

however the Rust official tooling does not support the generic `nightly` target
so I suggest using the latest dated version of `nightly` that the [RLS]()
supports. You can check that [on this page](https://rust-lang.github.io/rustup-components-history/)
and can install it with the following command. Make sure to keep this version
up to date.

```
$ rustup override set nightly-YYYY-MM-DD
```

After that it's as simple as
```
$ cargo build
```

And to run do
```
$ cargo run
```

## Deploying

Please read [the Setup instructions](./SETUP.md) for information on how to setup
and deploy observatory-new.

## Documentation
The code is primarily documented using in-code doc comments.
This can be viewed either by browsing the source or in a web browser with.
```
$ cargo doc --open
```
