use std::collections::HashMap;
// use std::cell::Cell;
use std::rc::Rc;
use crate::types::state::{Istate};
use crate::types::layersStorage::{DocumentStorage, LayersActions, PaletteElements, UIElement, UIElementObject};

use web_sys::{Element, Document};
use wasm_bindgen::prelude::*;
use crate::utils::main::log;
use wasm_bindgen::prelude::*;
use std::convert::TryInto;
use std::sync::{Arc, Mutex, RwLock};
use crossbeam_channel::{Receiver, Sender, TryRecvError, unbounded};
// use keccak_hasher::KeccakHasher;
// use memory_db::{HashKey, MemoryDB};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{ImageData, Path2d};
use serde::{Deserialize, Serialize};
use ndarray::{array, Array2, Array3, Axis};
use rand::Rng;
use web_sys::{HtmlElement, MessageEvent, Worker};
use crate::blocks::blocks::RenderParams;
use crate::console_log;
use crate::draw::mouse::{mousedown, mouseup, parseEvent};
use crate::types::storage::ElementsPalette;
use crate::types::events::{BlockEvent, EventTypes};
use crate::types::updators::{UpdatorChannels, UpdatorsStorage};
use crate::types::general::{ActiveElement};

pub fn layerAction(ds: Rc<RwLock<DocumentStorage>>, action: LayersActions) {
    match action {
        Hide => console_log!("hide!"),
        _ => console_log!("rest of hide!")
    }
}
pub fn generateConnections(connections: HashMap<u32, u32>, connections_reversed: HashMap<u32, u32>) -> (Vec<Vec<u32>>, Vec<String>) {
// capacity overflow
    let mut counter=0;
    let mut queues = vec![];
    let mut strstr: Vec<String> = vec![];

    for (k, v) in connections.iter() {
        let mut queue:Vec<u32> = vec![];
        let line = format!("<div>{}=>{}</div>", k, v);
        let chk = connections.get(k);
        if let Some(mut value) = connections.get(k) {
            if connections_reversed.get(k) == None {
                queue.push(*k);
                if connections.get(value)==None {
                     queue.push(*value);
                }
                while counter<100 {
                    if let Some(vvalue) = connections.get(value) {
                        queue.push(*value);
                        queue.push(*vvalue);
                        value = vvalue;
                        counter += 1;
                    } else {
                        counter = 100;
                    }
                }
               /* while let Some(vvalue) = connections.get(value) {
                    queue.push(*value);
                    queue.push(*vvalue);
                    value = vvalue;
                    counter+=1;
                } */
                if counter>100 {console_log!("exited due to counter");}


            }
            if queue.len()>0 {
                queues.push(queue);
            }
        }
        strstr.push(line);
    }
    (queues, strstr )
}
pub fn generateLayersList(alayers: Vec<UIElementObject>) -> Vec<String> {
    let mut strstr: Vec<String> = vec![];

    for layer in alayers.iter() {
        let line = format!("<div id='layer_{:?}'>id:{:?} type:{:?}, data:{:?}, color: {:?}, x,y,width,height: {:?} {:?} {:?} {:?} sorder:{:?} hover: {:?} params {:?}</div>", layer.id, layer.id, layer.element_kind, layer.data, layer.color, layer.startx, layer.starty, layer.width, layer.height, layer.sorder, layer.hovered, layer.parameters);
        strstr.push(line);
    }
    strstr
}
pub fn generateCodeFromQueue (queues: Vec<Vec<u32>>, pe: Rc<Mutex<PaletteElements>>, alayers: Vec<UIElementObject>) ->String {
    // TODO change mutex to hashmap
    // infinite cycle detection
    let mut code="".to_string();
    if let Ok(palette)=pe.lock() {
        for queue in queues.iter() {
            if queue.len() > 0 {
                for line in queue.iter() {
                    if let Some(elem) = alayers.iter().find(|e| e.id == *line) {
                        // console_log!("elem f {:?}", elem);
                         let elem_type=elem.element_kind;
                        let parameters=elem.clone().parameters;
                        let startx=elem.clone().startx;

                        for  elem in palette.iter() {
                            if let Ok(mut element)=elem.lock() {
                                let curent_params=parameters.clone();
                                if element.GetKind()==elem_type {
                                    let parametersPool=element.GetParametersPool();
                                     let renderParams=RenderParams {x: startx as f32,params:curent_params,parametersPool, y: 0.0, id: 0, scrollX: 0.0, scrollY: 0.0, formask: false, onlymain: false, hovered: false };

                                    let result=element.generateCode(renderParams.clone(), 23);
                                    code.push_str(&result);
                                  //  console_log!("element {:?} {:?}", result,element.GetKind());
                                }
                            }
                        }
                        // let pal=palette.
                    }
                }
            }
        }
    }
    code
}

