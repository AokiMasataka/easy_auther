# Usage

## set DB URL
```shell
export DATABASE_URL=postgres://pgsql:pgsql@localhost/pgsql
```

## update DB schema

```shell
docker compose up -d
cargo sqlx migrate
cargo sqlx prepare
```
