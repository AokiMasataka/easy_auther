FROM rust:1.84.0-bullseye AS build

WORKDIR /workspace

RUN apt update
RUN apt install -y build-essential
RUN apt install -y cmake
RUN apt install -y libclang-dev

COPY ./ .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build /workspace/target/release/ea_backend /usr/local/bin/ea_backend
CMD ["ea_backend"]