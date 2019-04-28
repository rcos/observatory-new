# Setup

There are two major ways of install Observatory-new. The first is a traditional
manual install from source, while the second uses the [Docker](https://docker.com)
container management system to make deployment simpler.

Generally speaking we recommend using Docker, however that may sometimes not
be ideal or even possible.

__**Please Note**__ that all versions of Observatory-new do not provide SSL/TLS
support out of the box. We highly suggest using a reverse proxy such
as NGINX or HAProxy in order to increase security.

## Docker

[Docker](https://docker.com) is a container runtime and management system that
makes deploying and updating applications easy and painless.

This is the recommended way of install Observatory.

Note that for all `docker` commands you will either need to be part of the
`docker` group or run the command as root with `sudo`.

### Building

The Docker image can be built using the following command from the directory
containing the `Dockerfile`.

```
$ docker build -t rcos/observatory .
```

### Running
Observatory can be run with Docker with the following command once it has been
built or pulled.

```
$ docker run --name observatory -i -p 8000:8000 rcos/observatory
```

Once that is running Observatory-new will be available on port 8000.

### Configuration

Before building the image you can edit `Rocket.toml` to configure the server
as you want it.

Runtime configuration can be done using enviroment variables.
See [this page in the Rocket documentation](https://rocket.rs/guide/configuration/#environment)
for more information.

If you did not set your secret key before building you should do so
using an enviroment variable.

### Mounts

The only mount of interest is the database which is stored at
`/var/observatory/` and can be mounted to the host system using the
`-v` flag or eqivalent.

## Manual

This is for a manual installation on a bare Unix-like OS.
We also provide Docker images for easier automated deployment.

The first 3 steps can be performed on any machine so long as it is the same
architecture and `libc` as the server.

The first few steps are quite similar to the [Building section in the README](./README.md#Building)

### Release Build

First you will have to build Observatory-new in release mode. This can be done
easily with
```
$ cargo build --release
```

### Configuration

The configuration for the server lives in `Rocket.toml`. While we have strived
for sane defaults you may want to change things in it.
Please follow the instrucitons in the config file on generating a secret key for
your server in production mode.

### Copy files to Server and Run

Now that all this is done, assuming you built the code on a machine other than
the server, you must copy the files over. There are only 3 necessary files to copy:

- `observatory` from `target/release/` to anywhere on the server.
- `Rocket.toml` to the same folder as `observatory`.

Once all that is done you can simply run the `observatory` file and it will be
available on port 8000.