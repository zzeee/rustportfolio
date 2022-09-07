//487a18c0-9450-4813-a427-625fb90a2501
use futures::executor;
//future::ok as fut_ok;
//use futures::future::FutureResult;
use std::str::FromStr;
use web3::contract::tokens::Tokenize;
use web3::types::{TransactionRequest, Bytes, TransactionParameters, H256};
use web3::ethabi::Token;
use web3::futures::Future;
use secp256k1::SecretKey;
use crate::data::{BlockchainWriterLine, AddrType, OracleStatData};
use serde_json::json;
extern crate reqwest;

use reqwest::header;
use warp::body::json;


pub async fn sendTron(addresses: &Vec<&AddrType>, orao_stat_data_cp: &Vec<OracleStatData>, apikey: String, ownersadd: String) {
    for contract_address in (addresses).iter() {
        for val in orao_stat_data_cp.iter() {
            // println!("Writing: {:?}", val);
            let key = val.key;
            let value = ((val.average as f64) * 1000.0) as i64;
            let baseVal = 1000;
            let timestamp = val.timestamp;
            let data_pool = BlockchainWriterLine {
                network_id: contract_address.network_id,
                contract_address: contract_address.address.clone(),
                sec_key: "".to_string(),
                u1: 0,
                u2: 0,
                u3: key as i32,
                u4: value as i32,
                u5: baseVal,
                u6: timestamp as i32,
            };
                 write_tron_event(&data_pool,apikey.clone(),ownersadd.clone()).await;

        }
    }
}
        pub async fn write_tron_event(datapool:&BlockchainWriterLine,  apikey: String, ownersAddress:String) {
            let mut headers = header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());
            headers.insert("TRON-PRO-API-KEY", apikey.clone().parse().unwrap());
            let data = json!({
            "owner_address": ownersAddress,
            "contract_address":&*datapool.contract_address,
            "function_selector": "check2()",
            });



            let res = reqwest::Client::new()
                .post("https://api.trongrid.io/wallet/triggersmartcontract")
                .headers(headers)
                .body(data.to_string())
                .send().await.unwrap();
                //.text();
          //  println!("{}", res);
        }


