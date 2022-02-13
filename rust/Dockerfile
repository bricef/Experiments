#
# Build Stage
#

# Build rust application
FROM amd64/rust:alpine AS builder
RUN apk add --update --no-cache make musl-dev

# Set up rust dependencies
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src/lib.rs src/lib.rs
RUN cargo build --lib --release --target x86_64-unknown-linux-musl
RUN rm src/lib.rs



# Build app
COPY src/ ./src/
RUN cargo build --bins --release --target x86_64-unknown-linux-musl

#
# Package Stage
#
FROM gcr.io/distroless/static AS main
COPY --from=builder target/x86_64-unknown-linux-musl/release/hello ./
CMD ["./hello"]
