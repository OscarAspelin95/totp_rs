.PHONY: db migrate prepare up down

db:
	docker compose up -d --wait postgres

migrate: db
	sqlx migrate run

prepare: migrate
	cargo sqlx prepare --workspace

up: prepare
	docker compose up -d --build server

down:
	docker compose down
