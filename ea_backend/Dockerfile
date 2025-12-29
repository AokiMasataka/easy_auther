FROM rust:1.87.0-bullseye AS build

WORKDIR /workspace

RUN apt update && \
    apt install -y \
      build-essential \
      cmake \
      libclang-dev \
      musl-tools

RUN rustup target add x86_64-unknown-linux-musl

COPY ./src /workspace/src
COPY ./migrations /workspace/migrations
COPY ./.sqlx /workspace/.sqlx
COPY ./Cargo.toml /workspace/Cargo.toml
COPY ./Cargo.lock /workspace/Cargo.lock

RUN SQLX_OFFLINE=true cargo build --release

# FROM debian:bullseye-slim
# COPY --from=build /workspace/target/release/ea_backend /usr/local/bin/ea_backend
# CMD ["ea_backend"]

FROM gcr.io/distroless/cc:nonroot
WORKDIR /app
COPY --from=build /workspace/target/release/ea_backend /app/ea_backend

USER nonroot
ENTRYPOINT ["/app/ea_backend"]