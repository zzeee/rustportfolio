//! Infozone module
///
// use std::cell::Cell;
use std::rc::Rc;
use crate::types::state::{Istate};
use crate::types::layersStorage::{DocumentStorage, PaletteElements};

use web_sys::{ Document};
use wasm_bindgen::prelude::*;
use crate::utils::main::log;
// use wasm_bindgen::prelude::*;
// use std::convert::TryInto;
use std::sync::{ Mutex, RwLock};
use crossbeam_channel::{Receiver, Sender, unbounded};
// use keccak_hasher::KeccakHasher;
// use memory_db::{HashKey, MemoryDB};
// use wasm_bindgen::{Clamped, JsCast};
// use web_sys::{ImageData, Path2d};
// use serde::{Deserialize, Serialize};
// use ndarray::{array, Array2, Array3, Axis};
// use rand::Rng;
// use web_sys::{HtmlElement, MessageEvent, Worker};
use crate::console_log;
// use crate::draw::mouse::{mousedown, mouseup, parseEvent};
// use crate::types::storage::ElementsPalette;
use crate::types::events::{BlockEvent, EventTypes};
use crate::types::general::ActiveElement;
use crate::types::updators::UpdatorsStorage;

pub fn redrawInfoZone(document: Document, rc: Rc<RwLock<Istate>>, rc1: Rc<RwLock<DocumentStorage>>, rc2: Rc<Mutex<PaletteElements>>, res: BlockEvent, updators: Rc<RwLock<UpdatorsStorage>>) {
    let infoZone = document.get_element_by_id("controlZone").unwrap();
    let mut code = 0;
    if res.code == EventTypes::ActiveElementChanged { code = res.subcode }
    if let Ok(mut state) = rc.write() {
        let rs = format!("<div style='border: thin solid blue'>INFOZONE:{:?} {:?} {:?} {:?} {:?} {:?}</div>", state.activeElement, state.moveMode, state.dragElement, code, state.scrollX, state.scrollY);
        infoZone.set_inner_html(&rs);
    } else {}
}

pub fn drawInfoZone(document: &Document, stateBase: Rc<RwLock<Istate>>, rc: Rc<RwLock<DocumentStorage>>, rc0: Rc<Mutex<PaletteElements>>,
                  //  mut receiver: Receiver<BlockEvent>,
                    updators: Rc<RwLock<UpdatorsStorage>>
) -> ActiveElement {
    let (sender,receiver ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();

    // let mut counter=0;
    let canvas0 = document.create_element("canvas").unwrap();
    let infoZone = document.create_element("div").unwrap();
    let infoZone2 = document.create_element("div").unwrap();
    infoZone.append_child(&infoZone2).unwrap();
    infoZone.set_id("infoZone");
    infoZone2.set_id("controlZone");
    canvas0.set_id("controlZoneCanvas");

    infoZone2.set_inner_html("<div style='border: thin solid red'>INFOZONE</div>");
    document.body().unwrap().append_child(&infoZone).unwrap();

   // let document01=document.clone();
    let document02=document.clone();
 //   // let document03_2=document.clone();
    let state2 = stateBase.clone();
    // let state3_2 = stateBase.clone();
 //   let state2_1 = stateBase.clone();
    let rcDocumentStorage03 = rc.clone();
  //  let rcDocumentStorage03_1 = rc.clone();
    // let rcDocumentStorage03_2 = rc.clone();
    let rcElementsPalette03 = rc0.clone();
  //  let rcElementsPalette03_1 = rc0.clone();
    // let rcElementsPalette03_2 = rc0.clone();
 //   let updators01=updators.clone();
    let updators02=updators.clone();
 let receiver01=receiver.clone();
   // let receiver02=receiver.clone();
   /* let cc2 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            let rt = receiver02.try_recv();
            // console_log!("Sss {:?}", rt);
            if let Ok(res) = rt {
                // parseEvent(event, state2.clone(), rcDocumentStorage03.clone(),rcElementsPalette03.clone(),res.clone());
                redrawInfoZone(document01.clone(), state2_1.clone(),rcDocumentStorage03_1.clone(),rcElementsPalette03_1.clone(), res, updators01.clone());
               //  console_log!("Canvas_evt {:?}",res.clone());
            }
        }) as Box<dyn FnMut(_)>);*/
    // document.body().unwrap().add_event_listener_with_callback("mousemove", cc2.as_ref().unchecked_ref()).unwrap();
    // document.body().unwrap().add_event_listener_with_callback("mouseup", cc2.as_ref().unchecked_ref()).unwrap();
    // document.body().unwrap().add_event_listener_with_callback("mousedown", cc2.as_ref().unchecked_ref()).unwrap();
   //  cc2.forget();
    // let mut updator:Rc<Mutex<dyn FnMut()>> =Rc::new( Mutex::new((|| {})));


    let mut cca = 0;
    (Box::new(move || {
        // console_log!("update infozone {:?}",cca);
        cca = cca + 1;
        //
        let rt = receiver01.try_recv();
        // console_log!("Sss {:?}", rt);
        if let Ok(res) = rt {
           //  console_log!("update infozone event! {:?}", res);
            // parseEvent(event, state2.clone(), rcDocumentStorage03.clone(),rcElementsPalette03.clone(),res.clone());
            redrawInfoZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent { code: EventTypes::ActiveElementChanged, subcode: 0 }, updators02.clone());
            //  console_log!("Canvas_evt {:?}",res.clone());
        }
    }), sender, receiver)

   // let mut state = stateBase.read().unwrap();
   // let mut CurrentElementsPalette = rc0.read().unwrap();
  //  let mut CurrentDocumentStorage = rc.read().unwrap();
}
    /* CurrentElementsPalette.addElement(UIElement {
        id: 23,
        kind: 0,
        text: "Element 1".to_string(),
        renderer: |data: &Istate| -> String {
            // log(&format!("Element_1 {:?}", data));
            format!("X {:?}", data)
        },
    });
    CurrentElementsPalette.addElement(UIElement { id: 35, kind: 1, text: "Element 2".to_string(), renderer: |data: &Istate| -> String { "".to_string() } });
      let elements = CurrentElementsPalette.getElements();

    for element in elements.iter() {
        let element0 = &element.clone();
        let state00 = stateBase.clone();
        // log(&format!("ELEMENT: {:?}", element));
        if element0.text != "" {
            let button = document.create_element("button").unwrap();
            button.set_inner_html(&element.text);
            let id = element0.id;
            document.body().unwrap().append_child(&button).unwrap();
            let elementChooser = Closure::wrap(Box::new(
                move |event: web_sys::MouseEvent| {
                    let mut state0 = state00.write().unwrap();
                    state0.setActiveElement(id);
                }) as Box<dyn FnMut(_)>);
            let cc_temo = elementChooser.as_ref().unchecked_ref();
            button.add_event_listener_with_callback("click", cc_temo).unwrap();
            elementChooser.forget();
        }
    }

    let infoZonePaletter = document.create_element("div").unwrap();
    infoZonePaletter.set_id("infoPalette");
    infoZonePaletter.set_inner_html("infoPalette");
    document.body().unwrap().append_child(&infoZonePaletter).unwrap();
}

     */
