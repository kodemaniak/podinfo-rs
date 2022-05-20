FROM docker.io/rust:1.61.0 as builder

WORKDIR /usr/src/podinfo-rs
COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.15.4

COPY --from=builder /usr/src/podinfo-rs/target/x86_64-unknown-linux-musl/release/podinfo-rs /

WORKDIR /

USER 1000

ENV PODINFORS_BIND_IP=0.0.0.0

ENTRYPOINT [ "./podinfo-rs" ]