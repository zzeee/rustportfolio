use std::cell::Cell;
use std::rc::Rc;
use crate::types::state::{Istate};
use crate::types::layersStorage::{ DocumentStorage, PaletteElements, UIElement};
use web_sys::{Element, Document};
use wasm_bindgen::prelude::*;
use crate::utils::main::{log,info, cerror, cwarn};
use crate::types::events::{CollisionZone, MouseActions, MouseResult, OutputInput};

use wasm_bindgen::prelude::*;
use std::convert::TryInto;
use std::sync::{Mutex, RwLock};
use crossbeam_channel::Sender;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{ImageData, Path2d};
use serde::{Deserialize, Serialize};
use ndarray::{array, Array2, Array3, Axis};
// use rand::Rng;
use web_sys::{HtmlElement, MessageEvent, Worker};
use crate::blocks::blocks::AllMasks;
use crate::console_log;
use crate::console_info;
use crate::console_error;
use crate::console_warn;
use crate::main::collisions::checkCollisions;
use crate::types::layersStorage::{UIElementObject,CollisionsLine};
use crate::types::events::{BlockEvent, EventTypes, EventUpdators};
use crate::types::storage::ElementsPalette;
use crate::types::updators::{UpdatorChannels, UpdatorsStorage};


pub fn mousewheel(event: web_sys::WheelEvent, rc: Rc<RwLock<Istate>>, context: Rc<web_sys::CanvasRenderingContext2d>, rc1: Rc<RwLock<DocumentStorage>>, rc2: Rc<Mutex<PaletteElements>>, us0_1: Rc<RwLock<UpdatorsStorage>>)->bool {
    // console_log!("wheel {:?} {:?} ", event.delta_x(),event.delta_y());
    let mut moved=false;
    if let Ok(mut state) = rc.write() {
        if state.scrollY + event.delta_y() as f32 >= 0.0 && state.scrollX + event.delta_x() as f32 >= 0.0 {
            state.scrollY = state.scrollY + event.delta_y() as f32;
            state.scrollX = state.scrollX + event.delta_x() as f32;
            moved = true
        }
    }
    if moved {
        if let Ok(mut updators) = us0_1.write() {
            updators.sendOnlySignalByEvent(BlockEvent { code: EventTypes::MoveCanvas, subcode: 0 });
            // updators.updateAll();
        } else {
            console_log!("updators locked@wheel")
        }
    }
    moved
}
pub fn mousedown(event: web_sys::MouseEvent, rc: Rc<RwLock<Istate>>, context: Rc<web_sys::CanvasRenderingContext2d>, rc1: Rc<RwLock<DocumentStorage>>, rc2: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) {
    let mut state = rc.write().unwrap();
    // log(&format!("mousedown {:?} {:?}",state.paletteClicked, *state));
    context.begin_path();
    context.move_to(event.offset_x() as f64, event.offset_y() as f64);
    state.setCoordinates(0, event.offset_x());
    state.setCoordinates(1, event.offset_y());
    state.setMousePressed(true);
    {
        if let Ok(mut uss)=us.write() {
            uss.sendOnlySignalByEvent(BlockEvent{code:EventTypes::MouseMove, subcode:0});
        }
    }
}


pub fn parseEvent(event: web_sys::MouseEvent, rc: Rc<RwLock<Istate>>, rc1: Rc<RwLock<DocumentStorage>>, rc2: Rc<Mutex<PaletteElements>>, res: BlockEvent, uc: Rc<RwLock<UpdatorsStorage>>) {
    // console_log!("parseEvent evt!!! {:?} {:?} ", res, res.subcode);
    if res.code == EventTypes::ActiveElementChanged {
        {
            if let Ok(mut state)= rc.write() {
              state.setActiveElement(res.subcode);
            } else {
                console_log!("error parseEvent busy state")
            }
          //  let mut state = rc.write().unwrap();
        }
        {
            if let Ok(mut ucc)=uc.write() {
                ucc.sendOnlySignalByEvent(res.clone());


                ucc.updateAll();
            }
            else {
                console_log!("error parseEvent busy Updators")
            }
        }
    }
}

