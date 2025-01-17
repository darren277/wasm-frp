FROM rust:1.81 as builder
WORKDIR /usr/src/myapp

COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

RUN cargo install --path .

COPY ./static ./static

FROM rust:1.81 as runtime

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/wasm_frp_backend /usr/local/bin/wasm_frp_backend
COPY --from=builder /usr/src/myapp/static /app/static

EXPOSE 8080

CMD ["wasm_frp_backend"]
