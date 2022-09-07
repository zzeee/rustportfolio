// Orao Fast Gateway 1.0 - Data agnostic Oracle for blockchains

#[macro_use]
extern crate serde_big_array;
big_array! { BigArray; }

mod data;
mod db;
mod routes;
mod db2;
mod tron;
mod service;
mod calculate;
mod addhandler;
mod old;
mod error;
mod handler;
mod web3_plasm;
mod web3_eth;

use web3_plasm::*;
use tron::*;
use web3_eth::*;
use warp::{Rejection};
use std::{sync::{Arc}, time::{Duration, Instant}};
use tokio::{process, sync::{Mutex as TMutex}, task, time, sync::mpsc::{Sender, Receiver}, time::delay_for};
use crate::{data::*, DBCon, DBPool};
use std::thread;

type Result<T> = std::result::Result<T, Rejection>;

// main function that listens to port tcp/8000 and routes the http calls
#[tokio::main]
async fn main() {
    println!("[Info] Orao - starting... ");
    let (mut tx, mut channel_receiver): (Sender<i32>, Receiver<i32>) = tokio::sync::mpsc::channel(1);
    let db_pool = db::create_pool().expect("[Fatal Error] database connection pool cannot be created.");
    let db_pool3 = db_pool.clone();
    let db_pool4 = db_pool.clone();
    let conn1_1 = db::get_db_con(&db_pool.clone()).await;
    if let Ok(conn1)=conn1_1 {
        // let conn2 = db::get_db_con(&db_pool.clone()).await.unwrap();

        let mut tvec: Vec<AddrType> = Vec::new();

        let contractAddresses = Arc::new(TMutex::new(tvec));
        let contractAddresses_c = contractAddresses.clone();

        {
            let mut cAddress = contractAddresses.lock().await;
            *cAddress = db2::get_list_of_addresses2(conn1).await.unwrap();
        }
        let plasm_seckey = std::env::var("ORAO_PLASM_PK").unwrap_or("".to_string());
        let eth_seckey = std::env::var("ORAO_ETH_PK").unwrap_or("".to_string());
        let tron_apikey = std::env::var("ORAO_TRON_APIKEY").unwrap_or("4471a89f-3566-425c-8beb-cb4896307521".to_string());
        let tron_owners_addr = std::env::var("ORAO_TRON_OWNERADDR").unwrap_or("417c6762a6bad35903b1ee3ae635b0c1e1757abcb1".to_string());


        let voracle_data_arc = Arc::new(TMutex::new(Vec::new()));
        let plasm_events_queue: Arc<TMutex<BlockchainWriterStore>> = Arc::new(TMutex::new(BlockchainWriterStore::new()));
        let plasm_events_copy = plasm_events_queue.clone();

        let voracle_stat_data_arc = Arc::new(TMutex::new(Vec::new()));
        let voracle_stat_data_arc2 = (&voracle_stat_data_arc).clone();
        let store_sv3 = Arc::new(TMutex::new(data::OraoStoreSV3::new()));
        let store_sv3_v3 = (store_sv3).clone();
        // let store = Arc::new(TMutex::new(data::OraoStoreS::new()));


        let mut interval2 = time::interval(Duration::from_millis(60000));
        let tw1 = task::spawn(async move {
            let mut thread_tx = tx.clone();
            println!("update listener!");
            let _res = db2::notifications(thread_tx).await;
        });

        /* let tw5 = tokio::spawn(async move {
         let mut interval3 = time::interval(Duration::from_millis(20000));
         let mut ii: i32 = 0;
         loop {
             interval3.tick().await;
             println!("TEST5TICK {:?}", ii);
             ii += 1;
         }
     });*/
        let tw3 = tokio::spawn(async move {
            println!("Queue module spawned");
            let mut interval_reg = time::interval(Duration::from_millis(6000));

            loop {
                interval_reg.tick().await;
                println!("Web3 loop tick20");
                let rt = store_sv3_v3.lock().await;
                let orao_stat_data = (*voracle_stat_data_arc2).lock().await;
                let orao_stat_data_cp: &Vec<OracleStatData> = &*orao_stat_data;
                let all_addresses = contractAddresses.lock().await;

                println!("ropsten_seckey {:?} {:?}", eth_seckey, eth_seckey.len());
                if eth_seckey.len() > 0 {
                    println!("eth");
                    let ethereum_addresses: Vec<&AddrType> = (*all_addresses).iter().filter(|e| e.network_id == 2).collect();
                    //  let when = Instant::now() + Duration::from_millis(10);
                    //    Delay::new(when).await;
                    delay_for(Duration::from_millis(20000)).await;
                    sendEth(&ethereum_addresses, orao_stat_data_cp, eth_seckey.clone()).await;
                } else {
                                        println!("noeth!");

                }
                if plasm_seckey.len() > 0 {
                                        println!("plasm");

                    let plasm_addresses: Vec<&AddrType> = (*all_addresses).iter().filter(|e| e.network_id == 1).collect();
                    sendPlasm(&plasm_addresses, orao_stat_data_cp, plasm_seckey.clone(), plasm_events_queue.clone()).await;
                }
                else {
                                        println!("noplasm");

                }
                if false && tron_apikey.len() > 0 && tron_owners_addr.len() > 0 {
                    println!("tron");
                    let tron_addresses: Vec<&AddrType> = (*all_addresses).iter().filter(|e| e.network_id == 3).collect();
                    sendTron(&tron_addresses, orao_stat_data_cp, tron_apikey.clone(), tron_owners_addr.clone()).await;
                }  else {
                                        println!("notron");

                }
            }
        });
        println!("[Info] Orao - Fast Gateway preparing");

        let tw9 = tokio::spawn(async move {
            let mut interval3 = time::interval(Duration::from_millis(1000));
            let mut blocked_1 = false;
            let mut blockcounter: i32 = 0;
            let mut ii: i32 = 0;

            loop {
                interval3.tick().await;
                // println!("Write plasm queeu: launch counter{:?}, orao blocked: {:?} blockcounter: {:?}", ii, blocked_1, blockcounter);
                let conn2 = db::get_db_con(&db_pool4.clone()).await.unwrap();
                let mut qq = plasm_events_copy.lock().await;
                let len = qq.len();
                if blocked_1 { blockcounter = blockcounter + 1; }
                if blockcounter > 200 { blocked_1 = false; }
                if len > 0 && !blocked_1 {
                    println!("Write plasm queeu: launch counter{:?}, orao blocked: {:?} blockcounter: {:?}", ii, blocked_1, blockcounter);

                    let line = (*qq).pop().unwrap();
                    if line.network_id == 1 {
                        blocked_1 = true;
                        let callResult = write_plasm_event_wrapped(&line).await;
                        if let Ok(resu) = callResult {
                            println!("Transaction ID: {:?}", resu);
                            let transactionstr2 = format!("{:#x}", resu);
                            db2::writeTransactionResult(conn2, 1, &*line.contract_address, "", &*transactionstr2).await;
                            blocked_1 = false;
                        } else if let Err(resu) = callResult {
                            println!("Err ID: {:?}", resu);
                            db2::writeTransactionResult(conn2, 1, &*line.contract_address, &*resu.to_string(), "").await;
                        }
                    } else {
                        println!("Skipping writing of transaction due to waiting of previous one");
                    }
                }
                // println!("TEST TICK {:?} LEN:{:?}", ii, len);
                ii += 1;
            }
        });
        let tw2 = tokio::spawn(async move {
            println!("Listener module spawned");
            loop {
                println!("Waiting update signal:");
                let ares = channel_receiver.recv().await.unwrap();
                println!("RECEIVED UPDATE SIGNAL: {:?}", ares);
                { // updating list
                    let mut cAddress = contractAddresses_c.lock().await;
                    let conn3 = db::get_db_con(&db_pool).await.unwrap();
                    *cAddress = db2::get_list_of_addresses2(conn3).await.unwrap();
                }
            }
        });


        println!("[Info] Orao - Fast Gateway 1.03 is started and waiting for connections on port tcp/8000");
        //let tw4=tokio::spawn(async move {
        let _ = routes::getroutes(db_pool3, voracle_data_arc, voracle_stat_data_arc, store_sv3/*,store_filter,store_filter_sv3*/).await;
        //});
    } else {

        println!("Fail! There is no database connection");
    }
}


