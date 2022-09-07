use std::cell::Cell;
use std::rc::Rc;
use crate::types::state::{ Istate};
use web_sys::{Element, Document};
use crate::utils::main::log;
use wasm_bindgen::prelude::*;
use std::convert::TryInto;
use std::sync::{Mutex, RwLock};
use crossbeam_channel::{Sender, Receiver, unbounded};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{ImageData, Path2d};
use serde::{Deserialize, Serialize};
use ndarray::{array, Array2, Array3, Axis};
use rand::Rng;
use web_sys::{HtmlElement, MessageEvent, Worker};
use crate::draw::mouse::{mousedown, mouseup, mousewheel, parseEvent, doMouseMove};
// use resvg;
use crate::{console_log, loglog};
use crate::types::layersStorage::{DocumentStorage, PaletteElements};
use crate::types::events::{BlockEvent, EventTypes, EventUpdators, MouseActions};
use crate::types::general::ActiveElement;
use crate::types::storage::ElementsPalette;
use crate::types::updators::UpdatorsStorage;

pub fn drawCanvas(document: &Document, state: Rc<RwLock<Istate>>, rcDocumentStorage: Rc<RwLock<DocumentStorage>>, rcElementsPalette: Rc<Mutex<PaletteElements>>,

                  us: Rc<RwLock<UpdatorsStorage>>
) ->  ActiveElement {
        let (sender0,receiver0 ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();

    let receiver01=receiver0.clone();
     let canvas0 = document
        .create_element("canvas").unwrap();
    canvas0.set_id("testtets");
    let us_1=us.clone();
    let us_2=us.clone();
    let us_3=us.clone();


    let state2 = state.clone();
    let state_local = state2.read().unwrap();
    let state02 = state.clone();
    let state1 = state.clone();
    let state3 = state.clone();
    let state011 = state.clone();
    let state012 = state.clone();
    let rcDocumentStorage01 = rcDocumentStorage.clone();
    let rcDocumentStorage02 = rcDocumentStorage.clone();
    let rcDocumentStorage03 = rcDocumentStorage.clone();
    let rcDocumentStorage011 = rcDocumentStorage.clone();
    let rcDocumentStorage012 = rcDocumentStorage.clone();
    let rcDocumentStorage013 = rcDocumentStorage.clone();

    let rcElementsPalette01 = rcElementsPalette.clone();
    let rcElementsPalette02 = rcElementsPalette.clone();
    let rcElementsPalette03 = rcElementsPalette.clone();
    let rcElementsPalette011 = rcElementsPalette.clone();
   // let aSenders = senders.clone();
   // let aSenders1 = senders.clone();
   // let aSenders2 = senders.clone();
    let canvas = canvas0.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();
    canvas.set_width(state_local.canvasWidth as u32);
    canvas.set_height(state_local.canvasHeight as u32);
    document.body().unwrap().append_child(&canvas).unwrap();
    let context = canvas
        .get_context("2d").unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    let context = Rc::new(context);

    let context0 = context.clone();
    let context1 = context.clone();
    let context3 = context.clone();
    let context011 = context.clone();


    let cc2 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            mousedown(event, state02.clone(), context0.clone(),rcDocumentStorage01.clone(), rcElementsPalette01.clone(), us_1.clone());
        }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", cc2.as_ref().unchecked_ref()).unwrap();
    cc2.forget();

    let cc3 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            mouseup(event, state1.clone(), context1.clone(),rcDocumentStorage02.clone(),rcElementsPalette02.clone(), us_2.clone());
           // for sender in aSenders.iter() {
           //     let res = sender.clone().try_send(BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
           // }

        }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", cc3.as_ref().unchecked_ref()).unwrap();
    cc3.forget();
    let us0_1=us.clone();
    let us0_2=us.clone();
    let ccwheel = Closure::wrap(Box::new(
        move |event: web_sys::WheelEvent| {
          //  log(event);
           let moved= mousewheel(event, state011.clone(), context011.clone(),rcDocumentStorage011.clone(), rcElementsPalette011.clone(), us0_1.clone());
            if let Ok(mut uss)=us0_2.write() {
                uss.updateAll();
            }

        }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousewheel", ccwheel.as_ref().unchecked_ref()).unwrap();
    ccwheel.forget();

    let cckey = Closure::wrap(Box::new(
        move |event: web_sys::KeyboardEvent| {
            // console_log!("evt {:?} {:?} {:?}",event, event.key_code(),event.char_code());
            if event.key_code() == 27 {
                if let Ok(mut state) = state012.write() {
                    console_log!("27");
                    state.movingElementId = 0;
                    state.dragElement = None;
                    state.switchToMoveMode(false);
                    state.setActiveTool(0);
                   // state.setStickMode(false);
                }
                if let Ok(mut documents) = rcDocumentStorage013.write() {
                    documents.setHoveredStatusToNothing();
                }
                if let Ok(mut rs) = us_3.write() {
                    rs.sendOnlySignalByEvent(BlockEvent { code: EventTypes::ActiveElementChanged, subcode: 0 });
                    rs.updateAll();
                }
            }
        }) as Box<dyn FnMut(_)>);
    document.body().unwrap().add_event_listener_with_callback("keyup", cckey.as_ref().unchecked_ref()).unwrap();
    document.body().unwrap().add_event_listener_with_callback("keydown", cckey.as_ref().unchecked_ref()).unwrap();
    cckey.forget();
    let us0_2 = us.clone();

    let closure_mousemove = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            let buttons = event.buttons();
            //if buttons > 0 {

           // }
            let needrender= doMouseMove(event, state3.clone(), context3.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), us0_2.clone());
            match needrender.actionCode {
                MouseActions::NoAction => {},
                MouseActions::CollisionDetected => {
                   console_log!("COLLISION!!! {:?}", needrender);
                     if let Ok(mut rs) = us0_2.write() {
                        rs.sendOnlySignalByEvent(BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
                        rs.updateAll();
                    }

                }, MouseActions::CollisionInputOutputDetected=> {
                   // console_log!("COLLISION!!! {:?}", needrender);
                    if let Ok(mut ds)=rcDocumentStorage012.write(){
                        if let Some(connections)=needrender.payload {
                            ds.addInputOutputConnection(connections);
                        }
                    }
                     if let Ok(mut rs) = us0_2.write() {
                        rs.sendOnlySignalByEvent(BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
                        rs.updateAll();
                    }

                },
                (MouseActions::ElementMoved | MouseActions::ElementHovered | MouseActions::EmptySpace) => {
                    if let Ok(mut rs) = us0_2.write() {
                        rs.sendOnlySignalByEvent(BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
                        rs.updateAll();
                    }
                }
            }

        }) as Box<dyn FnMut(_)>);
    document.get_element_by_id("testtets").unwrap().add_event_listener_with_callback("mousemove", closure_mousemove.as_ref().unchecked_ref()).unwrap();
    closure_mousemove.forget();

    let closure_drag = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            console_log!("dragevent! {:?}", event);
            //parseUnder(event, state3.clone(), context3.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone());
        }) as Box<dyn FnMut(_)>);
    document.get_element_by_id("testtets").unwrap().add_event_listener_with_callback("ondrag", closure_drag.as_ref().unchecked_ref()).unwrap();
    closure_drag.forget();
    let mut cca=0;
    (Box::new(move || {
        // console_log!("update_canvasdraw {:?}",cca);
        cca=cca+1;
        let rt = receiver0.try_recv();
            if let Ok(res) = rt {
               //  console_log!("event targeted to cabvas {:?}", res);
                //redrawCanvasZone(document02.clone(), state2.clone(), rcDocumentStorage02.clone(), rcElementsPalette02.clone(), res,us1.clone() );
            }
         //redrawInfoZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent{code:0,subcode:0}, updators02.clone());

    }),sender0, receiver01)


}
