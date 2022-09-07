use std::cell::Cell;
use std::rc::Rc;
use crate::types::state::{Istate};
use crate::types::layersStorage::{DocumentStorage, PaletteElements, UIElement};

use web_sys::{Element, Document, HtmlCanvasElement};
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
use crate::console_log;
use crate::draw::mouse::{mousedown, mouseup, parseEvent};
use crate::types::storage::ElementsPalette;
use crate::types::events::BlockEvent;
use crate::blocks::blocks::{RenderParams};
use crate::types::general::ActiveElement;
use crate::types::updators::UpdatorsStorage;

pub fn redrawCanvasZone(document: Document, rc: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, allElements: Rc<Mutex<PaletteElements>>, res: BlockEvent, us: Rc<RwLock<UpdatorsStorage>>) -> bool {
    let drawZone = document.get_element_by_id("testtets").unwrap();
    let canvas = drawZone.dyn_into::<HtmlCanvasElement>().unwrap();
    let state=rc.read().unwrap();
    // canvas.clear();


    let context = canvas
        .get_context("2d").unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    context.clear_rect(0.0,0.0,state.canvasWidth.into(),state.canvasHeight.into());
     // console_log!("REDRAW:::blockscanvas!!! {:?} ", context);
        let mut updated = false;

    let layers = ds.read().unwrap();

    for layer in layers.documentStorageAsIs().iter() {
        let element_kind = layer.element_kind;
        let color = layer.color;
        let element_id=layer.id;
        let sorder = layer.sorder;
        let startx=layer.startx;
        let params=layer.clone().parameters;

        let hovered = layer.hovered;
        let starty=layer.starty;
        let data = layer.data.clone();
        let mut elementsO = allElements.lock().unwrap();
        let mut elements = elementsO.iter();

        let ele = elements.find(|el| el.lock().unwrap().GetKind() == element_kind);
        if let Some(el) = ele {
            let ele2 = el.lock().unwrap();
            // let mut params=vec![];

            let parametersPool=ele2.GetParametersPool();
            // console_log!("ele2 " );
            ele2.RenderOnMainCanvas(RenderParams {x:startx as f32,  params,parametersPool, y:starty as f32,
                scrollX:state.scrollX, scrollY:state.scrollY, id:element_id,

                onlymain:false, formask:false, hovered}, Rc::new(context.clone()));
            updated=true;
        } else {
            // console_log!("Nothing to draw {:?} {:?}", element_kind,layer);
        }


    }
    updated
}

pub fn CanvasPainter(document: &Document, stateBase: Rc<RwLock<Istate>>, rc: Rc<RwLock<DocumentStorage>>, rc0: Rc<Mutex<PaletteElements>>,
                    // mut receiver: Receiver<BlockEvent>,
                    // alldata: Vec<Sender<BlockEvent>>,
                     us: Rc<RwLock<UpdatorsStorage>>,
) -> ActiveElement {
    let (sender,receiver ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();

    let document01 = document.clone();
    let document02 = document.clone();
    let state2_1 = stateBase.clone();
    let state2 = stateBase.clone();
    let rcDocumentStorage02 = rc.clone();
    let rcDocumentStorage03_1 = rc.clone();
    let rcElementsPalette02 = rc0.clone();
    let rcElementsPalette03_1 = rc0.clone();
    let receiver01=receiver.clone();
    let receiver02=receiver.clone();

    let us1=us.clone();



     let mut cca=0;
    (Box::new(move || {
        // console_log!("update_canvaspainter {:?} {:?}",cca,receiver02.len() );
        cca=cca+1;
         let rt = receiver02.try_recv();
            if let Ok(res) = rt {
                // console_log!("update canvaspainter {:?}",res);

                let updated=redrawCanvasZone(document02.clone(), state2.clone(), rcDocumentStorage02.clone(), rcElementsPalette02.clone(), res,us1.clone() );
                 if updated {
                     // let mut uuodators = us1.write().unwrap();
                     // console_log!("a2");
                     // uuodators.updateAll();
                     // console_log!("a3");
                 }
            }
         //redrawInfoZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent{code:0,subcode:0}, updators02.clone());

    }),sender, receiver)
}