pub fn redrawLayersZone(document: Document, rc: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, pe: Rc<Mutex<PaletteElements>>, res: BlockEvent, us: Rc<RwLock<UpdatorsStorage>>) {
    let infoZone = document.get_element_by_id("layersControlZone").unwrap();
    let layersbase=ds.clone();
    let mut layers = layersbase.write().unwrap();

    let mut strstr: Vec<String> = vec![];
    let alayers = layers.documentStorageSorted();

    // let str0=;
    strstr.extend(generateLayersList(alayers.clone()));
    let (connections,connections_reversed) = layers.connectionsStorage();
    let (queues, connectionsStr)=generateConnections(connections, connections_reversed);
    strstr.extend(connectionsStr);
    let mut code=generateCodeFromQueue(queues,pe.clone(),alayers.clone());
    strstr.push(format!("<br/><code>{:?}</code>",code));
    let libe = strstr.concat();

    let mut rs = String::new(); //{:?}</div>", libe);
    rs.push_str("<div style='border: thin solid green'>");
    rs.push_str(&*libe);
    rs.push_str("</div>");
    infoZone.set_inner_html(&rs);

        for layer in alayers.iter() {
            let us1=us.clone();
            let us2=us.clone();
            let us3=us.clone();
            let us4=us.clone();
            // let us=us.clone();
            let store00=rc.clone();

            let element0=document.get_element_by_id(&format!("layer_{:?}",layer.id));
            if let Some(element)=element0 {
                let idd=layer.id;
                let cc_del = Closure::wrap(Box::new(
                    move |event: web_sys::MouseEvent| {
                        console_log!("pressed delete layer{:?}",idd);
                        let stor=us1.write();
                        if let Ok(mut store)=stor {
                            // store.sendSignalByCode(UpdatorChannels::LayersUpdator, BlockEvent {code: EventTypes::DeleteLayer, subcode:idd});
                            store.sendOnlySignalByEvent( BlockEvent {code: EventTypes::DeleteLayer, subcode:idd});
                            store.sendOnlySignalByEvent(BlockEvent {code: EventTypes::RefreshAll, subcode:0});
                            store.updateAll();

                        }
                    }) as Box<dyn FnMut(_)>);
                let cc_settings = Closure::wrap(Box::new(
                    move |event: web_sys::MouseEvent| {
                        console_log!("pressed cc_settings {:?}",idd);
                        if let Ok(mut state)=store00.write() {
                              state.openSettings(idd);
                        }

                        if let Ok(mut store) = us4.write() {
                            store.sendOnlySignalByEvent(BlockEvent { code: EventTypes::OpenSettings, subcode: idd });
                            store.updateAll();
                        }
                    }) as Box<dyn FnMut(_)>);
                let cc_up = Closure::wrap(Box::new(
                    move |event: web_sys::MouseEvent| {
                        console_log!("pressed up layer{:?}",idd);
                        let stor = us2.write();
                        if let Ok(mut store) = stor {
                            store.sendOnlySignalByEvent(BlockEvent { code: EventTypes::UpLayer, subcode: idd });
                            store.sendOnlySignalByEvent(BlockEvent { code: EventTypes::RefreshAll, subcode: 0 });
                            store.updateAll();

                        }
                    }) as Box<dyn FnMut(_)>);
                let cc_down = Closure::wrap(Box::new(
                    move |event: web_sys::MouseEvent| {
                        console_log!("pressed down layer{:?}",idd);
                         let stor = us3.write();
                         if let Ok(mut store) = stor {
                            store.sendOnlySignalByEvent(BlockEvent { code: EventTypes::DownLayer, subcode: idd });
                            store.sendOnlySignalByEvent(BlockEvent { code: EventTypes::RefreshAll, subcode: 0 });
                            store.updateAll();

                        }
                    }) as Box<dyn FnMut(_)>);
                {
                    let button = document.create_element("button").unwrap();
                    button.set_inner_html(&format!("Del {:?}", layer.id));
                    button.set_id(&format!("btn_del_{:?}", layer.id));
                    element.append_child(&button).unwrap();
                    button.add_event_listener_with_callback("mousedown", cc_del.as_ref().unchecked_ref()).unwrap();
                } {
                    let button = document.create_element("button").unwrap();
                    button.set_inner_html(&format!("Settings {:?}", layer.id));
                    button.set_id(&format!("btn_settings_{:?}", layer.id));
                    element.append_child(&button).unwrap();
                    button.add_event_listener_with_callback("mousedown", cc_settings.as_ref().unchecked_ref()).unwrap();
                }
                {
                    let button = document.create_element("button").unwrap();
                    button.set_inner_html(&format!("Up {:?}", layer.id));
                    button.set_id(&format!("btn_up_{:?}", layer.id));
                    element.append_child(&button).unwrap();
                    button.add_event_listener_with_callback("mousedown", cc_up.as_ref().unchecked_ref()).unwrap();
                }
                {
                    let button = document.create_element("button").unwrap();
                    button.set_inner_html(&format!("Down {:?}", layer.id));
                    button.set_id(&format!("btn_down_{:?}", layer.id));
                    element.append_child(&button).unwrap();
                    button.add_event_listener_with_callback("mousedown", cc_down.as_ref().unchecked_ref()).unwrap();
                }

                cc_up.forget();
                cc_down.forget();
                cc_settings.forget();
                // cc_hide.forget();
                cc_del.forget();
            }

        }

}

