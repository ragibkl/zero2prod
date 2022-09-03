DB_USER=user
DB_PASSWORD=password
DB_PORT=5432
DB_NAME=newsletter

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
# sqlx database create
sqlx migrate run
