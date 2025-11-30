#!/bin/bash

# Load .env file into environment
set -e
set -a
source .env
set +a

until docker compose exec postgres pg_isready -U $POSTGRES_USER -d $POSTGRES_DB; do
	echo -e "Postgres is not available yet..\n"
	sleep 3
done

echo "Postgres ready. Initializing migrations."

echo "Running migrations with sqlx..."
sqlx migrate run --source ./db/migrations --database-url postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:5432/$POSTGRES_DB

echo "Successfully applied migrations"
