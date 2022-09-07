use std::cell::Cell;
use std::rc::Rc;
use web_sys::{ImageData, Path2d};
use wasm_bindgen::{Clamped, JsCast};
use ndarray::{array, Array2,Array3, Axis};
use wasm_bindgen::prelude::*;
use crate::utils::main::{log};
use crate::types::state::{Istate};

/*
pub(crate) fn getOnClick(event: web_sys::MouseEvent, mut rc: Rc<Cell<Istate>>) {
    let mut value = rc.get(); // buttonPressed
    value.buttonPressed = true;
    value.counter = value.counter + 1;
    rc.set(value);
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas0 = document.get_element_by_id("testtets").unwrap();
    let canvas = canvas0.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas
        .get_context("2d").unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    //let context = context.clone();

    let rt: ImageData = context.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
    let ares = rt.data();
    let frm = format!("aa {:?} vvv", ares);
    let newshape = ares.to_vec();
    //     log(&format!("aa {:?} vvv {:?}", newshape,newshape.len()));

    let arr = Array3::from_shape_vec((100, 100, 4), newshape).unwrap();

    let arr01 = ((arr + 50) * 2);
    let arr2 = arr01.t(); //.reversed_axes();
    let arr3 = arr2.to_shape(40000).unwrap();
    let mut arr4 = arr3.to_vec();
    let mut slice_data = Clamped(&arr4[..]);
    let imageData = ImageData::new_with_u8_clamped_array_and_sh(slice_data, 100, 100).unwrap();
    context.put_image_data(&imageData, 50.0, 50.0);

    log("btn");
    // log(&frm);
    // log(&format!("aa {:?} vvv ", arr2));
}
*/
