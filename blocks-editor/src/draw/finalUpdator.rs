use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use crossbeam_channel::{Receiver,Sender, unbounded};
use web_sys::Document;
use crate::console_log;
use crate::types::layersStorage::{DocumentStorage, PaletteElements};
use crate::types::events::{BlockEvent, EventTypes};
use crate::types::events::EventTypes::RedrawCanvas;
use crate::types::state::Istate;
use crate::types::updators::{UpdatorChannels, UpdatorsStorage};
use crate::utils::main::log;

pub fn finalUpdator(document: &Document, stateBase: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>,
                              us: Rc<RwLock<UpdatorsStorage>>,
) -> Box<dyn FnMut()>  {
    let mut cca=0;
    Box::new(move || {
        console_log!("from final updator {:?}", cca);
        cca=cca+1;


         //redrawInfoZone(document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent{code:0,subcode:0}, updators02.clone());

    })

}
