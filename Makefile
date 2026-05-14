.PHONY: run test fmt clippy check docker-up docker-down

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

docker-up:
	docker compose up --build

docker-down:
	docker compose down
