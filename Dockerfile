# Dockerfile for Observatory-new

# This takes advantage of multi-stage Docker builds to first compile and
# bundle Observatory-new and then create a Docker image containing only
# the necessary components.

# Instructions on deploying this Docker image are in SETUP.md

# --- Docker Build Stage 1 ---

# Pulls from the MUSL builder since we are going to target Alpine Linux
FROM ekidd/rust-musl-builder:nightly as builder

# Copy in all the source files and switch to it
COPY . /build/
WORKDIR /build/

# Fix permissions on source code.
RUN sudo chown -R rust:rust /build/

# Build the project in release mode
RUN cargo build --release

# --- Docker Build Stage 2 ---

# Use Alpine Linux for it's small footprint.
FROM alpine

# Create a new user
RUN adduser -S observatory

# Create the folder that the database will be in
RUN mkdir -p /var/observatory

# Change the owner of the database folder
RUN chown -R observatory /var/observatory

# Switch to the user
USER observatory

# Copy in the binary from the builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/observatory /

# Copy in the config from the builder
COPY --from=builder /build/Rocket.toml /

# Expose the HTTP port
EXPOSE 8000

# Run Observatory
CMD /observatory