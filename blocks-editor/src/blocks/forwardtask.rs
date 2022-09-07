use std::{f32::consts::PI, fmt, rc::{Rc}};
// use js_sys::Math::sqrt;
use wasm_bindgen::{ JsCast,JsValue};
use web_sys::{HtmlCanvasElement, ImageData};
use ndarray::{  Array2, Array3,  Axis };
use crate::{console_log, DeclateBasicBlockStorage, BasicBlock, GetSetBitMaskBasic, GetSetIconBasic, GetSetHoverBasic, loglog, utils::main::log, blocks::blocks::{AllMasks,BlockParameterVar, PaletteElement, BlockControl, RenderParams }};
use crate::blocks::blocks::{BlockConfig, HoverStatus};
DeclateBasicBlockStorage!(ForwardTaskBlock);
const COLOR:&str="#8ED6FF";
use crate::main::collisions::printShape;
use std::sync::{Arc, Mutex};

impl PaletteElement for ForwardTaskBlock {
    // fn init(self) {}
    fn render(self) {}
    fn update(self) {}
}

impl BlockControl for ForwardTaskBlock {
    /*  fn new(p[=name: String, kind:i32, scale:i8) -> IfBlock {
          IfBlock {name, kind, scale, width:0, height:9, clickZones:vec![], iconPicture: Default::default(), inputsBitmask:vec![], Bitmask: Default::default() }
      }
  */
    BasicBlock!();
    fn init(&mut self, scale:f32) {
        self.Scale(scale);
        self.render_masks();
    }
    fn GetConfig(&self) -> BlockConfig {
        BlockConfig::new(true, true, true, true, true)
    }

    fn RenderOnMainCanvas(&self, params: RenderParams, ctx: Rc<web_sys::CanvasRenderingContext2d>) {
        let xbase = params.x;
        let ybase = params.y;
        let id=params.id;
        let scrollX = params.scrollX;
        let scrollY = params.scrollY;
        if  xbase+self.height as f32-scrollY>=0.0 && ybase +(self.width as f32 )-scrollX>=0.0 {
            let x=xbase-scrollX;
            let y=ybase-scrollY;
            let hovered = params.hovered;
            ctx.begin_path();
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_text(&id.to_string(), (x  ).into(), (y ).into());
            ctx.set_fill_style(&JsValue::from_str(COLOR));
            let (width, height) = self.GetSize();
            let radius: f32 = self.size_param3 as f32;
            ctx.fill_rect((x + radius).into(), (y).into(), self.size_param1.into(), self.size_param2.into());
            ctx.fill();

            if hovered {
                ctx.set_fill_style(&JsValue::from_str("red"));
                ctx.stroke_rect((x - 1.0).into(), (y - 11.0).into(), (width as f32 + 2.0).into(), (height as f32 + 2.0).into());
                ctx.stroke();
                let document = web_sys::window().unwrap().document().unwrap();
                let drawZone = document.get_element_by_id("testtets").unwrap();
                let canvas = drawZone.dyn_into::<HtmlCanvasElement>().unwrap();
                canvas.style().set_property("cursor", "pointer");
            } else {
                // canvas.set_style(&JsValue::from_str("cursor:alias"));
                // canvas.style().set_property("cursor","alias");
            }
            ctx.close_path();
            let config = self.GetConfig();
            let mut newparams = params.clone();
            newparams.x = x;
            newparams.y = y;
            if config.withSettings { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskSettings); }
            if config.withParameters { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskParameters); }
            if config.withBody { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskBody); }
            if config.withInputs { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskInputs); }
            if config.withOutputs { self.RenderSubzones(ctx.clone(), newparams.clone(), AllMasks::MaskOutputs); }
        }
    }

    fn RenderSubzones(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, params: RenderParams, subzone: AllMasks) {
        let x=params.x;
        let y=params.y;
        match subzone {
            AllMasks::MaskSettings => {
                // console_log!("data MaskSettings {:?} {:?}",x,y);
                ctx.save();
                ctx.set_fill_style(&JsValue::from_str("grey"));
                let base = self.size_param3 as f64;
                let swidth: f64 = base * 1.0;
                let sradius: f64 = base / 5.0;
                ctx.translate((x as f32 + (self.size_param1 - self.size_param3 - 4.0) + swidth as f32).into(), (y as f32 + (self.size_param3 + 2.0) - swidth as f32).into());
                ctx.fill_rect(0.0, 0.0, swidth, swidth);
                ctx.rotate((45.0 * PI / 180.0).into());
                ctx.set_fill_style(&JsValue::from_str("grey"));
                let x1 = swidth / (2.0 * (1.0 + 2.0_f64.sqrt()));
                let y1 = swidth / 2.0;
                let d1 = swidth * 2.0_f64.sqrt() / 2.0;

                ctx.fill_rect(x1, -y1, swidth, swidth);
                ctx.set_fill_style(&JsValue::from_str("lightgrey"));
                ctx.arc(d1, 0.0, sradius, 0.0, (2.0 * PI).into());
                ctx.fill();
                //  ctx.fill_rect(0.0,0.0,20.0,20.0 );
                // ctx.translate(20.0,-5.0);

                ctx.rotate((-45.0 * PI / 180.0).into());
                ctx.restore();
                ctx.set_fill_style(&JsValue::from_str("white"));
            }// inputs
            AllMasks::MaskBody => { // inputs
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                ctx.arc((params.x + self.size_param3 as f32+self.size_param1 as f32/2.0).into(), (params.y + 0.0).into(), self.size_param3 as f64, 0.0, (2.0*PI).into());
                ctx.fill();

            }
            AllMasks::MaskInputs => { // outputs
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                ctx.arc((params.x + self.size_param3 as f32).into(), (params.y + self.size_param3 as f32*2.0).into(), self.size_param3 as f64, 0.0, (2.0 * PI).into());
                ctx.fill();
                ctx.close_path();

                //ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                // ctx.fill_text("O", (params.x + 10.0).into(), (params.y + 50.0).into());
            }
            AllMasks::MaskOutputs  => { // body
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { COLOR }));
                ctx.arc((params.x + self.size_param1 as f32+self.size_param3 as f32).into(), (params.y + self.size_param2 as f32/2.0).into(), self.size_param3 as f64, 00.0, (2.0 * PI).into());
                ctx.fill();
                ctx.close_path();


            }
            AllMasks::MaskParameters => { // parameters
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { COLOR }));

                ctx.arc((params.x  + self.size_param3 as f32+self.size_param1 as f32/2.0).into(), (params.y +  self.size_param2 as f32).into(), self.size_param3 as f64, 00.0, (2.0 * PI).into());
                ctx.fill();

            }
            AllMasks::MaskMain | AllMasks::MaskEmpty => {

            }


        }
    }
    fn Scale(&mut self, scale: f32) {
        self.scale = scale;

        self.size_param1=80.0*(scale);
        self.size_param2=40.0*(scale);
        self.size_param3=10.0*(scale);

        self.width =  (self.size_param1+2.0*self.size_param3) as usize;
        self.height =  (self.size_param2+2.0*self.size_param3) as usize;
        console_log!("SCALE W:{:?} H:{:?}", self.width,self.height )
    }



     fn htmlRender(&mut self, scale: i8) -> String {
        let rs=self.GetIcon();
        format!("<div>{}</div><img src={:?} />",self.name,rs)
    }
}
