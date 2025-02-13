# builder stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# copy the binary from builder
COPY --from=builder /app/target/release/mylodev /app/mylodev
COPY --from=builder /app/templates /app/templates

# expose the port your app runs on
EXPOSE 3000

# run the binary
CMD ["./mylodev"] 