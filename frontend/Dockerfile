# Builder stage for compiling Rust to WebAssembly
FROM rust:1.56 as builder
WORKDIR /usr/src/myapp
COPY ./frontend .
# Use wasm-pack to build the project
RUN cargo install wasm-pack
RUN wasm-pack build --target web

# Final stage for serving the static files
FROM nginx:stable-alpine
# Copy static files and WASM output to the Nginx serve directory
COPY --from=builder /usr/src/myapp/static /usr/share/nginx/html
COPY --from=builder /usr/src/myapp/pkg /usr/share/nginx/html/pkg
# Expose port 80 for the frontend
EXPOSE 80
