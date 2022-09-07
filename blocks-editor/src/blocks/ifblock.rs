use std::{f32::consts::PI, fmt, rc::{Rc}};
use wasm_bindgen::{JsValue};
use web_sys::{HtmlCanvasElement, ImageData};
use ndarray::{Array2, Array3, Axis};
use std::sync::{Arc, Mutex};

use crate::{console_log, loglog, DeclateBasicBlockStorage, utils::main::log, BasicBlock, GetSetBitMaskBasic, GetSetHoverBasic, GetSetIconBasic, blocks::blocks::{BlockParameterVar,AllMasks, PaletteElement, BlockControl, RenderParams}};
use crate::blocks::blocks::{BlockConfig, HoverStatus};
DeclateBasicBlockStorage!(IfBlock);
const COLOR:&str="#FF9797";
use crate::main::collisions::printShape;

impl PaletteElement for IfBlock {
    // fn init(self) {}
    fn render(self) {}
    fn update(self) {}
}

impl BlockControl for IfBlock {
     BasicBlock!();

    /*  fn new(p[=name: String, kind:i32, scale:i8) -> IfBlock {
          IfBlock {name, kind, scale, width:0, height:9, clickZones:vec![], iconPicture: Default::default(), inputsBitmask:vec![], Bitmask: Default::default() }
      }
  */
    fn init(&mut self, scale: f32) {
        self.Scale(scale);
        self.render_masks();
    }
    fn GetConfig(&self) -> BlockConfig {
        BlockConfig::new(true, true, false, false, true)
    }


    fn RenderOnMainCanvas(&self, params: RenderParams, ctx: Rc<web_sys::CanvasRenderingContext2d>) {

        let xbase = params.x;
        let ybase = params.y;
        let scrollX = params.scrollX;
        let scrollY = params.scrollY;
        if  xbase+self.height as f32-scrollY>=0.0 && ybase +(self.width as f32 )-scrollX>=0.0 {
            let x=xbase-scrollX;
            let y=ybase-scrollY;
            let hovered = params.hovered;
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str(COLOR));
            let (width, height) = self.GetSize();
            ctx.fill_text("DATA", (x + 20.0).into(), (y + 20.0).into());
            let radius: f32 = self.size_param3 as f32;

            ctx.fill_rect((x + radius).into(), (y).into(), self.size_param1.into(), self.size_param2.into());
            // ctx.arc((x + 50.0).into(), (y + 50.0).into(), 50.0, 0.0, (2.0 * PI).into());
            ctx.fill();

            if hovered {
                ctx.set_fill_style(&JsValue::from_str("red"));
                ctx.stroke_rect((x - 1.0).into(), (y - 11.0).into(), (width as f32 + 2.0).into(), (height as f32 + 2.0).into());
                //ctx.arc(x.into(), y.into(), 150.0, 0.0, (2.0 * PI).into());
                ctx.stroke()
            }
            let config = self.GetConfig();
            let mut newparams=params.clone();
            newparams.x=x;
            newparams.y=y;
            if config.withParameters { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskParameters); }
            if config.withBody { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskBody); }
            if config.withInputs { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskInputs); }
            if config.withOutputs { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskOutputs); }
        }
    }

    fn RenderSubzones(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, params: RenderParams, subzone: AllMasks) {
        match subzone {
            AllMasks::MaskBody => { // inputs
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { COLOR }));
                ctx.arc((params.x + self.size_param3 as f32+self.size_param1 as f32/2.0).into(), (params.y + 0.0).into(), self.size_param3 as f64, 0.0, (2.0*PI).into());
                ctx.fill();
            }
            AllMasks::MaskOutputs => { // outputs
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                ctx.arc((params.x + self.size_param3 as f32).into(), (params.y + self.size_param3 as f32*2.0).into(), self.size_param3 as f64, 0.0, (2.0 * PI).into());
                ctx.fill();

                //ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                // ctx.fill_text("O", (params.x + 10.0).into(), (params.y + 50.0).into());
            }
            AllMasks::MaskParameters => { // parameters
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { COLOR }));

                ctx.arc((params.x  + self.size_param3 as f32+self.size_param1 as f32/2.0).into(), (params.y +  self.size_param2 as f32).into(), self.size_param3 as f64, 00.0, (2.0 * PI).into());
                ctx.fill();
                // console_log!("params! {:?} {:?} {:?}",params.x  + self.size_param3 as f32+self.size_param1 as f32/2.0, params.y +  self.size_param2 as f32, self.size_param3 as f64);
                // ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                //  ctx.fill_text("P", (params.x + 50.0).into(), (params.y + 80.0).into());
            }
            AllMasks::MaskInputs  => { // body
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { COLOR }));
                ctx.arc((params.x + self.size_param1 as f32+self.size_param3 as f32).into(), (params.y + self.size_param2 as f32/2.0).into(), self.size_param3 as f64, 00.0, (2.0 * PI).into());
                ctx.fill();
            }
            _ => {}
        }
    }
    fn Scale(&mut self, scale: f32) {
        self.scale = scale;

        self.size_param1=80.0*(scale);
        self.size_param2=40.0*(scale);
        self.size_param3=10.0*(scale);

        self.width =  (self.size_param1+2.0*self.size_param3) as usize;
        self.height =  (self.size_param2+2.0*self.size_param3) as usize;
        console_log!("SCALE{:?} {:?}", self.width,self.height )
    }


    fn htmlRender(&mut self, scale: i8) -> String {
        let rs = self.GetIcon();
        format!("<div>{}</div><img src={:?} />", self.name, rs)
    }
}
