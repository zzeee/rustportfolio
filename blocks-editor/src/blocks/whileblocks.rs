use std::f64::consts::PI;
use std::fmt;
use std::rc::Rc;
use crate::blocks::blocks::{PaletteElement,BlockParameterVar, BlockControl, RenderParams,  AllMasks, BlockConfig};
use ndarray::{ Array2};
use wasm_bindgen::{ JsValue};
use web_sys::ImageData;
use crate::{console_log, DeclateBasicBlockStorage, BasicBlock, GetSetBitMaskBasic, GetSetIconBasic, GetSetHoverBasic };
use crate::utils::main::log;
use crate::main::collisions::printShape;
use std::sync::{Arc, Mutex};

DeclateBasicBlockStorage!(WhileBlock);

impl PaletteElement for WhileBlock {
    // fn init(self) {}
    fn render(self) {}
    fn update(self) {}
}


impl BlockControl for WhileBlock {
    /* fn new(name: String, kind:i32, scale:i8) -> Self {
        WhileBlock {name, kind, scale, width:0, height:9, clickZones:vec![], iconPicture: Default::default(), inputsBitmask:vec![], Bitmask: Default::default() }
    }
*/
    BasicBlock!();
    fn init(&mut self, scale: f32) {
        console_log!("initing while");
        self.Scale(scale);
        self.render_masks();
    }
    fn GetConfig(&self) -> BlockConfig {
       BlockConfig::new(true, true, true, true, true)
   }

    fn RenderOnMainCanvas(&self, params: RenderParams, ctx: Rc<web_sys::CanvasRenderingContext2d>) {
        let x = params.x;
        let y = params.y;
        let hovered = params.hovered;
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(if params.formask {"black"} else {"magenta"}));
        ctx.fill_text("IF", x.into(), y.into());
        ctx.arc((x + 50.0).into(), (y + 50.0).into(), 50.0, 0.0, (2.0 * PI).into());
        ctx.fill();
        if hovered {
            ctx.set_fill_style(&JsValue::from_str("red"));
            ctx.stroke_rect((x - 1.0).into(), (y - 11.0).into(), (102.0).into(), (112.0).into());
            ctx.stroke()
        }
        ctx.move_to(x.into(), y.into());
        // if !params.formask {
        self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskInputs);
        self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskOutputs);
        self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskParameters);
        self.RenderSubzones(ctx.clone(), params.clone(), AllMasks::MaskBody);
        //  }
    }

    fn RenderSubzones(&self, ctx: Rc<web_sys::CanvasRenderingContext2d>, params: RenderParams, subzone: AllMasks) {
        match subzone {
            AllMasks::MaskInputs => { // inputs
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask {"black"} else {"green"}));
                ctx.arc((params.x + 50.0).into(), (params.y + 10.0).into(), 10.0, 10.0, PI.into());
                ctx.fill();
            }
            AllMasks::MaskOutputs => { // outputs
                ctx.begin_path();
                ctx.set_fill_style(&JsValue::from_str(if params.formask {"black"} else {"blue"}));
                ctx.arc((params.x + 10.0).into(), (params.y + 50.0).into(), 10.0, 0.0, (2.0 * PI).into());
                ctx.fill();

                ctx.set_fill_style(&JsValue::from_str(if params.formask { "black" } else { "white" }));
                ctx.fill_text("O", (params.x + 10.0).into(), (params.y + 50.0).into());
            }
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
        format!("<div>IF</div><img src={:?} />",rs)
    }

}
