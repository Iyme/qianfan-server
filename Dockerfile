
FROM rust:1.82.0
WORKDIR /qianfan-server

COPY . .

RUN cargo build --release

EXPOSE 8181
CMD ["./target/release/qianfan-server"]


