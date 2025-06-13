FROM rust:1.81 as builder

# Install nightly toolchain
RUN rustup install nightly && rustup default nightly

WORKDIR /usr/src/myapp

COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

# Use nightly to build the project
RUN cargo +nightly install --path .

RUN cargo install --path .

COPY ./static ./static

FROM rust:1.81 as runtime

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/wasm_frp_backend /usr/local/bin/wasm_frp_backend
COPY --from=builder /usr/src/myapp/static /app/static

EXPOSE 8080

CMD ["wasm_frp_backend"]
