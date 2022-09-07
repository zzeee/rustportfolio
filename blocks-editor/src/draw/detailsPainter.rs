//! Block settings module
///
use std::rc::Rc;
use crate::types::state::{Istate};
use crate::types::layersStorage::{DocumentStorage, PaletteElements};
use web_sys::{ Document};
use wasm_bindgen::prelude::*;
use crate::utils::main::log;
use std::sync::{ Mutex, RwLock};
use crossbeam_channel::{Receiver, Sender, unbounded};
use crate::blocks::blocks::{BlockParameterVar, BlockParamTypes};
use crate::console_log;
use crate::types::events::{BlockEvent, EventTypes};
use crate::types::general::ActiveElement;
use crate::types::updators::UpdatorsStorage;
use wasm_bindgen::{Clamped, JsCast};

use std::convert::TryInto;



pub fn redrawSettingsZone(document: Document, docState: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, pe: Rc<Mutex<PaletteElements>>, updators: Rc<RwLock<UpdatorsStorage>>, res: BlockEvent) {
    let infoZone = document.get_element_by_id("controlsettingsZone").unwrap();
    let mut code=0;
    console_log!("evt {:?}", res);
    if res.code==EventTypes::OpenSettings {
        code = res.subcode;
        let rs = format!("<div style='border: thin solid blue'>CONTROL:{:?} </div>", res.subcode);
        let mut strstr = vec![rs];
        let mut fElementKind: u32 = 0;
        let mut storedParams:Vec<(u32, String)>=vec![];

        if code != 0 {
            if let Ok(mut storage) = ds.read() {
                let elementKind = storage.getKindByElementId(code);
                console_log!("elementKind {:?}",elementKind);
                if let Some(kind) = elementKind {
                    fElementKind = *kind;
                    // console_log!("kidn{:?}", kind);
                }
                if let Some(currentLayer)=storage.getLayerById(code) {
                    storedParams=currentLayer.parameters;
                }

            }

            if fElementKind != 0 {
                let mut parameter: Option<Vec<BlockParameterVar>> = None;
                if let Ok(mut elements) = pe.lock() {
                    for elementWrapper in elements.iter() {
                        if let Ok(element) = elementWrapper.lock() {
                            if element.GetKind() == fElementKind {
                                parameter = Some(element.GetParametersPool());
                                console_log!("found el {:?}", parameter);
                            }
                        }
                    }
                }
                if let Some(params) = parameter {
                    let mut vecids = vec![];
                    let mut vecidsM = vec![];
                    for paramsLine in params.iter() {
                        let block_title = paramsLine.clone().block_title;
                        let block_id = paramsLine.clone().block_id;
                        let mut block_value:String="".to_string();
                        // console_log!("storedParams {:?} {:?}", storedParams, block_id);
                        let block_type = paramsLine.clone().block_type;
                        if storedParams.len()>0 {
                            if let Some(storedValue)=storedParams.iter().find(|e|e.0==block_id) {
                                block_value=storedValue.clone().1;
                            }
                            //console_log!("val {:?}", val);
                        }
                        match block_type {
                            BlockParamTypes::BlockInput => {
                                let line = format!("<div><label>{}</label><input id='control_{}' type='text'  value='{}' /></div>", block_title, block_id, block_value); // add html due to types
                                strstr.push(line);
                            }
                            BlockParamTypes::BlockText => {
                                let line = format!("<div><label>{}</label><textarea id='control_{}'>{}</textarea></div>", block_title, block_id,block_value); // add html due to types
                                strstr.push(line);
                            }
                        }
                        vecidsM.push(block_id);
                        vecids.push((block_id, block_type));
                    }

                    infoZone.set_inner_html(&strstr.join(""));
                    let button = document.create_element("button").unwrap();
                    let vecidsd = vecids.clone();
                    let vecidsd2 = vecidsM.clone();
                    let documentStorage = ds.clone();
                    let docState01 = docState.clone();
                    let usStore01 = updators.clone();
                    let element_code = code;
                    let saveButton = Closure::wrap(Box::new(
                        move |event: web_sys::MouseEvent| {
                            console_log!("save element!!! {:?}", vecidsd2);
                            let mut results = vec![];
                            for uelement in vecidsd.iter() {
                                let block_id = uelement.0;
                                let line = format!("control_{}", block_id);
                                let block_type = uelement.1;
                                // console_log!("checking! {:?} {:?}",line,uelement.1);
                                let mut value = "".to_string();
                                if let Some(element) = document.get_element_by_id(&line) {
                                    match block_type {
                                        BlockParamTypes::BlockInput => {
                                            if let Ok(aelemet) = element.dyn_into::<web_sys::HtmlInputElement>() {
                                                value = aelemet.value();
                                            }
                                        }
                                        BlockParamTypes::BlockText => {
                                            if let Ok(aelemet) = element.dyn_into::<web_sys::HtmlTextAreaElement>() {
                                                value = aelemet.value();
                                            }
                                        }
                                    }
                                    results.push((block_id, value));
                                    // console_log!("found {}={}",block_id,value);
                                } else {
                                    console_log!("no1");
                                }
                            }
                            if let Ok(mut docStorage) = documentStorage.write() {
                                docStorage.saveParameters(element_code, results);
                            } else { console_log!("storage busy") }
                            if let Ok(mut state) = docState01.write() {
                                state.openSettings(0);
                            }
                            if let Ok(mut upd) = usStore01.write() {
                                console_log!("results! ");

                                upd.sendOnlySignalByEvent(BlockEvent { code: EventTypes::OpenSettings, subcode: 0 });
                                upd.updateAll();
                            }
                        }) as Box<dyn FnMut(_)>);
                    let saveButtonWrapped = saveButton.as_ref().unchecked_ref();
                    button.add_event_listener_with_callback("click", saveButtonWrapped).unwrap();


                    saveButton.forget();
                    button.set_inner_html("save");
                    //let id = element0.id;
                    infoZone.append_child(&button).unwrap();
                }
            }
        } else {
            infoZone.set_inner_html(&strstr.join(""));
            console_log!("code0")
        }


    }

    /*if  let Ok(mut state) = rc.write() {
        let rs = format!("<div style='border: thin solid blue'>CONTROL:{:?} </div>", state.openSettingsElement);
       infoZone.set_inner_html(&rs);
    } else {
    //    console_log!("redrawInfoZone stated locked!");
    }*/

      //  console_log!("redrawInfoZone3");


}

