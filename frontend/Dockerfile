# Builder stage for compiling Rust to WebAssembly
FROM rust:1.81 as builder
#RUN mkdir /usr/src/myapp
WORKDIR /usr/src
COPY ./ .
# Use wasm-pack to build the project
RUN cargo install wasm-pack
RUN wasm-pack build --target web

# Final stage for serving the static files
FROM nginx:stable-alpine
# Copy static files and WASM output to the Nginx serve directory
COPY --from=builder /usr/src/static /usr/share/nginx/html
#COPY --from=builder /usr/src/wasm_frp_frontend/pkg /usr/share/nginx/html/pkg
# Expose port 87 for the frontend
EXPOSE 87
