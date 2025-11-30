FROM rust:1.85-slim-bullseye AS builder
WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/ && echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/binsider*
COPY . .
RUN cargo build --locked --release
RUN mkdir -p build-out/
RUN cp target/release/binsider build-out/

FROM debian:bullseye-slim AS runner
WORKDIR /app
COPY --from=builder /src/build-out/binsider .
USER 1000:1000
ENTRYPOINT ["./binsider"]
