FROM alpine:3.15.4 AS builder

WORKDIR /home/bb

COPY . .

RUN  apk add --no-cache gcc musl-dev rust cargo \
  && cargo build --release

FROM alpine:3.15.4

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D bb

COPY --from=builder /home/bb/target/release/bb /opt/bb/bb

WORKDIR /home/bb
USER bb

ENTRYPOINT [ "/opt/bb/bb" ]
