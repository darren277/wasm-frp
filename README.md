# How to Build and Run

## About

This is a Rust only full stack application, with the backend written in Rust, a WebAssembly Rust front end (Yew), and even uses SurrealDB (written in Rust) for multimodel data storage.

## Docker

1. `docker-compose build`.
2. `docker-compose up`.

## How to run locally

### Frontend

From `frontend` directory:
1. `cargo install wasm-pack` and `cargo install simple-http-server` first.
2. `make build`.
3. `make copy`.

### Backend

From `backend` directory:
1. `cargo build`.
2. `cargo run`.

# Tasks

- [ ] IDEA: Integrate with SurrealDB.
- [ ] ADD UNIT TESTS...

