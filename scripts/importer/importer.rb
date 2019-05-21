#!/usr/bin/env ruby

require "mongo"

# Make sure the Mongo URL is given
if not ARGV[0]
    raise RuntimeError, "No MongoDB URL given"
end

# Figure out where the database is or error
def find_db
    opath = "../../observ.db"
    vpath = "/var/lib/observatory/observ.db"
    if File.file?(opath);
        opath
    elsif File.file?(vpath)
        vpath
    else
        raise RuntimeError, "Could not find the database"
    end
end

# Connect to the mongo instance based on the CLI argument
# URL needs to be completely done out like this:
#
# 'mongodb://127.0.0.1:27017/test'
#
# with the database to access at the end
mongo_url = ARGV[0]
mongo_client = Mongo::Client.new(mongo_url)
mongo_db = client.database

# Create the output file
outfile = File.new("importer.sql", "w+")
outfile.puts("BEGIN TRANSACTION;");

# Iterate through all the projects and insert
# them into the database
client[:projects].each do |p|
    outfile.puts("INSERT INTO projects () VALUES ()")
end

outfile.puts("COMMIT;")
outfile.close()
