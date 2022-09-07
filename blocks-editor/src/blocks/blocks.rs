use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use ndarray::{stack, Array, Array1, Zip, Array2, Array3, ArrayBase, Axis, Data, Ix1, Ix2, OwnedRepr};
use wasm_bindgen::{Clamped, JsValue, JsCast};
use web_sys::{HtmlCanvasElement, ImageData};
// use image_base64_wasm::{vec_to_base64};
use crate::{console_log, utils::main::log};
use serde::{Serialize, Deserialize};
use data_encoding::{BASE64, BASE64_MIME};
use crate::main::collisions::{printShape, printShape32};
use rand::prelude::*;

#[derive(Clone, Copy,Debug, PartialEq)]
pub enum BlockParamTypes {
    BlockInput, BlockText
}

#[derive(Clone,  Debug)]
pub struct BlockParameterVar {
    pub block_title: String,
    pub block_id:u32,
    pub icode:u32,
    pub block_type: BlockParamTypes,
    pub sorder: u8,

}

impl BlockParameterVar {
    pub fn new(block_title: String, block_type:BlockParamTypes, sorder:u8, icode:u32) -> Self {
        let mut rng = rand::thread_rng();
        let block_id:u32 = ((rng.gen::<f64>() )*1000000.0) as u32;

        BlockParameterVar { block_title,block_type, sorder,block_id, icode }
    }
}


#[derive(Clone,  Debug)]
pub struct BlockParameterValue {
    pub value: String,
    pub block_value_id: u32,
    pub block_type: u32,
}


#[derive(Clone,  Debug)]
pub struct RenderParams {
    pub x: f32,
    pub y: f32,
    pub id: u32,
    pub scrollX:f32,
    pub scrollY:f32,
    pub formask: bool,
    pub onlymain: bool,
    pub hovered: bool,
    pub params:Vec<(u32, String)>,
    pub parametersPool:Vec<BlockParameterVar>
}

#[derive(Clone, Copy,Debug, PartialEq)]
pub enum AllMasks {
    MaskMain,
    MaskInputs,
    MaskOutputs,
    MaskParameters,
    MaskBody,
    MaskSettings,
    MaskEmpty
}


pub type HoverStatus = (u32, u8, u8, u8, u8, u8,u8);

pub(crate) type DataMask = Option<Array2<u128>>;

pub struct BlockConfig {
    pub(crate) withInputs: bool,
    pub(crate) withOutputs: bool,
    pub(crate) withParameters: bool,
    pub(crate) withBody: bool,
    pub(crate) withSettings: bool,
}

impl BlockConfig {
    pub(crate) fn new(withInputs: bool, withOutputs: bool, withParameters: bool, withBody: bool, withSettings:bool) -> BlockConfig {
        BlockConfig { withInputs, withOutputs, withParameters, withBody,withSettings }
    }
}

#[macro_export]
macro_rules! GetSetBitMaskBasic {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        fn GetBitMask(&self, code: AllMasks) -> Array2<u8> {
         match code {
             AllMasks::MaskMain => self.Bitmask.clone(),
             AllMasks::MaskInputs => self.mask_inputs.clone(),
             AllMasks::MaskOutputs => self.mask_outputs.clone(),
             AllMasks::MaskParameters => self.mask_parameters.clone(),
             AllMasks::MaskBody => self.mask_body.clone(),
         }
     }
        fn SetBitMask(&mut self, code: AllMasks, bitmask:Array2<u8>)  {
            // printShape(bitmask.clone(), code,false,"sbm");
         match code {
             AllMasks::MaskMain => self.Bitmask=bitmask,
             AllMasks::MaskInputs => self.mask_inputs=bitmask,
             AllMasks::MaskOutputs => self.mask_outputs=bitmask,
             AllMasks::MaskParameters => self.mask_parameters=bitmask,
             AllMasks::MaskBody => self.mask_body=bitmask,
         }
     }
    };
}

#[macro_export]
macro_rules! DeclateBasicBlockStorage {
    ($struct_name:ident)=>{
    pub(crate) struct $struct_name {
    pub(crate) name: String,
    pub(crate) kind: u32,
    pub(crate) id: u32,
    // pub(crate) renderFunction: Option<Box<dyn FnMut(RenderParams)->String>>,
    pub(crate) renderFunction2: Option<Mutex<Arc<Mutex<dyn FnMut(RenderParams)->String>>>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) size_param1: f32,
    pub(crate) size_param2: f32,
    pub(crate) size_param3: f32,
    pub(crate) size_param4: f32,
    pub(crate) hovered: bool,
    pub(crate) scale: f32,
    pub(crate) bitmap: String,
    pub(crate) parametersPool:Vec<BlockParameterVar>,
    pub(crate) clickZones: Vec<Array2<i8>>,
    pub(crate) iconPicture: Array2<i8>,
    pub(crate) inputsBitmask: Vec<Array2<i8>>,
    pub(crate) Bitmask: Array2<u128>,
    pub(crate) mask_inputs: Array2<u128>,
    pub(crate) mask_outputs: Array2<u128>,
    pub(crate) mask_parameters: Array2<u128>,
    pub(crate) mask_body: Array2<u128>,
    pub(crate) mask_settings: Array2<u128>,
    pub(crate) mask_empty: Array2<u128>,
    }
        impl fmt::Debug for $struct_name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Istate")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
    }

}

