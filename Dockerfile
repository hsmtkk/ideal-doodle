FROM rust:1.57 AS builder

WORKDIR /opt

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11 AS runtime

COPY --from=builder /opt/target/release/ideal-doodle /usr/local/bin/ideal-doodle
COPY --from=builder /opt/target/release/print_env_vars /usr/local/bin/print_env_vars

ENTRYPOINT ["/usr/local/bin/print_env_vars"]
