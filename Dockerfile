FROM rust:alpine AS build_alpine

WORKDIR /build
ADD . .

RUN cargo build --release
