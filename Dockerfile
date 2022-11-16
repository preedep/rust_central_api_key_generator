# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.65.0-slim-buster as cargo-build

RUN apt-get update
RUN apt-get install musl-tools -y
RUN apt-get install -y build-essential cmake libcppunit-dev git subversion wget libpq-dev libc-dev unzip zip libssl-dev
RUN apt-get install -y zlib1g-dev && rm -rf /var/lib/apt/lists/*

ARG PREFIX="/usr/local/ssl"

ARG OPENSSL_VERSION="OpenSSL_1_1_1g"
ARG OPENSSL_SHA256="41bac751d85f89a7d821324b7ffb35526a310db014ab6a4fe17fddaa011b7024"
RUN cd /usr/local/src \
  && wget "https://github.com/openssl/openssl/archive/${OPENSSL_VERSION}.zip" -O "${OPENSSL_VERSION}.zip" \
  && echo "$OPENSSL_SHA256" "${OPENSSL_VERSION}.zip" | sha256sum -c - \
  && unzip "${OPENSSL_VERSION}.zip" -d ./ \
  && cd "openssl-${OPENSSL_VERSION}" \
  && ./config --static -static -d --prefix=${PREFIX} --openssldir=${PREFIX} && make -j$(nproc) all && make install \
  && mv /usr/bin/openssl /root/ \
  && ln -s ${PREFIX}/bin/openssl /usr/bin/openssl

RUN echo "${PREFIX}/lib" >> /etc/ld.so.conf.d/ssl.conf && ldconfig

ENV OPENSSL_DIR ${PREFIX}

#ENV OPENSSL_DIR /usr/local/ssl/
# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/web_api_key

COPY Cargo.toml Cargo.toml

RUN mkdir src/
COPY src/ src/

# RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cargo install --path .


#FROM ekidd/rust-musl-builder:latest as cargo-build
#ADD --chown=rust:rust . ./
#RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM debian:buster-slim

RUN apt-get update && apt-get -y install curl libssl-dev libpq-dev libc-dev zlib1g-dev

RUN openssl s_client -connect southeastasia-1.in.applicationinsights.azure.com:443 -showcerts </dev/null 2>/dev/null | sed -e '/-----BEGIN/,/-----END/!d' | tee "/usr/local/share/ca-certificates/ca.crt" >/dev/null && \
update-ca-certificates

RUN mkdir -p /web_api_key

COPY --from=cargo-build /usr/local/cargo/bin/web_api_key /web_api_key/web_api_key.linux


#COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/web_api_key /web_api_key/web_api_key.linux

WORKDIR /web_api_key

ADD env.sh env.sh
ADD static/ static/


USER 1000
# default command
CMD ["/web_api_key/web_api_key.linux"]