# 1: Build
FROM rust:1.63.0 as builder

# 1a: Prepare toolchain
RUN apt update && \
    apt install -y musl-tools musl-dev && \
    rustup target add wasm32-unknown-unknown && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download and compile Rust dependencies using fake source code and store as a separate Docker layer
WORKDIR /home/appuser/app

COPY .docker/main.rs src/bin/server.rs

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --target x86_64-unknown-linux-musl --release

# 1c: Build the application using the real source code
COPY src/ src/

RUN cargo build --target x86_64-unknown-linux-musl --release

# 2: Copy the excutable and extra files to an empty Docker image
FROM scratch

COPY --chown=root:root .docker/passwd /etc/passwd
COPY --chown=root:root .docker/group /etc/group

USER appuser:appgroup

ENV CONTAINER_REGISTRY_GATEWAY__HTTP_SERVER__HOST=0.0.0.0
ENV CONTAINER_REGISTRY_GATEWAY__HTTP_SERVER__PORT=80

EXPOSE 80

WORKDIR /home/appuser/app

COPY --chown=appuser:appgroup --from=builder /home/appuser/app/target/x86_64-unknown-linux-musl/release/container-registry-gateway-server container-registry-gateway-server

CMD [ "./container-registry-gateway-server" ]