pub fn doMouseMove(event: web_sys::MouseEvent, rc: Rc<RwLock<Istate>>, context: Rc<web_sys::CanvasRenderingContext2d>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) ->MouseResult {
    let x = event.offset_x();
    let y = event.offset_y();
    let mut action: MouseActions = MouseActions::NoAction;
    if let Ok(mut state) = rc.write() {
        let scrolledy = state.scrollY as i32 + y;
        let scrolledx = state.scrollY as i32 + x;
        let previousX = state.mouseXclicked;
        let previousY = state.mouseYclicked;
        let mousePressed = state.mousePressed;
        let mut detected: Option<(Vec<CollisionsLine>, i32, i32, i32, i32)> = None;
        if let Ok(mut documents) = ds.write() {
            let layers = documents.documentStorageAsIs();
            let mut layers_iter = layers.iter();
            let movingElement=state.movingElementId;
            // add sort by order to find only the element which on top!
           // block finding by coordinate after pressing esc.
            let el = layers_iter.find(|el| (el.startx < scrolledx && el.startx + el.width > scrolledx) && (el.starty < scrolledy && el.starty + el.height > scrolledy));
            if let Some(elem) = el {
                if state.stickyMode {
                state.switchToMoveMode(true);
                state.setMoveElementId(elem.id);
                if mousePressed {
                   // console_log!("mp");
                    let deltaX = x - previousX;
                    let deltaY = y - previousY;
                    state.setDragged(true);
                    let mut moveElement: u32 = elem.id;
                    if state.dragElement == None {
                        state.setDraggedElement(elem.id);
                    } else if let Some(elem) = state.dragElement {
                        moveElement = elem;
                    }
                    detected = documents.detectCollision(moveElement, scrolledx, scrolledy);
                    if deltaX + deltaY != 0 {
                        documents.moveElement(moveElement, deltaX, deltaY);
                        state.setCoordinates(0, event.offset_x());
                        state.setCoordinates(1, event.offset_y());
                        action = MouseActions::ElementMoved;
                    } else {
                       // console_log!("skipped0");
                    }
                } else {
                  //  console_log!("nmp");
                    let parseIsInside = documents.parseIsInside(elem.id, event.offset_x(), event.offset_y(), ps.clone(), us.clone());
                    state.setMouseHovered(parseIsInside);
                    documents.setHoveredStatusToOne(elem.id, true);
                    action = MouseActions::ElementHovered;
                }
            } //    console_log!("found_! P{:?}",elem.id);
            } else {

                let deltaX = x - previousX;
                let deltaY = y - previousY;
                // console_log!("nothing found {:?}",movingElement);
                if movingElement!=0 && mousePressed {

                    if deltaX + deltaY != 0 {
                        documents.moveElement(movingElement, deltaX, deltaY);
                        state.setCoordinates(0, event.offset_x());
                        state.setCoordinates(1, event.offset_y());
                        action = MouseActions::ElementMoved;
                    }
                }
                state.switchToMoveMode(false);
                documents.setHoveredStatusToNothing();
                action = MouseActions::EmptySpace;
            }
        }


        if let Some(detectCollision) = detected {
             web_sys::window().unwrap()
        .performance()
        .expect("should have a Performance")
        .now();
            if detectCollision.clone().0.len() > 1 {
                // console_log!("detected collisions! {:?}",detectCollision.clone().0.len());
                if let Some(crossed) = checkCollisions(detectCollision, ds.clone(), ps.clone()) {
                    let o1 = crossed.clone();
                    let o2 = crossed.clone();
                    let outputs: Vec<CollisionZone> = o1.into_iter().filter(|e| e.element_zone == AllMasks::MaskOutputs).collect();
                    let inputs: Vec<CollisionZone> = o2.into_iter().filter(|e| e.element_zone == AllMasks::MaskInputs).collect();
                    let mut finalLine: Vec<OutputInput> = vec![];
                    if outputs.len() > 0 && inputs.len() > 0 {
                        for input in inputs.iter() {
                            for output in outputs.iter() {
                                if output.element_id != input.element_id {
                                    finalLine.push(OutputInput { output_element_id: output.element_id, input_element_id: input.element_id });
                                }
                            }
                        }
                    }

                    if finalLine.len() > 0 {
                        return MouseResult { actionCode: MouseActions::CollisionInputOutputDetected, payload: Some(finalLine) };
                    } else { return MouseResult { actionCode: MouseActions::NoAction, payload: None }; }
                }
            }
        }
         web_sys::window().unwrap()
        .performance()
        .expect("should have a Performance")
        .now();
        return MouseResult { actionCode: action, payload: None }
    } else {
        console_log!("busy state at mouse");
    }
    MouseResult { actionCode: MouseActions::NoAction, payload: None }
}

