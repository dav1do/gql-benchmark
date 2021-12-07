# Use Amazonlinux as our base image.
FROM amazonlinux:latest AS base

# The Rust toolchain to use when building our image.
ARG TOOLCHAIN=stable

# Make sure we have basic dev tools for building C libraries. Our goal
# here is to support the musl-libc builds and Cargo builds needed for a
# large selection of the most popular crates.
RUN yum -y update \
 && yum -y groupinstall "Development Tools" \
 && yum -y install openssl-devel \
 && yum -y install zip wget jq

ENV BUILD_DIR=/build \
    OUTPUT_DIR=/output \
    PACKAGE_DIR=/package \
    RUST_BACKTRACE=1 \
    RUSTFLAGS="-C debuginfo=0" \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    PREFIX=/musl

RUN mkdir -p $OUTPUT_DIR \
  && mkdir -p $PACKAGE_DIR

WORKDIR $PREFIX

ENV CARGO_ENV=/usr/local/cargo/env

# Install our Rust toolchain and the `musl` target.  We patch the
# command-line we pass to the installer so that it won't attempt to
# interact with the user or fool around with TTYs.  We also set the default
# `--target` to musl so that our users don't need to keep overriding it manually.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TOOLCHAIN \
    && . $CARGO_ENV

RUN cat $CARGO_ENV

ENV PKG_CONFIG_ALL_STATIC=true \
    PKG_CONFIG_ALLOW_CROSS=true \
    PATH=/root/.cargo/bin:$PATH

# could make ^^ a dockerfile


ENV PROJECT=api-server \
    BUILD_DIR=/build \
    EXPORT_DIR=/export \
    RUST_BACKTRACE=1 \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo

RUN mkdir -p $EXPORT_DIR

WORKDIR $BUILD_DIR

ADD api-server/ api-server/
ADD db/ db/
ADD bench-graphql/ bench-graphql/
ADD shared-types/ shared-types/
ADD util/ util/
ADD docker/.env .env
ADD docker/build-ecs.sh build-ecs.sh
COPY Cargo.toml Cargo.loc[k] $BUILD_DIR/

VOLUME $EXPORT_DIR

CMD /bin/bash build-ecs.sh
