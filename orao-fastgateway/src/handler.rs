use crate::{data::*, db, DBPool, Result,
            calculate::{calcDeviation, std_deviation, mean},
            service::{getSec, get_latestVector, get_latestACalc, get_latestCalc, get_oneCalc}};
use warp::{reject, Reply};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use tokio::sync::{MutexGuard, oneshot, Mutex as TMutex};

// function to reply back with statistics of all available keys with timestamp
pub async fn get_stats_item_v2(vars: ReqStat, voracle_stat_data_arc: Arc<TMutex<Vec<OracleStatData>>>) -> Result<impl Reply> {
    println!("[Info] get_average_items_array_v2: {:?}", vars);
    // acquires lock to voracle_data getting a new pointer to data
    let oracle_stat_mutex = Arc::clone(&voracle_stat_data_arc);
    let voracle_stat_data = oracle_stat_mutex.lock().await;
    // search for the request key
    let mut a = StatAnswer::new();
    let keyv: u32 = vars.vector_id;
    for pos in 0..(voracle_stat_data.len()) {
        if keyv == voracle_stat_data[pos].key {
            a.key = voracle_stat_data[pos].key;
            a.min = voracle_stat_data[pos].min;
            a.max = voracle_stat_data[pos].max;
            a.deviation = voracle_stat_data[pos].deviation;
            a.timestamp = voracle_stat_data[pos].timestamp;
        }
    }
    println!("[Info] Answering Stats: {:?}", a);
    Ok(warp::reply::json(&a))
}

// function to reply back with average of all available keys for the xx last seconds set in environment variable ORAOAVERAGESEC
pub async fn get_average_item_v2(vars: ReqAverage, voracle_data_arc: Arc<TMutex<Vec<OracleData>>>) -> Result<impl Reply> {
    println!("[Info] get_average_items_array_v2");
    // get last seconds for average calculation
    let averagesec: u64 = match std::env::var("ORAOAVERAGESEC") {
        Ok(sec) => sec.parse().unwrap(),
        Err(_) => {
            300
        }
    };
    println!("[Info] Average Setting is set for the last {} seconds", averagesec);
    // acquires lock to voracle_data getting a new pointer to data
    let oracle_mutex = Arc::clone(&voracle_data_arc);
    let voracle_data = oracle_mutex.lock().await;
    // compute average
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - averagesec;
    let mut total: f64 = 0.0;
    let mut cnt: f64 = 0.0;
    let keyv: u32 = vars.vector_id;
    for pos in 0..(voracle_data.len()) {
        if voracle_data[pos].key == keyv && voracle_data[pos].timestamp >= t {
            total = total + voracle_data[pos].value;
            cnt = cnt + 1.0;
        }
    }
    let mut a = AverageAnswer::new();
    a.key = keyv;
    a.average = total / cnt;
    println!("[Info] Answering Key: {} Average: {}", keyv, total / cnt);
    Ok(warp::reply::json(&a))
}


pub async fn health_handler() -> Result<impl Reply> {
    let res = json!(ReqArrayStruc{vectors:vec!(1,2,3),protocol_id:[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],node_id:3,block_id:33});
    Ok(warp::reply::json(&res))
}

pub async fn get_list_all(store: OraoStore) -> Result<impl Reply> {
    let l22 = store.lock().await;
    Ok(warp::reply::json(&(*l22).requests))
}

pub async fn get_latestitems(v1: ReqStruc, store: OraoStore) -> Result<impl Reply> {
    let res = get_latestCalc(v1, store).await;
    Ok(warp::reply::json(&res))
}

pub async fn get_latestitemsArray(v1: ReqArrayStruc, store: OraoStore) -> Result<impl Reply> {
    println!("DEBUG! get_latestitemsArray");
    let res = get_latestACalc(v1, store).await;
    Ok(warp::reply::json(&res))
}

pub async fn get_one(v1: ReqStruc, store: OraoStore) -> Result<impl Reply> {
    let res = get_oneCalc(v1, store).await;
    let ores = warp::reply::json(&res);

    Ok(ores)
}

pub async fn log_response(response: warp::http::Response<warp::hyper::body::Bytes>) -> Result<impl Reply> {

    // println!("{:?}", response);
    Ok(response)
}

pub async fn get_latestvector(p1: u32, v1: u32, store: OraoStore) -> Result<impl Reply> {
    println!("vvv v1:{:?} p1:{:?} S:{:?}", v1, p1, (store));
    let res = get_latestVector(p1, v1, store).await;
    // let res="1231";
    Ok(warp::reply::json(&res))
}


