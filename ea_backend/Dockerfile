FROM rust:1.87.0-bullseye AS build

WORKDIR /workspace

RUN apt update && \
    apt install -y \
      build-essential \
      cmake \
      libclang-dev \
      musl-tools

# RUN rustup target add x86_64-unknown-linux-musl

COPY src src
COPY migrations migrations
COPY .sqlx .sqlx
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

ENV DATABASE_URL=postgres://pgsql:pgsql@localhost/pgsql
ENV SQLX_OFFLINE=true

RUN cargo build --release


FROM gcr.io/distroless/cc:nonroot
WORKDIR /app
COPY --from=build /workspace/target/release/ea_backend /app/ea_backend

USER nonroot
ENTRYPOINT ["/app/ea_backend"]