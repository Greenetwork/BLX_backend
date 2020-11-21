use postgres::error::Error;
use postgres::Connection;
use postgres::tls::openssl::openssl::ssl::{SslConnectorBuilder, SslMethod};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;
use openssl::ssl::*;

use crate::model::APN;

pub fn get_database_url() -> String {
    env::var("PG_URL").expect("PG_URL must be set")
}

pub fn get_client() -> Pool<PostgresConnectionManager> {
    let mut connbuilder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    connbuilder.set_verify(postgres::tls::openssl::openssl::ssl::SSL_VERIFY_NONE);
    let negotiator = postgres::tls::openssl::OpenSsl::from(connbuilder.build());
    let manager = PostgresConnectionManager::new(
        get_database_url().as_ref(), 
        TlsMode::Require(Box::new(negotiator)),
    ).unwrap();
    let pool_size: u32 = env::var("PG_POOL_SIZE").expect("PG_POOL_SIZE must be set").parse::<u32>().unwrap();
    Pool::builder().max_size(pool_size).build(manager).unwrap()
}

pub fn get_apn(identifier: i64, db: &mut Connection) -> Result<Option<APN>, Error> {
    let statement = db
        .prepare(
            "select * from blx_consolidated_apn where apn = $1 ",
        )?;

    let apn: Option<APN> = statement.query(&[&identifier])?
        .iter()
        .fold(None, |_acc, row| {
            let apn_chr: String = row.get("apn_chr");
            let apn: i64 = row.get("apn");
            let geometry: Vec<u8> = row.get("geometry");
            let object_id: i64 = row.get("objectid");
            let agency_name: String = row.get("agencyname");
            let agency_unique_id: String = row.get("agencyuniqueid");
            let dwr_revise: String = row.get("dwr_revise");
            let region: String = row.get("region");
            let acres: f64 = row.get("acres");
            let county: String = row.get("county");
            let crop2016: String = row.get("crop2016");
            let id: i32 = row.get("id");

            Some(APN {
                apn_chr,
                apn,
                geometry,
                object_id,
                agency_name,
                agency_unique_id,
                dwr_revise,
                region,
                acres,
                county,
                crop2016,
                id,
            })
        });

    Ok(apn)
}

