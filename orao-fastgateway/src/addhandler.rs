use std::sync::{Arc,Mutex};
use crate::{data::*};
use warp::{reject, Reply};
use serde_json::json;
use std::time::SystemTime;
use crate::{data::*, db, DBPool, Result,
            calculate::{calcDeviation, std_deviation, mean},
    service::{get_latestACalc,get_latest_acalc_v3}

};
use tokio::sync::{MutexGuard, oneshot, Mutex as TMutex};


pub async fn add_list_item_v3(item: OraoArrayV3, db_pool: DBPool, voracle_data_arc: Arc<TMutex<Vec<OracleData>>>, voracle_stat_data_arc: Arc<TMutex<Vec<OracleStatData>>>) -> Result<impl Reply> {
    // get provider id in hex decimal format from the binary received
    let providerid = hex::encode(item.provider);
    // store data in the database before the mutex locking to avoid a race condition with postgresql
    /* for pos in 0..item.data.len(){
        let r=db::store_new_item_from_provider(&db_pool,providerid.clone(),item.data[pos].vector_id,item.data[pos].value)
        .await
        .map_err(|e| reject::custom(e))?;
    } */
    // println!("add_list_item_v3 {:?}", item);
    // acquires lock to voracle_data getting a new pointer to data
    let oracle_mutex = Arc::clone(&voracle_data_arc);
    //expected `Mutex<Vec<OracleData, Global>>`, found `MutexGuard<Vec<<unknown>, Global>>`
    let mut voracle_data = oracle_mutex.lock().await;
    // acquires lock to voracle_stat_data getting a new pointer to data
    let oracle_stat_mutex = Arc::clone(&voracle_stat_data_arc);
    let mut voracle_stat_data = oracle_stat_mutex.lock().await;
    // get averagesec info from environment variable
    let averagesec: u64 = match std::env::var("ORAOAVERAGESEC") {
        Ok(sec) => sec.parse().unwrap(),
        Err(_) => {
            300
        }
    };

    // get system epoch time
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    // load the key/values received in voracle_data
    for pos in 0..item.data.len() {
        voracle_data.push(OracleData { key: item.data[pos].vector_id, value: item.data[pos].value, timestamp: t });
        println!("[Info] Loaded Key:{:?} Value: {:?}", item.data[pos].vector_id, item.data[pos].value);
        println!("State after load: {:?}", voracle_data.len());
        // start computing average,min, max and deviation
        let tc = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - averagesec;
        let mut total: f64 = 0.0;
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
        let mut cnt: f64 = 0.0;
        let keyv: u32 = item.data[pos].vector_id;
        let mut datadeviation = Vec::<f64>::new();
        //calculate array of deviations standard deviation
        let res: calcDev = calcDeviation(tc, keyv, (*voracle_data).clone());
        datadeviation = res.result;
        min = res.min;
        max = res.max;
        cnt = res.cnt;
        total = res.total;

        // start updating OracleStatData
        // remove previous record (if any)
        if voracle_stat_data.len() > 0 {
          //  println!(" {:?} {:?}",voracle_stat_data.len(),0);
            for poss in 0..voracle_stat_data.len()-1  {
                println!("67: {:?} {:?} {:?} {:?}",poss,pos,voracle_stat_data.len(),item.data.len());
                if voracle_stat_data[poss].key == item.data[pos].vector_id {
                    voracle_stat_data.remove(poss);
                }
            }
        }
        //compute standard deviation
        let mut deviation: f64 = 0.0;
        if datadeviation.len() > 1 {
            deviation = match std_deviation(datadeviation) {
                Some(dv) => dv,
                None => 0.0,
            };
        }
        // add new record in OracleStatData
        voracle_stat_data.push(OracleStatData { key: item.data[pos].vector_id, average: total / cnt, min: min, max: max, deviation: deviation, timestamp: t });
        //end computing average, min, max and deviation
    }
    // remove expired items
    if voracle_stat_data.len() > 0 {
        for pos in 0..voracle_data.len()  {
            println!("voracle_stat_data88 {:?} {:?}",voracle_data.len(),pos);
            if voracle_data[pos].timestamp < (t - averagesec) {
                voracle_data.remove(pos);
            }
        }
    }
    //end process to remove expired item
    // lock is released automatically when the pointer goes out of scope
    // sending empyt answer with positive Status 200
    let resp = r#"{"result": "OK"}"#;
    Ok(warp::reply::json(&resp))
}

pub async fn get_allarray(v1: ReqDirect,store: OraoStoreV3 ) -> Result<impl Reply> {
    let l22 = store.lock().await;
    Ok(warp::reply::json(&(*l22).requests))
}
pub async fn get_latestitems_array_v3(v1: ReqArrayStrucV3,voracle_stat_data_arc: Arc<TMutex<Vec<OracleStatData>>>,store: OraoStoreV3 ) -> Result<impl Reply> {
    println!("DEBUG! get_latestitemsArray {:?}",voracle_stat_data_arc);
    let res = get_latest_acalc_v3(v1, store).await;
    Ok(warp::reply::json(&res))
}