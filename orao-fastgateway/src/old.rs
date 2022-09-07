use std::sync::{Arc,Mutex};
use crate::{data::*};
use warp::{reject, Reply};
use serde_json::json;
use std::time::SystemTime;
use crate::{data::*, db, DBPool, Result,
             service::{getSec},
            calculate::{calcDeviation, std_deviation, mean},

};

pub async fn add_list_item(item: OraoArray, db_pool: DBPool, store: OraoStore) -> Result<impl Reply> {
    let mut provider_code = (String::from_utf8_lossy(&item.provider)).to_string();
    provider_code.truncate(47); //  TODO check if 47 is correct(!)
    println!("[Info] providerCode {:?} ", provider_code);

    let provider_id = db::get_provider_id(&db_pool, provider_code)
        .await
        .map_err(|e| reject::custom(e))?;
    println!("[Info] provider_id {:?} ", provider_id);

    let mut l22 = store.lock().await;
    (*l22).counter += 1;
    let line: OraoArrayTimestamp = OraoArrayTimestamp { data: item.data.clone(), provider: provider_id, timestamp: getSec() };
    (*l22).requests.push(line);
    println!("CRES {} {:?}", (*l22).requests.len(), (*l22).counter);
    Ok(warp::reply::json(&(*l22).requests))
}
// function to add new key/values from a data provider, keeping in share memory the data

pub async fn add_list_item_v2(item: OraoArrayV2, db_pool: DBPool, voracle_data_arc: Arc<Mutex<Vec<OracleData>>>, voracle_stat_data_arc: Arc<Mutex<Vec<OracleStatData>>>) -> Result<impl Reply> {
    // get provider id in hex decimal format from the binary received
    let providerid = hex::encode(item.provider);
    // store data in the database before the mutex locking to avoid a race condition with postgresql
    /* for pos in 0..item.data.len(){
        let r=db::store_new_item_from_provider(&db_pool,providerid.clone(),item.data[pos].vector_id,item.data[pos].value)
        .await
        .map_err(|e| reject::custom(e))?;
    } */
    // acquires lock to voracle_data getting a new pointer to data
    let oracle_mutex = Arc::clone(&voracle_data_arc);
    let mut voracle_data = oracle_mutex.lock().unwrap();
    // acquires lock to voracle_stat_data getting a new pointer to data
    let oracle_stat_mutex = Arc::clone(&voracle_stat_data_arc);
    let mut voracle_stat_data = oracle_stat_mutex.lock().unwrap();
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
            for poss in 0..voracle_stat_data.len() - 1 {
                if voracle_stat_data[pos].key == item.data[pos].vector_id {
                    voracle_stat_data.remove(pos);
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
        for pos in 0..voracle_data.len() - 1 {
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
