FROM rust:1.67 as builder
WORKDIR /usr/src/ferengi-api
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ferengi-api /usr/local/bin/ferengi-api
CMD ["ferengi-api"]