pub fn drawLayersZone(document: &Document, stateBase: Rc<RwLock<Istate>>, rc: Rc<RwLock<DocumentStorage>>, rc0: Rc<Mutex<PaletteElements>>,
                     // mut receiver: Receiver<BlockEvent>,
                     // alldata: Vec<Sender<BlockEvent>>,
                      us: Rc<RwLock<UpdatorsStorage>>,
) ->ActiveElement {
    let (sender,receiver ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();

    // let canvas0 = document.create_element("canvas").unwrap();
    let layersZone = document.create_element("div").unwrap();
    let layersZone2 = document.create_element("div").unwrap();
    let us_0=us.clone();
    // infoZone.append_child(&canvas0).unwrap();
    layersZone.append_child(&layersZone2).unwrap();
    layersZone.set_id("layersZone");
    layersZone2.set_id("layersControlZone");
    // canvas0.set_id("controlZoneCanvas");

    let receiver0=receiver.clone();
    //canvas0.set_id("testtets");
    layersZone2.set_inner_html("<div style='border: thin solid red'>Layers</div>");
    document.body().unwrap().append_child(&layersZone).unwrap();

    let document01 = document.clone();
    // let state2 = stateBase.clone();
    let state2_1 = stateBase.clone();
    // let rcDocumentStorage03 = rc.clone();
    let rcDocumentStorage03_1 = rc.clone();
    // let rcElementsPalette03 = rc0.clone();
    let rcElementsPalette03_1 = rc0.clone();
 /*
    let cc2 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            let rt = receiver.try_recv();
            if let Ok(res) = rt {
               // console_log!("event: {:?}", res.clone());
                redrawLayersZone(document01.clone(), state2_1.clone(), rcDocumentStorage03_1.clone(), rcElementsPalette03_1.clone(), res);
            }
        }) as Box<dyn FnMut(_)>);
    document.body().unwrap().add_event_listener_with_callback("mousemove", cc2.as_ref().unchecked_ref()).unwrap();
    document.body().unwrap().add_event_listener_with_callback("mouseup", cc2.as_ref().unchecked_ref()).unwrap();
    document.body().unwrap().add_event_listener_with_callback("mousedown", cc2.as_ref().unchecked_ref()).unwrap();
    cc2.forget();*/

   // let mut state = stateBase.read().unwrap();
   // let mut CurrentElementsPalette = rc0.read().unwrap();
   // let mut CurrentDocumentStorage = rc.read().unwrap();

    let mut cca=0;
    (Box::new(move || {
        // console_log!("update layerszone {:?}",cca);
        cca=cca+1;
        let rt = receiver.try_recv();
            if let Ok(res) = rt {
               // console_log!("update layerszone event {:?}",res);

               // console_log!("event: {:?}", res.clone());
                redrawLayersZone(document01.clone(), state2_1.clone(), rcDocumentStorage03_1.clone(), rcElementsPalette03_1.clone(), res, us_0.clone());
            }
         //redrawInfoZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent{code:0,subcode:0}, updators02.clone());

    }),sender, receiver0)
}

