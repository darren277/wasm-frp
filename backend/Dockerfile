# Use an official Rust image as a builder stage
FROM rust:1.56 as builder
WORKDIR /usr/src/myapp
# Copy backend source code
COPY ./backend .
# Build your application
RUN cargo install --path .

# For the final image, use a minimal Debian image
FROM debian:buster-slim
# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/your_project_backend /usr/local/bin/your_project_backend
# Expose port 3000 for the backend service
EXPOSE 3000
# Run the binary
CMD ["your_project_backend"]
