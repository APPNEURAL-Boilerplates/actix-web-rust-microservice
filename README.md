# Actix Web Microservice Boilerplate

Production-ready Actix Web microservice starter for Rust APIs.

## Features

- Actix Web REST API
- Versioned routes under `/api/v1`
- Health and readiness endpoints
- Request ID middleware using `x-request-id`
- JSON success and error response helpers
- Central `ResponseError` implementation
- Invalid JSON handling
- Service/repository structure
- Example items module
- In-memory repository placeholder
- Outbound HTTP client placeholder
- Event publisher placeholder
- Background worker placeholder
- Integration tests
- Docker, Docker Compose, Makefile, and GitHub Actions CI

## Requirements

- Rust 1.88+
- Cargo
- Docker, optional

## Quick start

```bash
cp .env.example .env
cargo run
```

The service starts on:

```txt
http://localhost:8080
```

## Endpoints

```txt
GET  /                     Service metadata
GET  /api/v1/health        Health check
GET  /api/v1/ready         Readiness check
GET  /api/v1/items         List items
POST /api/v1/items         Create item
GET  /api/v1/items/{id}    Get item by ID
```

## Example request

```bash
curl -X POST http://localhost:8080/api/v1/items \
  -H "content-type: application/json" \
  -H "x-request-id: demo-request-1" \
  -d '{"name":"Keyboard","description":"Mechanical keyboard","price":99.99}'
```

## Test

```bash
cargo test
```

Full check:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Or:

```bash
make check
```

## Docker

```bash
cp .env.example .env
docker compose up --build
```

## Environment variables

| Variable | Default | Description |
| --- | --- | --- |
| `SERVICE_NAME` | `actix-web-microservice` | Service name returned by metadata endpoints |
| `ENVIRONMENT` | `local` | Runtime environment name |
| `HOST` | `127.0.0.1` | Bind host |
| `PORT` | `8080` | Bind port |
| `RUST_LOG` | `info,actix_web=info` | Tracing/env-filter log level |
| `CORS_ALLOWED_ORIGIN` | `*` | CORS origin; use a specific origin in production |
| `EXTERNAL_API_BASE_URL` | `http://localhost:8081` | Placeholder upstream base URL |
| `REQUEST_TIMEOUT_SECONDS` | `10` | Outbound HTTP timeout |

## Project structure

```txt
src/
├─ main.rs
├─ lib.rs
├─ app.rs
├─ config.rs
├─ logging.rs
├─ state.rs
├─ clients/
├─ common/
├─ events/
├─ modules/
│  ├─ health.rs
│  ├─ root.rs
│  └─ items/
└─ workers/
```

## Production notes

- Replace the in-memory repository with Postgres, MySQL, Redis, or another durable store.
- Replace the event publisher placeholder with Kafka, RabbitMQ, NATS, SQS, or your platform queue.
- Restrict CORS in production instead of using `*`.
- Keep secrets out of `.env.example` and source control.
- Add authentication/authorization before exposing private APIs.
