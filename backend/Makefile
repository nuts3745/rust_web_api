db:
	docker compose exec database psql -U admin todos

build:
	docker compose build

start:
	docker compose up

dev:
	sqlx db create
	sqlx migrate run
	cargo watch -x fmt -x run

test:
	cargo test

# standalone test
test-s:
	cargo test --no-default-features