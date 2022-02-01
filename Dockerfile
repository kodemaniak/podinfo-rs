FROM docker.io/ekidd/rust-musl-builder:1.57.0 as builder

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:3.15.0

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/podinfo-rs /

WORKDIR /

USER 1000

ENV PODINFORS_BIND_IP=0.0.0.0

ENTRYPOINT [ "./podinfo-rs" ]