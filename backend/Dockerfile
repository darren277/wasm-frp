# Use an official Rust image as a builder stage
FROM rust:1.81 as builder
WORKDIR /usr/src/myapp
# Copy backend source code
COPY ./ .
# Build your application
RUN cargo install --path .


# wasm_frp_backend: /lib/x86_64-linux-gnu/libm.so.6: version `GLIBC_2.29' not found (required by wasm_frp_backend)
# todo! fix this


# For the final image, use a minimal Debian image
FROM debian:buster-slim
# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/wasm_frp_backend /usr/local/bin/wasm_frp_backend
# Expose port 3007 for the backend service
EXPOSE 3007
# Run the binary
CMD ["wasm_frp_backend"]
