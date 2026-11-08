# totp_rs

## Requirements

- Ubuntu
- Rust
- Docker (+ compose)
- sqlx-cli (`cargo install sqlx-cli`)

## .env file

A `.env` file is required, both for local development and for running `server` and postgres inside docker. Note however that some environment variables are overwritten inside `docker-compose.yaml`.

```bash
# axum server
PORT=<port> # server port - e.g., 8080. Overwritten in docker-compose.yaml.
ADDR=127.0.0.1 # server address. Overwritten in docker-compose.yaml.
ADMIN_TOKEN=<token> # for admin server endpoints.
# postgres
POSTGRES_USER=<username> # e.g., "admin"
POSTGRES_PASSWORD=<password>
POSTGRES_PORT=<port> # e.g., 5432
POSTGRES_DB=<db_name> # e.g., my_db

# used by server and sqlx. Overwritten in docker-compose.yaml.
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}
```

## Running

```bash
make up   # start postgres, migrate, sqlx prepare, build + run server
make down
```

## Migrations

Are located in the `migrations` folder and should be handled by `sqlx-cli` (`cargo install sqlx-cli`).

Not run by the server or docker. Assuming `sqlx-cli` is run locally (outside docker) then:

```bash
sqlx migrate add "name_of_migration" # then, modify migration file as needed.
make migrate # or: sqlx migrate run (reads DATABASE_URL from .env)
```

## sqlx offline cache

`query_as!` checks queries against the database at compile time. The docker build has no database, so it uses the cache in `.sqlx` instead. Regenerate it (and commit) after changing queries or migrations:

```bash
make prepare # or: cargo sqlx prepare --workspace
```
