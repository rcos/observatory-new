# Frequently Asked Questions

## General Questions

### What is the Observatory?

The Observatory is the primary website for the
[Rensselaer Center for Open Source](https://rcos.io)
used to keep track of projects, keep member attendance, and more!

### Why is this one "new"?

Because we lost count of how many iterations the Observatory has been through.
No, really. By my count this is Observatory 5 but I'm honestly not sure.
So I have opted to just call this version "new".

Eventually once it has stabilized all other versions will be archived and this
repo will be renamed to just "Observatory". Hopefully...

### Why a rewrite?

The current version of the RCOS website is written in AngularJS in 2013.
That code is brittle and outdated, and has been causing us problems for years.
But at this stage is so difficult to try and upgrade that a rewrite is simply easier.

There actually was an attempt at a rewrite before this on in VueJS. But that was
unfortunately abandoned by its author, and had some severe issues.

## Technical Justifications

### Why Rust?

[Rust](https://rust-lang.org) is a powerful systems programming language with a
lot of innovative and interesting features.

But that doesn't answer the question. There are a few reasons that I chose to
use Rust for this project.

1. I like Rust, a lot. It is probably the language that I know best. When I
   started this project it was just me for a few months, so I worked with what I
   was comfortable with.
   Now before you get the pitchforks out and yell at me just for picking what I
   wanted, read the other reasons I chose Rust.

2. Rust is safe. The language itself provides a very large number of safety
   garuntees that force you to write better code. Rust won't make you instantly
   write perfect bug-free code, but entire types of problems are simply not
   possible in the language.

3. Rust has a strong type system. This may seem unimportant, but for a project
   that is going to have an unknown number of developers working on it, over an
   unknown period of time, the strictness that a strong type system provides
   becomes important. Changes made in one place will cause type errors in
   others, letting you know what has broken.

4. Rust is fast. On-par with languages like C and C++. For a simple website this
   may seem trivial, but RCOS is a small non-profit student organization. Every
   cent we don't spend on servers is one we can spend on something more important.

There are a number of other reasons to like Rust, but these are the
ones that I think matter for this project.
In a nutshell Rust provides a level of strictness and safety, while also holding
your hand, that I believe overall raises the quality of the code. It makes
long-term maintainability easier, since the compiler will reject code that does
not conform.
And code that doesn't compile is code that doesn't crash and corrupt the
database.

### Why Rocket as the Web Framework?

Despite the maturity concerns that [Rocket](https://rocket.rs) has due to being
only available on Rust Nightly, it is still the easiest to use and arguably most
robust web framework available for Rust.

And it is approaching aviaility on Rust Stable very quickly, so that concern may
soon be gone.

### Why SQLite?

[SQLite](https://sqlite.org) is a fast and small embedded database used in every
industry for nearly every purpose.
In my personal opinion when it comes to an SQL database the question is "Do I
need the full power of PostgreSQL?" if you don't, then SQLite is the way for you.

Since in our use case we will be dealing with no more than a few dozen requests
at once we don't need that much, so SQLite suits our needs nicely.

Since we are using [Diesel](https://diesel.rs) as our ORM it is actuall fairly
simple to change backend servers if we ever want to. But for now, SQLite is enough.

### Why a server-side architecture?

This version of the Observatory, unlike its two immediate predecessors, is a
server-side application with minimal client-side JavaScript.

The reason for this is simple: It's easier. Most of what this website does is
read and write data to a database. Using a client-side system we would need to
have a two layer approach, design and create an API, connect the client to it,
worry about unauthorized API calls, etc.

With the server-side approach we just recieve HTTP requests and perform the
database function directly, and then simply return the rendered HTML.
This is the way websites have worked for decades, and it ultimately results in a
faster and easier to maintain site.

### Why avoid JavaScript?

I won't say this project explicitly *avoids* JavaScript, in fact we use it for
some important features like the calendar.

But with the server-side architecture we don't really need JavaScript that much.
In the project as a whole I would say we use less than 200 lines of JavaScript
that we wrote and only one external library,
[FullCalendar](https://fullcalendar.io).

### Why Docker?

Docker makes deploying applications trivially simple, and we can version the
whole project very easily.
Though I will admit for this project we don't actually need Docker that much.
When building the project you just get a single binary, so there isn't much in
the way of dependencies.

But I like Docker (or Podman) and find it to be a good way of deploying
applications, so I set it up. If you don't want to use it you don't have to
(try Podman instead).

## How-To Questions

Make sure you read the [README](../README.md) and the [howto.md](./howto.md)!

### How do I get started?

Check out the [README](../README.md), the
[contributing guide](../CONTRIBUTING.md) and the
[intro documention](./intro.md).

### How do I deploy it to a server?

Read the [deploying documentation](./deploying.md).

### How do I <add basic feature>?

Check the [how to](./howto.md) document. It has guides for some basic tasks.