pub fn drawSettings(document: &Document, stateBase: Rc<RwLock<Istate>>, rc: Rc<RwLock<DocumentStorage>>, rc0: Rc<Mutex<PaletteElements>>,
                  //  mut receiver: Receiver<BlockEvent>,
                    updators: Rc<RwLock<UpdatorsStorage>>
) -> ActiveElement {
    let (sender,receiver ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();

    // let mut counter=0;
    let settingsZone = document.create_element("div").unwrap();
    let settingsZone2 = document.create_element("div").unwrap();
    settingsZone.append_child(&settingsZone2).unwrap();
    settingsZone.set_id("settingsZone");
    settingsZone2.set_id("controlsettingsZone");

    settingsZone2.set_inner_html("<div style='border: thin solid red'>settingsZone2</div>");
    document.body().unwrap().append_child(&settingsZone).unwrap();

   // let document01=document.clone();
    let document02=document.clone();
    let state2 = stateBase.clone();

    let rcDocumentStorage03 = rc.clone();
    let rcElementsPalette03 = rc0.clone();
    let updators02=updators.clone();
    let receiver01=receiver.clone();


    let mut cca = 0;
    (Box::new(move || {
        // console_log!("update infozone {:?}",cca);
        cca = cca + 1;
        //
        let rt = receiver01.try_recv();
        // console_log!("Sss {:?}", rt);
        if let Ok(res) = rt {
            console_log!("update ןמכםדםדד event! {:?}", res);
            // parseEvent(event, state2.clone(), rcDocumentStorage03.clone(),rcElementsPalette03.clone(),res.clone());
            redrawSettingsZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(),updators02.clone(), res );
            //  console_log!("Canvas_evt {:?}",res.clone());
        }
    }), sender, receiver)


}
