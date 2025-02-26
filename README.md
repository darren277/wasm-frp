# How to Build and Run

## About

This is a Rust only full stack application, with the backend written in Rust, a WebAssembly Rust front end (Yew), and even uses SurrealDB (written in Rust) for multimodel data storage.

## Docker

docker-compose build
docker-compose up

## How to run locally

### Frontend

From `frontend` directory:
0. `cargo install wasm-pack` and `cargo install simple-http-server` first.
1. `make build`.
2. `make copy`.

### Backend

From `backend` directory:
1. `cargo build`.
2. `cargo run`.