// BlockStorage!(wqeqw);
#[macro_export]
macro_rules! BasicBlock {
()=>{
        fn setParametersPool(&mut self, params:Vec<BlockParameterVar>)  {
            self.parametersPool=params;
        }
        fn GetParametersPool(&self)-> Vec<BlockParameterVar>  {
            self.parametersPool.clone()
        }

        fn setRenderFunction(&mut self, ufn: Arc<Mutex<dyn FnMut(RenderParams)->String>> ) {
            self.renderFunction2=Some(Mutex::new(ufn));
        }


        fn generateCode(&mut self, params:RenderParams, lang:u32)->String {
            if let Some(func)=&self.renderFunction2 {
                if let Ok(mut rf)=func.lock() {
                    let mut ll=rf.lock().unwrap();
                    return ll(params);
                }
            }
            "".to_string()
        }
        fn GetIcon(&self) -> String {
            self.bitmap.clone()
        }

        fn setId(&mut self, id: u32) {
            self.id = id;
        }

        fn getId(&self) -> u32 {
            self.id
        }

        fn setHover(&mut self, hover: bool) {
            self.hovered = hover
        }

        fn getHover(&self) -> bool {
            self.hovered
        }

        fn SetIcon(&mut self, data: String)
        {
            self.bitmap = data;
        }

        fn GetBitMask(&self, code: AllMasks) -> Array2<u128> {
            match code {
                AllMasks::MaskMain => self.Bitmask.clone(),
                AllMasks::MaskInputs => self.mask_inputs.clone(),
                AllMasks::MaskOutputs => self.mask_outputs.clone(),
                AllMasks::MaskParameters => self.mask_parameters.clone(),
                AllMasks::MaskBody => self.mask_body.clone(),
                AllMasks::MaskSettings => self.mask_settings.clone(),
                AllMasks::MaskEmpty => self.mask_empty.clone(),
            }
        }

        fn SetBitMask(&mut self, code: AllMasks, bitmask: Array2<u128>) {
            // printShape(bitmask.clone(), code,false,"sbm");
            match code {
                AllMasks::MaskMain => self.Bitmask = bitmask,
                AllMasks::MaskInputs => self.mask_inputs = bitmask,
                AllMasks::MaskOutputs => self.mask_outputs = bitmask,
                AllMasks::MaskParameters => self.mask_parameters = bitmask,
                AllMasks::MaskBody => self.mask_body = bitmask,
                AllMasks::MaskSettings => self.mask_settings = bitmask,
                AllMasks::MaskEmpty => self.mask_empty = bitmask,

            }
        }

        fn GetKind(&self) -> u32 {
            self.kind
        }

        fn GetSize(&self) -> (usize, usize) {
            (self.width as usize, self.height as usize)
        }
    }
}

#[macro_export]
macro_rules! GetSetHoverBasic {
()=>{
    fn setId(&mut self, id:u32) {
        self.id=id;
    }
    fn getId(&self)->u32 {
        self.id
    }
      fn setHover(&mut self, hover: bool) {
        self.hovered = hover
    }

    fn getHover(&self) -> bool {
        self.hovered
    }
}
    }
