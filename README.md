# BLX Backend

## How to build via Docker

```
docker build . -t blx_backend
```

## How to run via Docker

```
docker run --rm -e PG_POOL_SIZE=1 -e PG_URL=posgres://databaseurl/database -p 8000:8000 blx
```

## Endpoints

```
curl localhost:8000/apn/317053
```

## How is the project setup

- Project uses Rocket, Postgres, and R2D2 crates.
- Endpoint definitions are found in `src/main.rs`
- Database access is found in `src/db/mod.rs`
- Project is initialised in `src/main.rs` and 
- To add new endpoints, follow the format found in `src/main.rs`
- To add more DB methods, follow the format found in `src/db/mod.rs`
- To create model mappings, follow the format found in `src/model/mod.rs`
