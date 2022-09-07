use crate::data::*; // {OraoLineV3,OraoExportLineV3, ReqArrayStrucV3, OraoExportLine,ProviderGap, Message, ProviderId ,ReqStruc, OraoDataLines, OraoArrayTimestamp, OraoStore, OraoLine, ReqArrayStruc};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use serde_json::json;

use tokio::{sync::{Mutex as TMutex}};


async  fn getStorage(store: OraoStore) -> Vec<OraoArrayTimestamp> {
    let l22 = store.lock().await;
    let storage = (*l22).requests.clone();
    storage
}
async  fn getStorageV3(store: OraoStoreV3) -> Vec<OraoArrayTimestampV3> {
    let l22 = store.lock().await;
    let storage = (*l22).requests.clone();
    storage
}

pub fn getValVal(tval: OraoDataLines, vector_id: u32, protocol_id: u32) -> OraoDataLines {
    let valperrequest = (tval).iter();
    let mut max: f64 = 0.0;
    let mut min: f64 = 0.0;
    let mut resArr: OraoDataLines = OraoDataLines::new();
    for lval in valperrequest {
        println!("Got: {}", lval);
        if lval.vector_id == vector_id && lval.protocol_id == protocol_id
        {
            let ind = lval.clone();
            if max > lval.value { max = lval.value; }
            if min > lval.value { min = lval.value; }
            resArr.push(ind);
        }
    }
    resArr
}

pub async fn get_oneCalc(v1: ReqStruc, store: OraoStore) -> Value {
    let vector_id = v1.vector_id;
    let protocol_id = v1.protocol_id;
    let _block_id = v1.block_id;
    let storage = getStorage(store).await;
    let storageI = storage.iter();
    let mut resArr: OraoDataLines = vec! {};
    let mut tsum: f64 = 0.0;
    let mut len: u32 = 0;
    let now = SystemTime::now();
    let mut max: f64 = 0.0;
    let mut min: f64 = 0.0;
    let mut lastT: u64 = 0;
    let mut lastP: usize = 0;
    let mut indd = 0;

    for (pos, val) in storageI.enumerate() {
        let tmpTime = val.timestamp;
        if tmpTime > lastT {
            lastT = tmpTime;
            lastP = pos;
        }
        println!("QQQ {:?}/{:?}", pos, tmpTime);
        let valperrequest = val.data.iter();
        for lval in valperrequest {
            println!("Got: {}", lval);
            if lval.vector_id == vector_id && lval.protocol_id == protocol_id
            {
                len += 1;
                let ind = lval.clone();
                tsum += lval.value;
                if max > lval.value { max = lval.value; }
                if min > lval.value { min = lval.value; }
                resArr.push(ind);
            }
            indd += 1;
        }
    }
    let mut average: f64 = 0.0;
    if len > 0 { average = (tsum as f64) / (len as f64); }
    let res: Value = json!({"protocol_id":protocol_id,"vector_id":vector_id, "value":average});

    let since_the_epoch = now
        .duration_since(UNIX_EPOCH);

    println!("AAAA {:?} {:?} {:?}", average, getSec(), getTDiff(now));
    res
}

pub async fn get_latestCalc(v1: ReqStruc, store: OraoStore) -> Value {
    let vector_id = v1.vector_id;
    let protocol_id = v1.protocol_id;
    let block_id = v1.block_id;
    let storage = getStorage(store).await;
    let storageI = storage.iter();
    let mut data = json!({"res":"nok"});
    for (pos, val) in storageI.enumerate() {
        let mut tmpTime = val.timestamp;

        let r2: Vec<OraoLine> = val.data.clone().into_iter().filter(|line| line.vector_id == vector_id && line.protocol_id == protocol_id).collect();
        let r3 = r2.last();
        data = json!(&r3);
    }
    data
}



pub async fn get_latestVector(protocol_id:u32, vector_id: u32, store: OraoStore) -> Value {
    let storage = getStorage(store).await;
    let storage_i = storage.iter();
    let mut data = json!({"res":"nok"});
    let m1= storage_i.max_by(|x, y| (x.timestamp).cmp(&y.timestamp));
    if let Some(y) = m1 {
       // println!("m1:{:?}",y.data);
        let res=y.data.clone().into_iter().find(|line| line.vector_id == vector_id && line.protocol_id == protocol_id);
        let provider=y.provider.clone();

        if let Some(yy)=res {
            let value:u64=(yy.value* 1000000.0).round() as u64;
            let res=OraoExportLine{provider, vector_id:yy.vector_id,protocol_id:yy.vector_id,valuebase:1000000, value};
            data = json!(&res);
        }
    }
    data
}

