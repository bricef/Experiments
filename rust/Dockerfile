#
# Build Stage
#

# Build rust application
FROM amd64/rust:alpine AS builder
RUN apk add --update --no-cache make musl-dev

# Set up rust dependencies
WORKDIR /build
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN touch Cargo.*
RUN mkdir ./src/
RUN touch ./src/lib.rs
RUN cargo build --lib --release --target x86_64-unknown-linux-musl
RUN rm ./src/lib.rs

# Build app
COPY src/ ./src/
# Cargo will balk if the following isn't included.
RUN find src/ -exec touch {} +
RUN cargo build --bins --release --target x86_64-unknown-linux-musl

#
# Package Stage
#
FROM gcr.io/distroless/static AS runtime
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/hello ./
CMD ["./hello"]
