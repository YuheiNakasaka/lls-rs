FROM rust:1.66-buster

RUN apt-get update && \
  apt-get -y install curl git && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  rustup component add rls rust-analysis rust-src rustfmt clippy && \
  cargo install cargo-edit cargo-watch
