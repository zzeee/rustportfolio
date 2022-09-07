use std::sync::{Arc,Mutex};
use crate::{data::*};
pub fn calcDeviation(tc:u64, keyv:u32, voracle_data:Vec<OracleData>)->calcDev {
     let mut total:f64=0.0;
        let mut min:f64=0.0;
        let mut max:f64=0.0;
        let mut cnt:f64=0.0;
    let mut result = Vec::<f64>::new();
    for posc in 0..(voracle_data.len()) {
        if voracle_data[posc].key == keyv && voracle_data[posc].timestamp >= tc {

            total = total + voracle_data[posc].value;
            if voracle_data[posc].value > max {
                max = voracle_data[posc].value;
            }
            if voracle_data[posc].value < min || min == 0.0 {
                min = voracle_data[posc].value;
            }
            cnt = cnt + 1.0;
            result.push(voracle_data[posc].value);
        }

    }

    calcDev{min, max, cnt, total, result }
}


// function to calculate the standard deviation of an array of f64 numbers
pub fn std_deviation(data: Vec<f64>) -> Option<f64> {
    let d=data.clone();
    match (mean(d), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);
                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}


//function to calculate the average for f64 numbers
pub fn mean(data: Vec<f64>) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();
    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}