#[macro_export]
macro_rules! GetSetIconBasic {
()=>{
    fn GetIcon(&self)->String {
        self.bitmap.clone()
    }
    fn SetIcon(&mut self, data:String)
     {
        self.bitmap=data;
     }
    }
}
pub trait BlockControl {
    // fn new(name: String, kind:i32, scale:i8) -> Self;
    fn init(&mut self, scale: f32);
    // fn new(&mut self);
    fn GetConfig(&self) -> BlockConfig;
    fn GetBitMask(&self, code: AllMasks) -> Array2<u128>;
    // fn GetBitMask128(&self, code: AllMasks) -> Array2<u128>;
    fn SetBitMask(&mut self, code: AllMasks, bitmask: Array2<u128>);
    fn GetKind(&self) -> u32;
    fn GetSize(&self) -> (usize, usize);
    fn SetIcon(&mut self, data: String);
    fn GetIcon(&self) -> String;
    fn setId(&mut self,code:u32) ;
    fn getId(&self)->u32 ;
    fn setHover(&mut self, hover: bool);
    fn getHover(&self) -> bool;
    // fn setRenderFunction(&mut self, ufn: Box<dyn FnMut(RenderParams) ->String>);
    fn setRenderFunction(&mut self, ufn: Arc<Mutex<dyn FnMut(RenderParams) ->String>>);
    fn GetParametersPool(&self) -> Vec<BlockParameterVar>;
    fn setParametersPool(&mut self, parameters: Vec<BlockParameterVar>);
    fn generateCode(&mut self, params:RenderParams,lang:u32)->String;
    fn RenderOnMainCanvas(&self, parameters: RenderParams, ctx: Rc<web_sys::CanvasRenderingContext2d>);
    fn RenderSubzones(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, params: RenderParams, subzone: AllMasks);
    fn Scale(&mut self, scale: f32);
    fn htmlRender(&mut self, scale: i8) -> String;
    fn render_masks(&mut self) {
        // todo delete element after creating mask!
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas0 = document.create_element("canvas").unwrap();
        canvas0.set_attribute("style", "border:thin solid green; position:absolute; top:-1000px");
        document.body().unwrap().append_child(&canvas0).unwrap();
        let canvas = canvas0.dyn_into::<HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("2d").unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let (width, height) = self.GetSize();
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
      //  context.style
            //    canvas.style().set_property("border","thin solid red");

// todo
        self.RenderOnMainCanvas(RenderParams { params: vec![],parametersPool:vec![], x: 0.0, y: 0.0, id: self.getId(), scrollX:0.0, scrollY: 0.0, onlymain: false, formask: false, hovered: false }, Rc::new(context.clone()));

        let pp3=canvas.to_data_url().unwrap();
        self.SetIcon(pp3);
        context.clear_rect(0.0, 0.0, width as f64, height as f64);

        self.RenderOnMainCanvas(RenderParams {params: vec![],parametersPool:vec![], x: 0.0, y: 0.0, id: self.getId(), scrollX:0.0, scrollY: 0.0, onlymain: true, formask: true, hovered: false }, Rc::new(context.clone()));
        let picture = context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
        let newshape: Vec<u8> = picture.data().to_vec();
        self.SetBitMask(AllMasks::MaskMain, self.getMaskFromCanvas(newshape.clone(), AllMasks::MaskMain).unwrap());
        self.SetBitMask(AllMasks::MaskEmpty, self.getMaskFromCanvas(newshape.clone(), AllMasks::MaskMain).unwrap()*0);
        context.clear_rect(0.0, 0.0, width as f64, height as f64);

        let rp = RenderParams {params: vec![], x: 0.0, y: 0.0, parametersPool:vec![], scrollX:0.0, id: self.getId(), scrollY: 0.0,onlymain: false, formask: true, hovered: false };
        self.RenderSubzones(Rc::new(context.clone()), rp.clone(), AllMasks::MaskInputs);
        self.SetBitMask(AllMasks::MaskInputs, self.getMaskFromAll(Rc::new(context.clone()), AllMasks::MaskInputs).unwrap());

        context.clear_rect(0.0, 0.0, width as f64, height as f64);
        self.RenderSubzones(Rc::new(context.clone()), rp.clone(), AllMasks::MaskOutputs);
        self.SetBitMask(AllMasks::MaskOutputs, self.getMaskFromAll(Rc::new(context.clone()), AllMasks::MaskOutputs).unwrap());

        context.clear_rect(0.0, 0.0, width as f64, height as f64);
        self.RenderSubzones(Rc::new(context.clone()), rp.clone(), AllMasks::MaskParameters);
        self.SetBitMask(AllMasks::MaskParameters, self.getMaskFromAll(Rc::new(context.clone()), AllMasks::MaskParameters).unwrap());

        context.clear_rect(0.0, 0.0, width as f64, height as f64);
        self.RenderSubzones(Rc::new(context.clone()), rp.clone(), AllMasks::MaskBody);
        self.SetBitMask(AllMasks::MaskBody, self.getMaskFromAll(Rc::new(context.clone()), AllMasks::MaskBody).unwrap());

        context.clear_rect(0.0, 0.0, width as f64, height as f64);
        self.RenderSubzones(Rc::new(context.clone()), rp, AllMasks::MaskSettings);
        self.SetBitMask(AllMasks::MaskSettings, self.getMaskFromAll(Rc::new(context.clone()), AllMasks::MaskSettings).unwrap());
        // context.clear_rect(0.0, 0.0, width as f64, height as f64);
    }


