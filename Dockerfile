# builder stage
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo generate-lockfile

COPY src ./src
COPY templates ./templates

RUN cargo build --release

# runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# copy the binary from builder
COPY --from=builder /app/target/release/mylodev /app/mylodev
COPY --from=builder /app/templates /app/templates

# verify files are copied correctly
RUN ls -la /app && ls -la /app/templates

# expose the port your app runs on
EXPOSE 3000

# run the binary
CMD ["./mylodev"] 