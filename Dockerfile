# Build
FROM rust:1.73.0-alpine as builder

RUN apk update \
  && apk add musl-dev \
  && rustup target add aarch64-unknown-linux-musl

WORKDIR /build

COPY Cargo.toml Cargo.lock .

RUN mkdir src \
  && echo "fn main() {}" > src/main.rs \
  && cargo build --release --target aarch64-unknown-linux-musl

COPY src/ src/
COPY images/ images/
COPY templates/ templates/

RUN touch src/main.rs \
  && cargo build --release --target aarch64-unknown-linux-musl

# Run
FROM alpine as runtime

WORKDIR /opt/solar_system_info

COPY --from=builder build/target/aarch64-unknown-linux-musl/release/solar_system_info* .

ENTRYPOINT ["./solar_system_info" ]
