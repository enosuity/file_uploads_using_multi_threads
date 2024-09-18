# Stage 1: Build the Rust application with the nightly version
FROM rustlang/rust:nightly as builder

# Install Diesel CLI and other necessary packages
RUN cargo install diesel_cli --no-default-features --features postgres

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Rust project files into the container
COPY . .

# Install necessary tools for building Rust projects
RUN rustup default nightly && \
    rustup update && \
    cargo install --path .

# Build the project in release mode to optimize performance
RUN cargo build --release

# Stage 2: Create a minimal image with the compiled binary
# FROM debian:bullseye-slim
FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/app

# Install the necessary libraries for PostgreSQL
RUN apt-get update && apt-get install -y libpq-dev

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/blog /usr/local/bin/blog

# Copy the Diesel CLI for running migrations at runtime
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel


# Copy the .env file if it exists
COPY .env .env

# Copy migrations and templates if needed
COPY migrations ./migrations
COPY templates ./templates

# Expose the port the app runs on (modify if necessary)
EXPOSE 5000

# Run the compiled binary
# CMD ["blog"]
CMD ["sh", "-c", "diesel database setup && diesel migration run && blog"]
