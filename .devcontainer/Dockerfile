FROM debian:11

# install command line tools
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y --no-install-recommends \
    build-essential openssl libssl-dev pkg-config \
    ca-certificates sudo curl git postgresql-client-13

# add user
ARG LOCAL_UID="1000"
ARG LOCAL_USER="user"
ARG LOCAL_PASSWORD="passwd"
RUN useradd -m --uid ${LOCAL_UID} --groups sudo -s $(which bash) ${LOCAL_USER} && \
    echo "${LOCAL_USER}:${LOCAL_PASSWORD}" | chpasswd
USER ${LOCAL_USER}
ENV HOME=/home/${LOCAL_USER}

# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH=${PATH}:${HOME}/.cargo/bin

# install node via nvm
ENV NVM_DIR=${HOME}/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash && \
    . ${NVM_DIR}/nvm.sh && \
    nvm install node && \
    npm install -g npm
