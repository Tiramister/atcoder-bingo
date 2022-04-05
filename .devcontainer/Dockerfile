FROM rust:1.59-bullseye

# Install Rust packages for development
RUN rustup component add rustfmt clippy
RUN cargo install cargo-edit

# Install PostgreSQL client
RUN apt-get update \
    && apt-get install -y postgresql-client-13
