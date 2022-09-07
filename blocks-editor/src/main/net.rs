use wasm_bindgen::prelude::*;
// use std::cell::Cell;
// use std::convert::TryInto;
use std::rc::Rc;
use std::sync::{ Mutex};

use wasm_bindgen::{ JsCast};
use wasm_bindgen_futures::spawn_local;
// use web_sys::{ImageData, Path2d};
// use serde::{Deserialize, Serialize};
// use ndarray::{array, Array2, Array3, Axis};
use web_sys::{ MessageEvent,  ErrorEvent, WebSocket};
// use std::cell::RefCell;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
// use futures::executor::block_on;
use crate::types::state::Istate;
use crate::{console_log, loglog, utils::main::{log}};
use futures::future::{Future};
// use rand::Rng;

#[derive(Clone)]
pub struct TaskData {
    value: i32,
    maxValue:i32,
    // completed: bool,
    waker: Option<Waker>,

}

impl Default for TaskData {
    fn default() -> Self {
        Self {
            value: 0,
            maxValue:3,
            // completed: false,
            waker:None
        }
    }
}



impl TaskData {
    fn new(maxValue: i32) -> TaskData {
        Self {
            value:0,
            maxValue,
            waker:None
        }
    }
    fn increase(&mut self) {
        self.value = self.value + 1;
    }
    fn saveWaker(&mut self, waker:Waker) {
        self.waker=Some(waker.clone());
        waker.clone().wake();
    }
}

impl fmt::Debug for TaskData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskData:")
            .field("value", &self.value)
            .field("maxValue", &self.maxValue)
            .finish()
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            data: Rc::new(Mutex::new(TaskData::default())),
        }
    }
}

#[derive(Clone)]
pub struct Task {
    id: i32,
    data: Rc<Mutex<TaskData>>,
}


impl Future for Task {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loglog!(format!("poll:{:?}", self.id));
        let mut state = self.data.lock().unwrap();

        // loglog!(state.value.to_string());
        if state.value < state.maxValue {
            state.increase();
            // state.saveWaker(cx.waker().clone());
            cx.waker().clone().wake();
            Poll::Pending
        } else {
            Poll::Ready(state.value)
        }
    }
}

impl Task {
    pub fn new(id: i32, maxValue:i32) -> Self {
        let data = Rc::new(Mutex::new(TaskData::new(maxValue)));
        Task { id, data }
    }
}

pub fn initSocket(state: Rc<Mutex<Istate>>) {
    let art0 = async move {
        let rtt2 = Task::new(12, 33).await;
        //let mut qt = rtt2.await;
        loglog!(format!("REWS {:?}",rtt2.to_string()));
    };
    let art1 = async move {
        let rtt2 = Task::new(13, 43).await;
        //let mut qt = rtt2.await;
        loglog!(format!("REWS {:?}",rtt2.to_string()));
    };
     let art12 = async  {
         Task::new(23, 45).await;
    };

  //  spawn_local(art0);
  //  spawn_local(art1);
    // spawn_local(art12);
    spawn_local(async {
        Task::new(23, 2).await;
    });
/*
    let ws:WebSocket = WebSocket::new("ws://127.0.0.1:9000").unwrap();
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    init_socket(ws.clone());
    let mut state = state.lock().unwrap();
    (state).setSocket(ws);*/
}

pub fn init_socket(ws: WebSocket) {
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                Ok(_) => console_log!("binary message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);
            // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            // create onLoadEnd callback
            let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
                // here you can for example use the received image/png data
            })
                as Box<dyn FnMut(web_sys::ProgressEvent)>);
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        match cloned_ws.send_with_str("ping") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
        // send off binary message
        match cloned_ws.send_with_str(r###"
        {"payload":{},"type":"test"}"###
        ) {
            Ok(_) => console_log!("binary message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}
