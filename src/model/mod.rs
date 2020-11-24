use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct APN {
    pub apn_chr: Option<String>,
    pub apn: Option<i64>,
    pub geometry: Option<String>,
    pub object_id: Option<i64>,
    pub agency_name: Option<String>,
    pub agency_unique_id: Option<i64>,
    pub dwr_revise: Option<String>,
    pub region: Option<String>,
    pub acres: Option<f64>,
    pub county: Option<String>,
    pub crop2016: Option<String>,
    pub id: i32,
}

pub struct Storage {
    pub database: Pool<PostgresConnectionManager>,
}
