FROM rust:1.59-bullseye

# Install Rust packages for development
RUN cargo install cargo-edit sccache
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

# Install PostgreSQL client
RUN apt-get update \
    && apt-get install -y postgresql-client-13
