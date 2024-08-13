# Product Availability Matrix (PAM)

- Rust based graphql backend
- React MUI based front end
- PostgresQL persistence

## Setup the env

Create a `env` file in the project root with the following

```bash
POSTGRES_USER='<your user>'
POSTGRES_PASSWORD='<your password>'
POSTGRES_SCHEMA='availability_matrix'
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost/${POSTGRESL_SCHEMA}
```

## Setup Postgres

- Launch Postgres.  You can use the included `docker-compose.yml`

```bash
docker compose -f docker-compose-postgres.yml up -d
```

The docker-compose script links an init SQL.  So, if it succeeds, you're good to go.

## Run the backend

```bash
cargo run
```

## Browse to the Playground at `localhost:8080/api/graphiql`

## Browse to the React based front end at `locahost:8080/`
