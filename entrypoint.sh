#!/bin/sh

# This file is used with Docker to generate the secret key at runtime
# and *NOT* at build time which is very important

KEY=$(openssl rand -base64 32 | sed 's,\/,\\/,g')
sed -i "s/^# secret_key = \"CHANGEME\"/secret_key = \"$KEY\"/" Rocket.toml
./observatory