    fn getHoverInfo(&self, x: usize, y: usize,element_id:u32) -> Option<HoverStatus> {
        let config=self.GetConfig();
        let mask_main = self.GetBitMask(AllMasks::MaskMain);
        let mask_parameters = self.GetBitMask(AllMasks::MaskParameters);
        let mask_inputs = self.GetBitMask(AllMasks::MaskInputs);
        let mask_outputs = self.GetBitMask(AllMasks::MaskOutputs);
        let mask_body = self.GetBitMask(AllMasks::MaskBody);
        let mask_settings = self.GetBitMask(AllMasks::MaskSettings);
        if y >= 0 && y < mask_main.shape()[0] && x < mask_main.shape()[1] && x >= 0 {
            return Some((
                element_id,
                mask_main[[y, x]] .try_into().unwrap(),
                if config.withInputs && y < mask_inputs.shape()[0] && x < mask_inputs.shape()[1] { mask_inputs[[y, x]] } else { 0 } as u8,
                if config.withOutputs && y < mask_outputs.shape()[0] && x < mask_outputs.shape()[1] { mask_outputs[[y, x]] } else { 0 } as u8,
                if config.withParameters && y < mask_parameters.shape()[0] && x < mask_parameters.shape()[1] { mask_parameters[[y, x]] } else { 0 } as u8,
                if config.withBody && y < mask_body.shape()[0] && x < mask_body.shape()[1] { mask_body[[y, x]] } else { 0 } as u8,
                if config.withSettings && y < mask_settings.shape()[0] && x < mask_settings.shape()[1] { mask_settings[[y, x]] } else { 0 } as u8
            ));
        }
        None
    }

    fn getMaskFromCanvas(&self, newshape: Vec<u8>, code: AllMasks) -> DataMask {
        let (width, height) = self.GetSize();
        let newshape1:Vec<u32>=newshape.iter().map(|e|*e as u32).collect();
        let arr = Array3::from_shape_vec((height, width, 4 as usize), newshape1.clone()).unwrap();
        let arrMatrix0 = Array2::from_elem((height , width), 0);
        let l3 = &arr.index_axis(Axis(2), 3) + &arrMatrix0.clone();
        let l2 = &arr.index_axis(Axis(2), 2) + &arrMatrix0.clone();
        let l1 = &arr.index_axis(Axis(2), 1) + &arrMatrix0.clone();
        let l0 = &arr.index_axis(Axis(2), 0) + &arrMatrix0.clone();
        let mut lsum2= &l0+&l1+&l2+&l3;
        let mask2 = lsum2.mapv(|e| {
            if e!=0 { return 1 as u128;}
            else { return 0 as u128;}
        });
        Some(mask2)
    }

    fn getMaskFromAll(&self, context: Rc<web_sys::CanvasRenderingContext2d>, code: AllMasks) -> DataMask {
        let (width, height) = self.GetSize();
        let picture = context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
        let newshape: Vec<u8> = picture.data().to_vec();
        let mask: DataMask = self.getMaskFromCanvas(newshape.clone(), code);
        mask
    }

    fn drawOnCanvas(&self, context: Rc<web_sys::CanvasRenderingContext2d>, mask: Array2<u8>, x: i32, y: i32) {
        console_log!("draw  mask on canvas {:?} {:?} {:?}",x,y, mask.shape());
        // let mask1 = mask.clone();
        let awith = mask.shape();
        let awith0 = awith.clone();
        let newwidth = awith[0] * awith[1];
        let shaped = mask.clone().into_shape(newwidth).unwrap();
        let shaped01 = shaped.clone();
        // let shaped02 = shaped.clone();
        // let ts=shaped;
        // let clamped_buf: Clamped<&[u8]> = Clamped(shaped01.as_slice().unwrap());
        // =clamped_buf.into_iter(e|{})
        let mut newarr: Vec<u8> = vec![];
        for symb in shaped.clone().as_slice().unwrap().iter() {
            let csym = *symb;
            // let sym:f64=csym.into();
            if csym != 0 {
                newarr.push(200);
                newarr.push(200);
                newarr.push(200);
                newarr.push(200);
            } else {
                newarr.push(1);
                newarr.push(1);
                newarr.push(1);
                newarr.push(10);
            }
        }
        let toclump = &newarr[..];
        let claumpedb64: Clamped<&[u8]> = Clamped(toclump);
        let image_data_temp = ImageData::new_with_u8_clamped_array_and_sh(claumpedb64, awith0[0] as u32, awith0[1] as u32).unwrap();
        context.set_fill_style(&JsValue::from_str("yellow"));
        context.put_image_data(&image_data_temp, x.into(), y.into()).unwrap();
    }
}


pub(crate) trait PaletteElement {
    fn render(self);
    fn update(self);
}
