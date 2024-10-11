# How to Build and Run

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