pub async fn write_event_queue(mdata:&Arc<TMutex<BlockchainWriterStore>>,data:BlockchainWriterLine)  {
      //  println!("Event queue, waiting for unlock");

let mut qdata=mdata.lock().await;
  //  let mut adata=(*qdata);
    println!("Adding to Event queue!! {:?} :{:?}",qdata.len(), data);
    (*qdata).push(data);
}
pub async fn get_latest_acalc_v3(v1: ReqArrayStrucV3, store: OraoStoreV3) -> Value {
 let vectors = v1.vectors;
    let storage = getStorageV3(store).await;
    let storage_i = storage.iter();
    let mut data = json!({"res":"nok"});
    let m1= storage_i.max_by(|x, y| (x.timestamp).cmp(&y.timestamp));
    if let Some(y) = m1 {
        let provider=y.provider.clone();
        let val=y.data.clone();
        let r2:Vec<OraoLineV3> = val.into_iter().filter(|line| vectors.contains(&line.vector_id)).collect();
        let r3:Vec<OraoExportLineV3>=r2.into_iter().map(|el|{
            let value:u64=(el.value* 1000000.0).round() as u64;
            let res=OraoExportLineV3{provider,vector_id:el.vector_id,protocol_id:el.protocol_id,valuebase:1000000, value};
            res
        }).collect();
        data = json!(&r3);
        println!("DEB RES!!!{:?}",data);
    }
    data

}

pub async fn get_latestACalc(v1: ReqArrayStruc, store: OraoStore) -> Value {
    println!("DEBUG: {:?}",json!(v1));
    let vectors = v1.vectors;
    let storage = getStorage(store).await;
    let storage_i = storage.iter();
    let mut data = json!({"res":"nok"});
    let m1= storage_i.max_by(|x, y| (x.timestamp).cmp(&y.timestamp));
    if let Some(y) = m1 {
        let provider=y.provider.clone();
        let val=y.data.clone();
        let r2:Vec<OraoLine> = val.into_iter().filter(|line| vectors.contains(&line.vector_id)).collect();
        let r3:Vec<OraoExportLine>=r2.into_iter().map(|el|{
            let value:u64=(el.value* 1000000.0).round() as u64;
            let res=OraoExportLine{provider,vector_id:el.vector_id,protocol_id:el.protocol_id,valuebase:1000000, value};
            res
        }).collect();
        data = json!(&r3);
        println!("DEB RES!!!{:?}",data);
    }
    data
}


pub fn getSec() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH);
    //  return since_the_epoch.as_secs();
    if let Ok(e) = since_the_epoch {
        // println!("MIC:{:?}",e.as_micros());
        return e.as_millis() as u64;
    }
    0
}

pub fn getTDiff(start: SystemTime) -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(start);
    //  return since_the_epoch.as_secs();

    if let Ok(e) = since_the_epoch {
        //println!("getTDiff MIC:{:?} {:?}",e.as_millis() as u64,e.as_millis() as u64);
        return e.as_micros() as u64;
    }
    0
}



use std::collections::HashSet;
use serde_json::value::Value::Null;
use std::sync::Arc;

/// # Process Stream
///
/// Remove all duplicates from stream and calculate average gap of providers.
///
pub fn process_stream(stream: Vec<Message>) -> (Vec<Message>, i32) {
    // Keep track of timestamp rounded to closest seconds that
    // were received by each provider.
    let mut visited_timestamps: HashSet<(ProviderId, i64)> = HashSet::new();
    let mut provider_gaps: ProviderGap = ProviderGap::new();
    let mut res = vec![];

    for datapoint in stream.iter() {
        let closest_second = to_closest_second(datapoint.timestamp);
        // Skip if this is a duplicate or an error point.
        if visited_timestamps.contains(&(datapoint.provider_id, closest_second))
            || !is_within_interval(datapoint.timestamp) // Is a bad line
        {
            provider_gaps.increment(datapoint.provider_id);
            continue;
        }
        // Save this datapoint.
        visited_timestamps.insert((datapoint.provider_id, closest_second));
        res.push(datapoint.clone());
    }

    return (res, provider_gaps.maximum());
}

/// Returns the closest second a timestamp belongs to based on distance.
///
/// Eg:
///     999ms -> 1000ms
///     1001ms -> 1000ms
fn to_closest_second(timestamp: i64) -> i64 {
    let lower_bound = to_closest_elapsed_second(timestamp);
    let dist_to_lower_bound = timestamp - lower_bound;

    let upper_bound = to_closest_elapsed_second(timestamp) + 1000;
    let dist_to_upper_bound = upper_bound - timestamp;
    if dist_to_upper_bound > dist_to_lower_bound {
        return lower_bound;
    }
    upper_bound
}

// Returns true if timestamp fails within +/- 100ms (inclusive)
fn is_within_interval(timestamp: i64) -> bool {
    let closest_second = to_closest_elapsed_second(timestamp);
    closest_second - 100 <= timestamp && timestamp <= closest_second + 100
}

/// Convert a timestamp to the closest ELAPSED seconds.
fn to_closest_elapsed_second(second: i64) -> i64 {
    second / 1000 * 1000
}

