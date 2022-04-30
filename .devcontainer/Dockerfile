FROM rust:1.59-bullseye

# install command line tools
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y --no-install-recommends \
    sudo postgresql-client-13

# add user
ENV LOCAL_UID=${LOCAL_UID:-1000} \
    LOCAL_USER="vscode" \
    LOCAL_PASSWORD="vscode"
RUN useradd -m --uid ${LOCAL_UID} --groups sudo -s $(which bash) ${LOCAL_USER} && \
    echo "${LOCAL_USER}:${LOCAL_PASSWORD}" | chpasswd
USER ${LOCAL_USER}

# install Rust packages for development
RUN rustup component add rustfmt clippy
RUN cargo install cargo-edit
