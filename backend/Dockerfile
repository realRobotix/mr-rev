# Build stage
FROM rust:latest as builder

WORKDIR /app

# Create a dummy project to cache dependencies
RUN cargo new --bin backend
WORKDIR /app/backend
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN cargo build --release
RUN rm src/*.rs

# Copy actual source code
COPY ./src ./src

# Build the actual binary
RUN rm ./target/release/deps/backend*
RUN cargo build --release

# Runtime stage
FROM debian:stable-slim

# Copy binary
COPY --from=builder /app/backend/target/release/backend .

EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["./backend"]
