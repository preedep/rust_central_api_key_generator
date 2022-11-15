# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

#FROM rust:1.65.0-slim-buster as cargo-build

#RUN apt-get update
#RUN apt-get install musl-tools -y
#RUN apt-get install -y build-essential cmake libcppunit-dev git subversion wget libpq-dev libc-dev libssl-dev
#RUN apt-get install -y zlib1g-dev && rm -rf /var/lib/apt/lists/*

#RUN rustup target add x86_64-unknown-linux-musl

#WORKDIR /usr/src/web_api_key

#COPY Cargo.toml Cargo.toml

#RUN mkdir src/
#COPY src/ src/

#RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM ekidd/rust-musl-builder:latest as cargo-build

ADD --chown=rust:rust . ./

RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM debian:buster-slim

RUN apt-get update && apt-get -y install curl libssl-dev libpq-dev libc-dev zlib1g-dev

RUN openssl s_client -connect southeastasia-1.in.applicationinsights.azure.com:443 -showcerts </dev/null 2>/dev/null | sed -e '/-----BEGIN/,/-----END/!d' | tee "/usr/local/share/ca-certificates/ca.crt" >/dev/null && \
update-ca-certificates

RUN mkdir -p /web_api_key

#COPY --from=cargo-build /usr/local/cargo/bin/web_api_key /web_api_key/web_api_key.linux

COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/web_api_key /web_api_key/web_api_key.linux

WORKDIR /web_api_key

USER 1000
# default command
CMD ["/web_api_key/web_api_key.linux"]