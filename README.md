# graphql_rs
Example of how to use graphql with Rust.

## Getting started
First, make sure docker (with `compose`) is installed.

`Make` - will run all necessary commands to start services.

## Endpoints
`localhost:5050` - Postgres admin.


## Migrations
Uses (and requires) sqlx-cli. Install via `cargo install`.

### Add migration
`sqlx migrate add -r <name> --source ./db/migrations`

Then modify the `up` and `down` migration files.

### Apply migrations
`sqlx migrate run --source ./db/migrations --database-url <DB_URL>`

### Revert migrations
`sqlx migrate revert --source ./db/migrations --database-url <DB_URL>`
