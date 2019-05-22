# Importer

This is a [Ruby](https://ruby-lang.org") script that imports
from the old Observatory 3 [Mongo](https://mongodb.com) database
and generates an SQL file that can be used to on
the newer [SQLite3](https://sqlite.org) database.

Run the script with `rake run` or `bundle exec ruby importer.rb`.

## Dependencies
This uses the following Ruby Gems:
- [Mongo](https://rubygems.org/gems/mongo)

This gem requires that you have `ruby-devel`
installed in order to build its native bindings.
This should be available from your distributions package manager.

Gems can be installed locally using `rake deps` or
`bundle install --path vendor/bundle`.
