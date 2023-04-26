FROM rust:1.67 as builder
WORKDIR /usr/src/ferengi-api
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/ferengi-api /usr/local/bin/ferengi-api

EXPOSE 80

CMD ["ferengi-api"]