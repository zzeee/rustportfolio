use warp::{ Filter};
// use crate::service::*;
use crate::addhandler::*;
use crate::handler::*;
use crate::{data::*,  DBPool};
// use std::iter::Map;
use std::convert::Infallible;
use std::sync::{Arc};
use tokio::sync::{Mutex as TMutex};

// function to duplicate the db handle
fn with_db(db_pool: DBPool) -> impl Filter<Extract=(DBPool, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn getroutes(db_pool: DBPool, voracle_data_arc: Arc<TMutex<Vec<OracleData>>>, voracle_stat_data_arc: Arc<TMutex<Vec<OracleStatData>>>,
                       store_sv3: Arc<TMutex<OraoStoreSV3>> /*, store: Arc<Mutex<OraoStore>>, */) {
    let oracle_mutex_filter = warp::any().map(move || voracle_data_arc.clone());
    let oracle_stat_mutex_filter = warp::any().map(move || voracle_stat_data_arc.clone());
  //  let store_filter = warp::any().map(move || store.clone());
    let store_filter_sv3 = warp::any().map(move || (&store_sv3).clone());
    let add_items_v3 = warp::post()
        .and(warp::path("v3")).and(warp::path("update_structured"))
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and(oracle_mutex_filter.clone())
        .and(oracle_stat_mutex_filter.clone())
        .and_then(add_list_item_v3);
   let add_ipfsitems_v3 = warp::post()
        .and(warp::path("v3")).and(warp::path("update_ipfs"))
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and(oracle_mutex_filter.clone())
        .and(oracle_stat_mutex_filter.clone())
        .and_then(add_list_item_v3);
 /*
    // getting average for all the items (keys/values)
    let get_average_item_v2 = warp::post()
        .and(warp::path("v2")).and(warp::path("average"))
        .and(warp::body::json())
        .and(oracle_mutex_filter.clone())
        .and_then(get_average_item_v2);

    // getting statistics data (average,min,max,deviation) for all the items (keys/values)
    let get_stats_item_v2 = warp::post()
        .and(warp::path("v2")).and(warp::path("stats"))
        .and(warp::body::json())
        .and(oracle_stat_mutex_filter.clone())
        .and_then(get_stats_item_v2);

    // get all items
    let get_allitems = warp::get()
        .and(warp::path("v1")).and(warp::path("getall"))
        .and(store_filter.clone()).and_then(get_list_all);

    let get_latestitemsArray = warp::post()
        .and(warp::path("v1")).and(warp::path("getarray")).and(warp::body::json())
        .and(store_filter.clone()).and_then(get_latestitemsArray);
    let get_latestitems = warp::post()
        .and(warp::path("v1")).and(warp::path("getlatest")).and(warp::body::json())
        .and(store_filter.clone()).and_then(get_latestitems);
        */
    let get_getallarray = warp::post()
        .and(warp::path("v3")).and(warp::path("getallarray")).and(warp::body::json())
        .and(store_filter_sv3.clone()).and_then(get_allarray);


    let get_latestitemsArrayV3 = warp::post()
        .and(warp::path("v3")).and(warp::path("getarray"))
        .and(warp::body::json())
        .and(oracle_stat_mutex_filter.clone())
        .and(store_filter_sv3.clone())
        .and_then(get_latestitems_array_v3);
/*
    let get_item = warp::post()
        .and(warp::path("v1")).and(warp::path("getone")).and(warp::body::json())
        .and(store_filter.clone()).and_then(get_one);

    let getlatestvector = warp::path("v1").and(warp::path("latest"))
        .and(warp::path::param())
        .and(warp::path::param())
        .and(store_filter.clone())
        .and_then(get_latestvector);*/

    let health_route = warp::path!("live").and_then(health_handler);
    // TODO NEED TO FIND WAY TO LAUNCH SWAGGER via Tokio 0.2
    //let proxy=reverse_proxy_filter(("swagger").to_string(), "http://127.0.0.1:8080/".to_string()).and_then(handler::log_response);
    /*let proxy = warp::path!("swagger" / ..).and(
        reverse_proxy_filter("".to_string(), "http://127.0.0.1:4000/".to_string())
            .and_then(handler::log_response),
    );*/

    let routes2 = health_route
        .or(add_items_v3)
        .or(get_getallarray)
        .or(get_latestitemsArrayV3)
        .with(warp::cors().allow_any_origin());
/*
    let routes = health_route
        .or(getlatestvector)
        // .or(proxy)
        .or(get_allitems)
        //  .or(add_items)
        //  .or(add_items_v2)
        .or(add_items_v3)
        .or(get_average_item_v2)
        .or(get_stats_item_v2)
        .or(get_latestitemsArrayV3)
        .or(get_getallarray)
//        .or(get_latestitemsArray)
        //.or(get_latestitems)
        .or(get_item)
        .with(warp::cors().allow_any_origin());*/
    warp::serve(routes2).run(([0, 0, 0, 0], 8000)).await;
}