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

# Strip debug symbols out of binary
RUN strip /build/target/x86_64-unknown-linux-musl/release/observatory

# --- Docker Build Stage 2 ---

# Use Alpine Linux for it's small footprint.
FROM alpine

# Set the workdir
WORKDIR /

USER root

# Create the user's home folder and move to it
RUN mkdir -p /home/observatory
WORKDIR /home/observatory

# Create a new user
RUN adduser -h /home/observatory -S observatory

# Create the folder that the database will be in
RUN mkdir -p /var/lib/observatory

# Change the owner of the database folder
RUN chown -R observatory /var/lib/observatory/

# Switch to the user
USER observatory

# Copy in the binary from the builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/observatory .

# Expose the HTTP port
EXPOSE 8000

# Run Observatory
CMD ./observatory
