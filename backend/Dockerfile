FROM rust:1.81 as builder
WORKDIR /usr/src/myapp

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN cargo install --path .

COPY ./static ./static

FROM rust:1.81 as runtime

# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/wasm_frp_backend /usr/local/bin/wasm_frp_backend

EXPOSE 3007

CMD ["wasm_frp_backend"]
