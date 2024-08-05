# Product Availability Matrix (PAM)

- Rust based graphql backend
- React MUI based front end
- PostgresQL persistence

## Setup the env

Create a `env` file in the project root with the following

```bash
POSTGRES_USER='<your user>'
POSTGRES_PASSWORD='<your password>'
POSTGRESL_SCHEMA='availability_matrix'
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost/${POSTGRESL_SCHEMA}
```

## Setup Postgres

- Launch Postgres.  You can use the included `docker-compose-postgres.yml`

```bash
docker compose -f docker-compose-postgres.yml up -d
```

- Use the DB Admin tool of your choice to create a databased called `availability_matrix`

- Use the method of your choice to run the SQL script `init_db_postgres.qsl`

Maybe when the dust settles on the schema I'll auto load it in the docker-compose file.

## Run the backend

```bash
cargo run
```

## Browse to the Playground at `localhost:8080/api/graphiql`

## Browse to the React based front end at `locahost:8080/`
