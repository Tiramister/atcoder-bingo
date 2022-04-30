### builder image
FROM rust:1.59-bullseye as builder

WORKDIR /app

# pre-build Cargo.toml
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

# build
COPY ./src ./src
COPY ./templates ./templates
RUN cargo build --release


### release image
FROM debian:latest
WORKDIR /app

# install CA certificate
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates

# copy binaries and run
COPY ./static ./static
COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY --from=builder /app/target/release/choose_problems /usr/local/bin/choose_problems
COPY --from=builder /app/target/release/update_users /usr/local/bin/update_users
COPY ./script/run_backend.sh .
CMD ["sh", "run_backend.sh"]
