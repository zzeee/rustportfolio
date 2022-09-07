use std::fmt;
use crate::utils::main::{log};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use wasm_bindgen::{JsCast};
use web_sys::{MessageEvent, WebSocket};
use serde_json::{ json, Map, Value};
// use serde;
use std::collections::HashMap;
// use std::stringify;
// use parking_lot::RwLock;
use crate::{console_log};
use crate::types::layersStorage::{DocumentStorage, PaletteElements};
// use crate::types::storage::ElementsPalette;
/*
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NetType {
    #[serde(rename = "type")]
    kind: String,
    payload: Value
}

#[derive(Clone)]
pub struct NetVariables {
    id: i32,
    user_id: i32,
    data: HashMap<String, Value>,
   // datai32: HashMap<String, i32>
}

impl Default for NetVariables {
    fn default() -> Self {
        Self { id: 0, user_id: 0, data: HashMap::new() }
    }
}
impl NetVariables {
    fn inc(&mut self) {
        self.id = self.id + 1
    }
    fn addToMap(&mut self, key: String, value: Value) {
        self.data.insert(key, (value.clone()));
    }

}

impl fmt::Debug for NetVariables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetStorage")
            .field("id", &self.id)
            .field("data", &self.data)
            .finish()
    }
}

*/
#[derive(Clone)]
pub struct NetStorage {
    inited: bool,
    pub(crate) ws: Option<WebSocket>,
    pub(crate) wssec: Option<WebSocket>,
   // pub(crate) state: Rc<Mutex<NetVariables>>,
    pub(crate) data: Rc<Mutex<HashMap<String, Value>>>
}


impl fmt::Debug for NetStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      // let lock=&self.data.lock().unwrap();
      //  let keys=lock.keys();
        f.debug_struct("NetStorage")
            //.field("dataKeys", keys)
            .field("data", &self.data)
            .finish()
      //  f.debug_set().entries(keys).finish()
    }
}
impl NetStorage {
    pub(crate) fn new() -> NetStorage {
        // let state = Rc::new(Mutex::new(NetVariables::default()));
        let data = Rc::new(Mutex::new(HashMap::new()));
        NetStorage { inited: false, ws: None, wssec:None, data }
    }

    fn addToMap(&mut self, key: String, value: Value) {
        let mut store = self.data.lock().unwrap();
        store.insert(key, value.clone());
    }

    pub fn setSocket(&mut self, value: WebSocket, kind:i32) {
        match kind {
            0 => self.ws = Some(value),
            1 => self.wssec = Some(value),
            _ => self.ws = Some(value)
        }

    }

    pub fn parseLoginByLoginPassword(&mut self, value:Value) {
        let sid=value["sid"].as_str().unwrap();
        console_log!("parse Login1 {:?}", sid);
        console_log!("parse Login2 {:?}", self);
        let state=self.data.lock().unwrap();
        console_log!("aa");
        let payload = json!({
            "type":"connect",
            "payload":{
             "request_id":2313,
                "user_agent":"wewe",
             "request":{
            "key":sid,
            "customer_id":1542
                    },
        }
        }).to_string();
        console_log!("ba {:?}",payload );

        // let payloadStr=payload.as_str().unwrap();
        console_log!("PPPR {:?}", payload);
        let cloned_wss=self.wssec.clone().unwrap();
        cloned_wss.send_with_str(&payload);

    }


    pub(crate) fn commit(&mut self, kind:&str, val:Value /*payload: Map<String, Value>*/) {
        // let kind = (payload["type"]).to_string();
        // let val = &payload["payload"];
        {
            // let mut state = self.state.lock().unwrap();
            self.addToMap(kind.to_string(), val.clone());
            //console_log!("DDD {:?}", kind.clone(), );
        }
    }

    fn dispatch(&mut self, kind:&str, value:Value ) {
         match kind {
            "login" => self.parseLoginByLoginPassword(value.clone()),
            "printState" => console_log!("State: {:?} ", self.data),
             "ping" => console_log!("ping!"),
            _ => { console_log!("missed {:?}", kind.clone());}
        }
    }

    fn runObserver(&mut self, payload: Map<String,Value>) {
        let pl1=payload.clone();
        let kind = (pl1["type"]).to_string();
        let val = &pl1["payload"];
        let mut kind = (payload["type"]).to_string();
        kind= kind[1..(kind.len() - 1)].parse().unwrap();
        let val = &payload["payload"];
        self.commit(&*kind.clone(), val.clone());
        self.dispatch(&*kind.clone(), val.clone());
    }

    pub fn sendMsg(&mut self, value: String) {}

    fn connect(&mut self) {
        let wssec: WebSocket = WebSocket::new("ws://127.0.0.1:9000").unwrap();
        let wspub: WebSocket = WebSocket::new("ws://127.0.0.1:9100").unwrap();
        wssec.set_binary_type(web_sys::BinaryType::Arraybuffer);
        wspub.set_binary_type(web_sys::BinaryType::Arraybuffer);
        self.setSocket(wspub,0);
        self.setSocket(wssec,1);
        self.connectParsers();
    }

    fn connectParsers(&mut self) {
        let mut obs = self.clone();
        if let Some(wss) = &self.ws.clone() {
            // console_log!("connect pub");
            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    let txtx: String = txt.clone().into();
                    let checkValue = serde_json::from_str(&*txtx);
                    match checkValue {
                        Ok(payload) => {
                            if let serde_json::Value::Object(data) = payload {
                                obs.runObserver(data);
                            }
                        }
                        Err(aerror) => console_log!("error {:?}", aerror),
                        _ => console_log!("unknowe wer")
                    }
                } else {
                    console_log!("message event, received Unknown: {:?}", e.data());
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            let cloned_wss=wss.clone();
            let onopen_callback = Closure::wrap(Box::new(move |_| {
                console_log!("socket opened");
                match cloned_wss.send_with_str(r###"
        {"payload":{"request":{"login":"zzeeee@gmail.com","customer_id":1542,"password":"guy@guy"}},"type":"login"}"###
                ) {
                    Ok(_) => console_log!("binary message successfully sent"),
                    Err(err) => console_log!("error sending message: {:?}", err),
                }
            }) as Box<dyn FnMut(JsValue)>);
            wss.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            wss.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
            onopen_callback.forget();
        }
    }
}

pub fn testStorage(mut state: NetStorage) {
        state.commit("q2", serde_json::Value::String("B500".parse().unwrap()));
        // state.dispatch("printState", serde_json::Value::String("".parse().unwrap()));


}
pub fn initNetStorage(mut state: NetStorage, rc: Rc<RwLock<DocumentStorage>>, rc0: Rc<Mutex<PaletteElements>>) {
    // let mut storage = state.lock().unwrap();
    state.connect();
    // let state = netstorage.data.lock().unwrap();
    state.commit("t1", serde_json::Value::String("A200".parse().unwrap()));
    // state.dispatch("printState", serde_json::Value::String("".parse().unwrap()));
    state.commit("t2", serde_json::Value::String("A300".parse().unwrap()));
    // state.dispatch("printState", serde_json::Value::String("".parse().unwrap()));
}

