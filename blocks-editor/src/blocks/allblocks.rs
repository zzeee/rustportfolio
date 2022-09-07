use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use crate::blocks::blocks::{BlockControl, BlockParameterVar, BlockParamTypes, RenderParams};
use crate::blocks::{ifblock::IfBlock, whileblocks::WhileBlock, onblock::OnBlock, forwardtask::ForwardTaskBlock};
use crate::console_log;
use crate::types::layersStorage::PaletteElements;
use crate::types::layersStorage::{DocumentStorage};
use crate::types::state::Istate;
use crate::utils::main::log;
use crate::main::collisions::printShape;

pub fn initBlocks(stateBase: Rc<RwLock<Istate>>, rc: Rc<RwLock<DocumentStorage>>) -> PaletteElements {
    let mut rs = OnBlock {
        name: "on".to_string(),
        hovered: false,

        kind: 110,
        renderFunction2: None,
        width: 0,
        height: 0,
        scale: 1.0,
        clickZones: vec![],
        iconPicture: Default::default(),
        inputsBitmask: vec![],
        size_param1: 0.0,
        id: 0,
        size_param2: 0.0,
        size_param3: 0.0,
        size_param4: 0.0,
        Bitmask: Default::default(),
        bitmap: "".to_string(),
        parametersPool:vec![],
        mask_inputs: Default::default(),
        mask_outputs: Default::default(),
        mask_parameters: Default::default(),
        mask_settings: Default::default(),
        mask_body: Default::default(),
        mask_empty: Default::default(),
    };

    let mut rs1 = WhileBlock {
        name: "if".to_string(),
        hovered: false,
        kind: 111,
        width: 0,
        renderFunction2: None,
        height: 0,
        scale: 1.0,
        clickZones: vec![],
        iconPicture: Default::default(),
        inputsBitmask: vec![],
        size_param1: 0.0,
        id: 0,
        size_param2: 0.0,
        size_param3: 0.0,
        size_param4: 0.0,
                parametersPool:vec![],

        Bitmask: Default::default(),
        bitmap: "".to_string(),
        mask_inputs: Default::default(),
        mask_settings: Default::default(),
        mask_outputs: Default::default(),
        mask_parameters: Default::default(),
        mask_empty: Default::default(),

        mask_body: Default::default(),
    };
    let mut rs2 = IfBlock {
        name: "DATA SOURCE".to_string(),
        hovered: false,
        kind: 112,
        width: 0,
        height: 0,
        scale: 1.0,
        clickZones: vec![],
        iconPicture: Default::default(),
        inputsBitmask: vec![],
        size_param1: 0.0,
                parametersPool:vec![],

        id: 0,
        renderFunction2: None,
        size_param2: 0.0,
        size_param3: 0.0,
        size_param4: 0.0,
        Bitmask: Default::default(),
        bitmap: "".to_string(),
        mask_inputs: Default::default(),
        mask_settings: Default::default(),
        mask_outputs: Default::default(),
        mask_parameters: Default::default(),
        mask_body: Default::default(),
        mask_empty: Default::default(),

    };

    let mut rs3 = ForwardTaskBlock {
        name: "FORWARD".to_string(),
        hovered: false,
        kind: 113,
        width: 0,
        height: 0,
                parametersPool:vec![],

        scale: 1.0,
        clickZones: vec![],
        iconPicture: Default::default(),
        inputsBitmask: vec![],
        size_param1: 0.0,
        id: 0,
        renderFunction2: None,
        size_param2: 0.0,
        size_param3: 0.0,
        size_param4: 0.0,
        Bitmask: Default::default(),
        bitmap: "".to_string(),
        mask_inputs: Default::default(),
        mask_outputs: Default::default(),
        mask_settings: Default::default(),
        mask_empty: Default::default(),

        mask_parameters: Default::default(),
        mask_body: Default::default(),
        // counter: 0
    };
    rs3.setRenderFunction(Arc::new(Mutex::new(move |params: RenderParams| {
        // console_log!("render SEND_COPY! {:?}", params);
        "this.forwardTask();".to_string()
    })));
    let mut rs4 = ForwardTaskBlock {
        name: "SEND_COPY".to_string(),
        hovered: false,
        kind: 114,
        width: 0,
                parametersPool:vec![],

        height: 0,
        scale: 1.0,
        clickZones: vec![],
        iconPicture: Default::default(),
        inputsBitmask: vec![],
        size_param1: 0.0,
        size_param2: 0.0,
        id: 0,
        renderFunction2: None,
        size_param3: 0.0,
        size_param4: 0.0,
        Bitmask: Default::default(),
        bitmap: "".to_string(),
        mask_inputs: Default::default(),
        mask_outputs: Default::default(),
        mask_parameters: Default::default(),
        mask_settings: Default::default(),
        mask_body: Default::default(),
        mask_empty: Default::default(),

        // counter: 0
    };

    rs4.setRenderFunction(Arc::new(Mutex::new(move |params: RenderParams| {
        let mut email="".to_string();
        let mut subject="".to_string();
        let mut text="".to_string();
        if let Some(asubject) = params.parametersPool.iter().find(|e| e.icode == 0) {
            if let Some(subj) = params.params.iter().find(|e| e.0 == asubject.block_id) {
                email = subj.clone().1;
            }
        }
        if let Some(txt) = params.parametersPool.iter().find(|e| e.icode == 1) {
            if let Some(subj) = params.params.iter().find(|e| e.0 == txt.block_id) {
                subject = subj.clone().1;
            }
        }
        if let Some(txt) = params.parametersPool.iter().find(|e| e.icode == 2) {
            if let Some(subj) = params.params.iter().find(|e| e.0 == txt.block_id) {
                text = subj.clone().1;
            }
        }
      //  console_log!("render SEND_COPY! {:?} {:?}", params.params, params.parametersPool);

        format!("this.sendCopy('{}','{}','{}');", email,subject,text)
    })));
      // let mut rng = rand::thread_rng();
      //      let y: f64 = rng.gen();
    rs4.setParametersPool(vec![
        BlockParameterVar::new("email".to_string(), BlockParamTypes::BlockInput, 0,0),
        BlockParameterVar::new("subject".to_string(), BlockParamTypes::BlockInput, 1,1),
        BlockParameterVar::new("text".to_string(), BlockParamTypes::BlockText, 2,2),
    ]);
    let mut rs5 = ForwardTaskBlock {
        name: "FAST DECISION".to_string(),
        hovered: false,
        kind: 115,
        width: 0,
        height: 0,
        scale: 1.0,
        clickZones: vec![],
        iconPicture: Default::default(),
        inputsBitmask: vec![],
        size_param1: 0.0,
        size_param2: 0.0,
        id: 0,
        renderFunction2: None,
        size_param3: 0.0,
        size_param4: 0.0,
        Bitmask: Default::default(),
        bitmap: "".to_string(),
        parametersPool:vec![],
        mask_inputs: Default::default(),
        mask_outputs: Default::default(),
        mask_settings: Default::default(),
        mask_parameters: Default::default(),
        mask_body: Default::default(),
        mask_empty: Default::default(),

        // counter: 0
    };
    rs5.setRenderFunction(Arc::new(Mutex::new(move |params: RenderParams| {
        console_log!("params {:?}", params.params);
        "this.sendDecision();".to_string()
    })));
    let mut arr: PaletteElements;
    arr = vec![Arc::new(Mutex::new(rs))];
    arr.push(Arc::new(Mutex::new(rs1)));
    arr.push(Arc::new(Mutex::new(rs2)));
    arr.push(Arc::new(Mutex::new(rs3)));
    arr.push(Arc::new(Mutex::new(rs4)));
    arr.push(Arc::new(Mutex::new(rs5)));
    for blockMutex in arr.iter_mut() {
        let mut block = blockMutex.lock().unwrap();
        block.init(1.1);
    }
    arr
}
