FROM rust:label as builder

WORKDIR /app

COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./src ./

RUN cargo build

FROM 
