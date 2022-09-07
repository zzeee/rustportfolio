use crate::{DBCon, db,data};
// use tracing::*;
// use tokio::runtime::Handle;

// use bytes::{Bytes, BytesMut};
use futures::channel::mpsc;
use futures::{stream, FutureExt, SinkExt, StreamExt, TryStreamExt, task};
// use std::fmt::Write;
// use std::time::{Duration, Instant};
use tokio::net::TcpStream;
// use tokio::time;
// use tracing_subscriber;

// use tokio_postgres::error::SqlState;
use tokio_postgres::tls::{NoTls, NoTlsStream};
// use tokio_postgres::types::{Kind, Type};
use tokio_postgres::{
    AsyncMessage, Client, Config, Connection, Error,
};
// use tokio::process::Command;
use tokio::task::JoinHandle;
use std::task::{Poll, Context};
//use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc::{Sender};
use std::future::Future;
use tokio::future;
use std::sync::mpsc::{channel, SendError};
use tokio::sync::oneshot;
use std::pin::Pin;
use std::collections::HashMap;

const LAUNCHED: i32 = 10;
const UTABLE_UPDATED: i32 = 12;

async fn connect_raw(s: &str, s2: &str) -> Result<(Client, Connection<TcpStream, NoTlsStream>), Error> {
    let socket = TcpStream::connect(s2).await.unwrap();
    let config = s.parse::<Config>().unwrap();
    config.connect_raw(socket, NoTls).await
}

pub async fn notifications(mut txx: Sender<i32>) -> JoinHandle<()> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();


    let server_addr_string = match std::env::var("ORAO_DBSRV") {
        Ok(dbs) => dbs,
        Err(_) => {
            String::from("127.0.0.1:5434")
        }
    };

    let postgresqlconnstring = match std::env::var("ORAODB") {
        Ok(dbs) => dbs,
        Err(_) => {
            // String::from("postgres://orao:orao4oraoandrey@127.0.0.1:5432/orao")
            println!("default local database");
            String::from("postgres://orao:KtjjHh12io!S@127.0.0.1:5434/orao_server")
        }
    };
    txx.send(LAUNCHED).await;

    println!("Starting database listener! {:?} {:?}", postgresqlconnstring, server_addr_string);
    let (client, mut connection) = connect_raw(&postgresqlconnstring, &server_addr_string).await.unwrap();
    // println!("Database listener has started!2 {:?} {:?}", postgresqlconnstring, server_addr_string);
    let (tx, rx) = mpsc::unbounded();
    let stream = stream::poll_fn(move |cx| {
        let cpoll = connection.poll_message(cx);
        if let Poll::Ready(Some(Ok(AsyncMessage::Notification(note)))) = &cpoll {
            println!("UTABLE_UPDATED {:?}", note.payload());
            let mut qxxx = txx.clone();
            tokio::task::spawn(async move {
                qxxx.send(UTABLE_UPDATED).await;
            });
        }
        cpoll
    }).map_err(|e| panic!(e));


    let connection = stream.forward(tx).map(|r| { r.unwrap() });
    println!("PG Listener ..");
    let rt: JoinHandle<()> = tokio::spawn(connection);

    client.execute("LISTEN test_notifications", &[]).await.unwrap();
    let notifications = rx.collect::<Vec<_>>().await;
    rt
}


pub async fn writeTransactionResult(con: DBCon, networkId: i32, address: &str, answer: &str, transactionId: &str) -> Result<u64, Error> {
    println!("!!!! writeTransactionResult {:?} {:?}", answer, transactionId);
    let res = con.execute(r#"insert into transactions("networkId",address,answer,transaction_id)values($1,$2,$3,$4)"#, &[&networkId, &address, &answer, &transactionId]).await;
    res
}

pub async fn get_list_of_addresses(con: DBCon, param: i32) -> db::Result<Vec<String>> {
    println!("[Info] Orao - updating... ");
    //let con = db::get_db_con(db_pool).await?;
    let query: String = format!(r#"select "userNetworkAddress" from requests where "networkId"=$1  "#);

    let aparam = "1".to_string();
    let result = con.query(query.as_str(), &[&param]).await.unwrap();


    /* for val in result.iter() {
         let dd=val.columns();
                 println!("Got: {:?}", val );

     }*/
    let flt: Vec<String> = result.iter().map(|e| e.get(0)).collect();
    println!("updating:ddd {:?} ", flt);
    Ok(flt)
}

pub async fn get_list_of_addresses2(con: DBCon) -> db::Result<Vec<data::AddrType>> {
    println!("------ ");
    //let con = db::get_db_con(db_pool).await?;
    let query: String = format!(r#"select "networkId","userNetworkAddress" from requests  "#);

    let result = con.query(query.as_str(), &[]).await.unwrap();

    println!("PR {:?}", result);


    let mut address_base:Vec<data::AddrType> = Vec::new();

    for line in result.iter() {
        let network_id:i32=line.get(0);
        let address:String=line.get(1);
        let ins: data::AddrType = data::AddrType {network_id,address};
       // println!("QQQ:{:#?} {:?}",network_id,address);
       address_base.push(ins);
    }

    //println!("SSS {:?}",address_base);
    //  let mut scores = HashMap::new();


//    let flt: Vec<String> = result.iter().map(|e| e.get(0)).collect();
  //  println!("updating:ddd {:?} ", flt);
    Ok(address_base)
}
