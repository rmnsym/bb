FROM rust:1.60 AS builder
WORKDIR /home/bb
COPY . .
RUN adduser --home /home/bb --shell /bin/false bb ¥
  && cargo build --release ¥
  && cp target/release/bb /usr/local/bin/bb
USER bb
ENTRYPOINT [ "/usr/local/bin/bb" ]
