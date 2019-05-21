# Importer

This is a [Ruby](https://ruby-lang.org") script that imports
from the old Observatory 3 [Mongo](https://mongodb.com) database
and generates an SQL file that can be used to on
the newer [SQLite3](https://sqlite.org) database.

Run the script with `rake run` or `bundle exec ruby importer.rb`.

## Dependencies
This uses the following Ruby Gems:
- [Mongo](https://rubygems.org/gems/mongo)

These gems require that you have `ruby-devel`
installed in order to build their native bindings.
These should be available from your distrobutions package manager.

These gems can be installed locally using `rake install` or
`bundle install --path vendor/bundle`.
