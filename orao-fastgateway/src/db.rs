use crate::{error, error::Error::*, DBCon, DBPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use mobc::{Connection, Pool};
use std::time::Instant;
use std::str::FromStr;
use std::time::Duration;
use futures::channel::mpsc;




use tokio_postgres::{
    AsyncMessage, Client, Config, Connection as TConnection,NoTls, Error, IsolationLevel, SimpleQueryMessage,
};
use futures::{
    future, join, pin_mut, stream, try_join, FutureExt, SinkExt, StreamExt, TryStreamExt,
};
pub type Result<T> = std::result::Result<T, error::Error>;

// database settings 
const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

// function to return a connection to the database.
pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    let res = db_pool.get().await.map_err(DBPoolError);

    res
}

pub async fn get_db_con2(db_pool: &DBPool) -> Connection<PgConnectionManager<NoTls>> {
    let res = db_pool.get().await.map_err(DBPoolError).unwrap();
    res
}

// function to create the database connection (Postgres)
pub fn create_pool() -> std::result::Result<DBPool, Error> {
    // trying to read the postgres connection string from environment variable
    // settings default one if not found
    let postgresqlconnstring = match std::env::var("ORAODB") {
        Ok(dbs) => dbs,
        Err(_) => {
          //  String::from("postgres://orao:orao4oraoandrey@localhost:5432/orao")
            String::from("postgres://orao:KtjjHh12io!S@localhost:5434/orao_server")
        }
    };
    println!("[Info] Database main: {}", postgresqlconnstring);
    let config = Config::from_str(postgresqlconnstring.as_str())?;
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

// function to get data provider id
pub async fn get_provider_id(db_pool: &DBPool, search: String) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let qq = con.query_one("select * from oracles where key=$1", &[&search]).await;
    let mut res: i32 = 0;
    if let Ok(v) = qq {
        res = v.get(0);
        let r2: Option<String> = v.get("key");
        let id: i32 = v.get("id");
        println!("QQQ {:?} {:?} {:?}", res, r2, id);
        res = id;
    } else {
        if let Err(_q) = qq {}
    }
    Ok(res as u64)
}
/*
pub async fn get_contract_addresses(db_pool: &DBPool) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let qq = con.query_one("select * from oracles where key=$1", &[&search]).await;
    let mut res: i32 = 0;
    if let Ok(v) = qq {
        res = v.get(0);
        let r2: Option<String> = v.get("key");
        let id: i32 = v.get("id");
        println!("QQQ {:?} {:?} {:?}", res, r2, id);
        res = id;
    } else {
        if let Err(_q) = qq {}
    }
    Ok(res as u64)
}
*/


// function to store new item received from data provider
pub async fn store_new_item_from_provider(db_pool: &DBPool, providerid: String, key: u32, value: f64) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let provideridcs = providerid.clone();
    let provideridc = provideridcs.as_str();
    let keyc = key.clone();
    let valuec = value.clone();
    let r = match con.execute("INSERT INTO items (provider_id,key,value) VALUES ($1,$2,$3)", &[&provideridc, &keyc, &valuec]).await {
        Ok(nr) => nr,
        Err(e) => {
            println!("[Error] Error inserting a new item {}", e);
            0
        }
    };
    Ok(r as u64)
}


