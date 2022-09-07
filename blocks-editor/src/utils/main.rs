// use std::borrow::{Borrow, BorrowMut};
use wasm_bindgen::prelude::*;
use std::{sync::{Mutex, RwLock},rc::Rc, panic};
// use ndarray::{Array, Array2, Array3, Axis, concatenate, ShapeBuilder};
use crate::draw::{detailsPainter::drawSettings, drawPalette::{drawPalette}, canvas::{drawCanvas}, infoZone::{drawInfoZone}, LayersZone::{drawLayersZone}, CanvasPainter::{CanvasPainter}, documentstorage::documentStorageActions};
use crate::main::netstorage::{NetStorage, testStorage, initNetStorage};
use crate::types::{updators::UpdatorChannels, layersStorage::{DocumentStorage}, state::{Istate}, updators::{UpdatorsStorage}};
use crate::blocks::allblocks::{initBlocks};
use crate::draw::finalUpdator::finalUpdator;
use crate::types::general::ActiveElement;
use web_sys::{ Document};
use crate::types::layersStorage::PaletteElements;

extern crate console_error_panic_hook;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console )]
    pub(crate) fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console )]
    pub(crate) fn info(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name=error )]
    pub(crate) fn cerror(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name=warn )]
    pub(crate) fn cwarn(s: &str);
}


#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_info {
    ($($t:tt)*) => (info(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (cerror(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => (cwarn(&format_args!($($t)*).to_string()))
}


#[macro_export]
macro_rules! loglog {
    ($($t:tt)*) => (log(&format!("{:?}",$($t)*).to_string()))
}

//type ScreenZone=(&Document, Rc<RwLock<Istate>>,Rc<RwLock<DocumentStorage>>, Rc<Mutex<PaletteElements>>,Rc<RwLock<UpdatorsStorage>> );
//(document: &Document, stateBase: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>)
// main method
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let stateRW = Rc::new(RwLock::new(Istate::default()));
    let updatorsStorage = Rc::new(RwLock::new(UpdatorsStorage::default()));
    let CurrentDocumentStorage = Rc::new(RwLock::new(DocumentStorage::default()));
    let netstorage = NetStorage::new();// Rc::new(Mutex::new(NetStorage::new() ));
    let CurrentElements = Rc::new(Mutex::new(initBlocks(stateRW.clone(), CurrentDocumentStorage.clone())));
    let document = web_sys::window().unwrap().document().unwrap();

    // show and save/load current document
    let (zUpdator, infoSender, infoReceiver):ActiveElement = drawInfoZone(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());
    let (canvasUpdator, canvasSender, canvasReceiver):ActiveElement = drawCanvas(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());
    let (paletteUpdator, paletteSender, paletteReceiver):ActiveElement = drawPalette(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());
    let (canvasPainterUpdator, canvasEditorSender, canvasEditorReceived):ActiveElement = CanvasPainter(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());
    let (layersUpdator, layersSender, layersReceiver):ActiveElement = drawLayersZone(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());
    let (documentStorageUpdator, documentStorageUpdator_s,documentStorageUpdator_r):ActiveElement = documentStorageActions(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());
    let (drawSettings_updator, drawSettings_s,drawSettings_r):ActiveElement = drawSettings(&document, stateRW.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone(), updatorsStorage.clone());

    initNetStorage(netstorage.clone(), CurrentDocumentStorage.clone(), CurrentElements.clone());
    testStorage(netstorage.clone());

    let mut updatorState = updatorsStorage.clone();
    {
        if let Ok(mut updators) = updatorState.write() {
            updators.addUpdatorAndSignal(UpdatorChannels::LayersUpdator, documentStorageUpdator, documentStorageUpdator_s.clone(),documentStorageUpdator_r.clone());
            updators.addUpdatorAndSignal(UpdatorChannels::InfoZone, zUpdator, infoSender.clone(), infoReceiver.clone());
            updators.addUpdatorAndSignal(UpdatorChannels::PaletteMain, paletteUpdator, paletteSender.clone(), paletteReceiver.clone());
            updators.addUpdatorAndSignal(UpdatorChannels::LayersDraw, layersUpdator, layersSender.clone(),layersReceiver.clone());
            updators.addUpdatorAndSignal(UpdatorChannels::CanvasDraw, canvasUpdator, canvasSender.clone(), canvasReceiver.clone());
            updators.addUpdatorAndSignal(UpdatorChannels::CanvasPainter, canvasPainterUpdator, canvasEditorSender.clone(),canvasEditorReceived.clone());
            updators.addUpdatorAndSignal(UpdatorChannels::SettingsPainter, drawSettings_updator, drawSettings_s.clone(),drawSettings_r.clone());
        }
    }
    Ok(())
}
