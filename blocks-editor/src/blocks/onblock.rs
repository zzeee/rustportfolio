use std::{f32::consts::PI, fmt, rc::{Rc}};
use wasm_bindgen::{ JsCast,JsValue};
use web_sys::{HtmlCanvasElement, ImageData};
use ndarray::{  Array2, Array3,  Axis };
use crate::{console_log,DeclateBasicBlockStorage, BasicBlock, GetSetBitMaskBasic, GetSetIconBasic, GetSetHoverBasic, loglog, utils::main::log, blocks::blocks::{AllMasks,BlockParameterVar, PaletteElement, BlockControl, RenderParams }};
use crate::blocks::blocks::{BlockConfig, HoverStatus};
DeclateBasicBlockStorage!(OnBlock);
use crate::main::collisions::printShape;
use std::sync::{Arc, Mutex};

impl PaletteElement for OnBlock {
    // fn init(self) {}
    fn render(self) {}
    fn update(self) {}
}

impl BlockControl for OnBlock {
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
        BlockConfig::new(false, false, true, true, true)
    }

    fn RenderOnMainCanvas(&self, params: RenderParams, ctx: Rc<web_sys::CanvasRenderingContext2d>) {
        let x=params.x;
        let y=params.y;
        let hovered=params.hovered;
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("blue"));
        let (width, height) = self.GetSize();
        ctx.fill_text("ON START", x.into(), y.into());
        ctx.arc((x + 50.0).into(), (y + 50.0).into(), 50.0, 0.0, (2.0 * PI).into());
        ctx.fill();

        if hovered {
             ctx.set_fill_style(&JsValue::from_str("red"));
             ctx.stroke_rect((x-1.0).into(), (y-11.0).into(), (width as f32+2.0).into(), (height as f32+2.0).into());
             //ctx.arc(x.into(), y.into(), 150.0, 0.0, (2.0 * PI).into());
             ctx.stroke()
         }
        let config=self.GetConfig();
        if config.withParameters { self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskParameters); }
        if config.withBody { self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskBody); }
        if config.withInputs { self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskInputs); }
        if config.withOutputs { self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskOutputs); }
    }


   fn RenderSubzones(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, params: RenderParams, subzone: AllMasks) {
        match subzone {
            AllMasks::MaskParameters => { // parameters
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "red" }));

                ctx.arc((params.x + 50.0).into(), (params.y + 80.0).into(), 10.0, 10.0, PI.into());
                ctx.fill();
                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                ctx.fill_text("P", (params.x + 50.0).into(), (params.y + 80.0).into());
            }
            AllMasks::MaskBody => { // body
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask {"black"} else {"yellow"}));
                ctx.arc((params.x + 80.0).into(), (params.y + 50.0).into(), 10.0, 10.0, PI.into());
                ctx.fill();
            }
            _ => {}
        }
    }

    fn Scale(&mut self, scale: f32) {
        self.scale = scale;
        self.width = 100;
        self.height = 110;
    }


     fn htmlRender(&mut self, scale: i8) -> String {
        let rs=self.GetIcon();
        format!("<div>ON</div><img src={:?} />",rs)
    }
}