pub fn mouseup(event: web_sys::MouseEvent, rc: Rc<RwLock<Istate>>, context: Rc<web_sys::CanvasRenderingContext2d>, ds: Rc<RwLock<DocumentStorage>>, allElements: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) {
    if let Ok(mut state) = rc.write() {
        let mut DocumentStorage = ds.write().unwrap();
        if !state.dragged && !state.moveMode {
            let color = state.getActiveColor();
            let activeElement = state.getActiveElement();
            if activeElement == 0 { return }
            let x = state.mouseXclicked;
            let y = state.mouseYclicked;
            let drawx = if x > event.offset_x() { event.offset_x() } else { x };
            let drawy = if y > event.offset_y() { event.offset_y() } else { y };

            let mut storage = vec![];
            storage.push(drawx as i32);
            storage.push(drawy as i32);
            storage.push(x as i32);
            storage.push(y as i32);
            let mut height = 0;
            let mut width = 0;
            let mut elementsO = allElements.lock().unwrap();
            let ellen = DocumentStorage.getLayersCount();
            let mut elements = elementsO.iter();
            let ele = elements.find(|el| el.lock().unwrap().GetKind() == state.activeElement);
            if let Some(element) = ele {
                let elementActive = element.lock().unwrap();
                let size = (*elementActive).GetSize();
                width = size.0;
                height = size.1;
            }
            DocumentStorage.addCurrentElementToStorage(state.activeElement, drawx, drawy, width as i32, height as i32, state.chosenColor, storage, ellen as i32);
            // console_log!("Adding element");
            if let Ok(mut ucc) = us.write() {
                ucc.sendOnlySignalByEvent( BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
            }
        } else {
            let rs=state.hoveredStatus;
            if rs.6==1 {
                console_log!("settings! {:?} {:?}", rs,rs.0);

                state.openSettings(rs.0);
                if let Ok(mut ucc) = us.write() {
                ucc.sendOnlySignalByEvent( BlockEvent { code: EventTypes::OpenSettings, subcode: rs.0 });
            }
            }
               // if let Ok(mut documents) = ds.write() {
                 //   let parseIsInside = documents.parseIsInside(elem.id, event.offset_x(), event.offset_y(), ps.clone(), us.clone());

                    // todo add chosing of layer. not hover layer, but active layer
                    console_log!("skipped due to drag ");
              //  }
            state.setDragged(false);
        }
        state.setMousePressed(false);
        state.clearDraggedElement();
        state.setMoveElementId(0);
    } else {
        console_log!("busy state at mouseup");
    }
    if let Ok(mut ucc) = us.write() {
        ucc.sendOnlySignalByEvent( BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
        // ucc.sendOnlySignalByCode(UpdatorChannels::LayersUpdator, BlockEvent { code: EventTypes::MouseDraw, subcode: 0 });
        ucc.updateAll();
       // drop(state);
        //state.unlock()
       //  RwLock::

    }



}
