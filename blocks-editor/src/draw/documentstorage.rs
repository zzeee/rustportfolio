use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use crossbeam_channel::{Receiver,Sender, unbounded};
use web_sys::Document;
use crate::console_log;
use crate::types::layersStorage::{DocumentStorage, PaletteElements, SOrder};
use crate::types::events::{BlockEvent, EventTypes};
use crate::types::events::EventTypes::RedrawCanvas;
use crate::types::general::ActiveElement;
use crate::types::state::Istate;
use crate::types::updators::{UpdatorChannels, UpdatorsStorage};
use crate::utils::main::log;

pub fn documentStorageActions(document: &Document, stateBase: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) -> ActiveElement  {

    let (sender,documentStorageUpdator_r ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();
let documentStorageUpdator2=documentStorageUpdator_r.clone();

 let mut cca=0;
    (Box::new(move || {
        // console_log!("update documentStorage {:?}", cca);
        let mut updateCode=EventTypes::Nop;
        cca=cca+1;
        // let rt = receiver.try_recv();
        let rt = documentStorageUpdator_r.try_recv();
            if let Ok(res) = rt {
               // console_log!("update documentStorage event {:?}", res);
                match res.code {
                    EventTypes::DeleteLayer=> {
                        console_log!("delete layer! {:?}",res.clone());
                        if let Ok(mut storage)=ds.write(){
                            storage.deleteElementFromStorage(res.subcode);
                            //console_log!("deleted layer");
                            //updateCode=EventTypes::RefreshAll;
                        }
                    },
                    EventTypes::UpLayer=> {
                        console_log!("up layer! {:?}",res.clone());
                        if let Ok(mut storage)=ds.write(){
                                storage.changeElementOrder(res.subcode, SOrder::Up);

                           //  storage.deleteElementFromStorage(res.subcode);
                        }
                    },
                     EventTypes::DownLayer=> {
                        console_log!("down layer! {:?}",res.clone());
                        if let Ok(mut storage)=ds.write(){
                            storage.changeElementOrder(res.subcode, SOrder::Down);

                          //  storage.deleteElementFromStorage(res.subcode);
                        }
                    }
                     EventTypes::MouseDraw=> {

                     },
                     EventTypes::ActiveElementChanged=> {
                         console_log!("active element changed {:?}",res.clone())
                     },
                    _ => {
                        console_log!("unknown event layer! {:?}",res.clone())
                    }
                }
               // console_log!("event: {:?}", res.clone());
               // redrawLayersZone(document01.clone(), state2_1.clone(), rcDocumentStorage03_1.clone(), rcElementsPalette03_1.clone(), res);
            }

         //redrawInfoZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent{code:0,subcode:0}, updators02.clone());

    }),sender, documentStorageUpdator2)
}
