use postgres::error::Error;
use postgres::Connection;
use postgres::tls::openssl::openssl::ssl::{SslConnectorBuilder, SslMethod};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;
use openssl::ssl::*;

use crate::model::APN;
use crate::model::APN_CHAIN;

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
        .prepare(r#"
            SELECT apn_chr, apn, objectid, agencyname, agencyuniqueid,
            dwr_revise, region, apn_chr, county, crop2016, id, acres,
            ST_AsText(geometry) as geometry
            FROM blx_consolidated_apn where apn = $1
            "#
        )?;

    let apn: Option<APN> = statement.query(&[&identifier])?
        .iter()
        .fold(None, |_acc, row| {
            let apn_chr: Option<String> = row.get("apn_chr");
            let apn: Option<i64> = row.get("apn");
            let geometry: Option<String> = row.get("geometry");
            let object_id: Option<i64> = row.get("objectid");
            let agency_name: Option<String> = row.get("agencyname");
            let agency_unique_id: Option<i64> = row.get("agencyuniqueid");
            let dwr_revise: Option<String> = row.get("dwr_revise");
            let region: Option<String> = row.get("region");
            let acres: Option<f64> = row.get("acres");
            let county: Option<String> = row.get("county");
            let crop2016: Option<String> = row.get("crop2016");
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

pub fn get_apn_for_chain(identifier: i64, db: &mut Connection) -> Result<Option<APN_CHAIN>, Error> {
    let statement = db
        .prepare(r#"
            SELECT apn, agencyname
            FROM blx_consolidated_apn where apn = $1
            "#
        )?;

    let apn_chain: Option<APN_CHAIN> = statement.query(&[&identifier])?
        .iter()
        .fold(None, |_acc, row| {
            let apn: Option<i64> = row.get("apn");
            let agency_name: Option<String> = row.get("agencyname");

            Some(APN_CHAIN {
                apn,
                agency_name,
            })
        });

    Ok(apn_chain)
}