# BLX Backend

## How to build via Docker

```
docker build . -t blx_backend
```

## How to run via Docker

```
docker run --rm -e PG_POOL_SIZE=1 -e PG_URL=postgres://databaseurl/database -p 8000:8000 blx_backend
```

## Endpoints
when accessed return .json

### [front end](https://github.com/Greenetwork/BLX_frontend) usage:
```
curl localhost:8000/apn/317053
```
returns =       {  
    pub apn_chr: Option\<String>,  
    pub apn: Option\<i64>,  
    pub geometry: Option\<String>,  
    pub object_id: Option\<i64>,  
    pub agency_name: Option\<String>,  
    pub agency_unique_id: Option\<i64>,  
    pub dwr_revise: Option\<String>,  
    pub region: Option\<String>,  
    pub acres: Option\<f64>,  
    pub county: Option\<String>,  
    pub crop2016: Option\<String>,  
    pub id: i32,  
}

### [chain](https://github.com/spencerbh/BLX_chain_future) usage:
```
curl localhost:8000/apn/chain/317053
```
returns =       {  
    pub apn: Option\<i64>,  
    pub agency_name: Option\<String>,  
}

## How is the project setup

- Project uses Rocket, Postgres, and R2D2 crates.
- Endpoint definitions are found in `src/main.rs`
- Database access is found in `src/db/mod.rs`
- Project is initialised in `src/main.rs` and 
- To add new endpoints, follow the format found in `src/main.rs`
- To add more DB methods, follow the format found in `src/db/mod.rs`
- To create model mappings, follow the format found in `src/model/mod.rs`
