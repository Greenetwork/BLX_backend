use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct APN {
    pub apn_chr: String,
    pub apn: i64,
    pub geometry: Vec<u8>,
    pub object_id: i64,
    pub agency_name: String,
    pub agency_unique_id: String,
    pub dwr_revise: String,
    pub region: String,
    pub acres: f64,
    pub county: String,
    pub crop2016: String,
    pub id: i32,
}

pub struct Storage {
    pub database: Pool<PostgresConnectionManager>,
}
