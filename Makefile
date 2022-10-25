build:
	docker compose build

start:
	docker compose up

dev:
	cargo watch -x run

test:
	cargo test