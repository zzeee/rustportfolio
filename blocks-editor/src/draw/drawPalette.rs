use std::rc::Rc;
use std::sync::{ Mutex, RwLock};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Document};
use crate::console_log;
use crate::utils::main::{log};
use crate::types::state::{ Istate };
use crate::types::color::{Color};
use crate::types::layersStorage::{DocumentStorage, PaletteElements};

use wasm_bindgen::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use crate::draw::mouse::parseEvent;
use crate::types::events::{BlockEvent, EventTypes, EventUpdator, EventUpdators};
use crate::types::general::ActiveElement;
use crate::types::updators::{UpdatorChannels, UpdatorsStorage};


pub fn updateupdate(document: &Document, state: Rc<RwLock<Istate>>, rc1: Rc<RwLock<DocumentStorage>>, rc2: Rc<Mutex<PaletteElements>>, res: BlockEvent, senders: Vec<EventUpdator>, us: Rc<RwLock<UpdatorsStorage>>) {
   // console_log!("updateupdate");
    let mut usState=us.write().unwrap();
    usState.updateAll();
   // console_log!("stt {:?}", stator);
}
pub fn update(document: &Document, state: Rc<RwLock<Istate>>, rc1: Rc<RwLock<DocumentStorage>>, rc2: Rc<Mutex<PaletteElements>>, res: BlockEvent,  us: Rc<RwLock<UpdatorsStorage>>) {
    // console_log!("drawPalette update!");
    let infoZone = document.get_element_by_id("controlPaletteZone").unwrap();
      //  console_log!("bb");

    // console_log!("redraw palette!!! {:?} {:?}", res, state);
    drawOrRedrawSave(&document.clone(), state.clone());
         //  console_log!("cc");

    drawOrRedrawReset(&document.clone(), state.clone(), rc1.clone(), rc2.clone(), us.clone(), );
            //    console_log!("dd");
    // console_log!("drawPalette completed!");


}


pub fn drawPalette(document: &Document, state: Rc<RwLock<Istate>>, dc: Rc<RwLock<DocumentStorage>>, rcElementsPalette: Rc<Mutex<PaletteElements>>,
               //    senders: EventUpdators,
                //   receiver: Receiver<BlockEvent>,
                   updators: Rc<RwLock<UpdatorsStorage>>,
) ->   ActiveElement {
        let (sender,receiver ):(Sender<BlockEvent>, Receiver<BlockEvent>) = unbounded();

    let receiver0=receiver.clone();
    let document01=document.clone();
    let document02=document.clone();
    // let senders00=senders.clone();
    let state01 = state.clone();
    let state2 = state.clone();
    let rcDocumentStorage01 = dc.clone();
    let rcDocumentStorage03 = dc.clone();
    let rcElementsPalette01 = rcElementsPalette.clone();
    let rcElementsPalette03 = rcElementsPalette.clone();
   // let senders0=senders.clone();
    // let updators01=updators.clone();
    let updators02=updators.clone();
    let updators03=updators.clone();
    let updators04=updators.clone();
  //  let us_1=updators.clone();
   /*
    let cc2 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            let rt = receiver.try_recv();
            if let Ok(res) = rt {
                // console_log!("event! {:?}", res);
                //if res.code==2 {
                    update(&document01.clone(), state01.clone(), rcDocumentStorage01.clone(), rcElementsPalette01.clone(), res, updators01.clone(), );
                    // console_log!("active el {:?}", state01.clone() );
               // }
            }
        }) as Box<dyn FnMut(_)>);

    document.body().unwrap().add_event_listener_with_callback("mousemove", cc2.as_ref().unchecked_ref()).unwrap();
    document.body().unwrap().add_event_listener_with_callback("mouseup", cc2.as_ref().unchecked_ref()).unwrap();
    document.body().unwrap().add_event_listener_with_callback("mousedown", cc2.as_ref().unchecked_ref()).unwrap();
    cc2.forget();*/

    let paletteZone = document.create_element("div").unwrap();
    let paletteZone2 = document.create_element("div").unwrap();
    let paletteZone3 = document.create_element("div").unwrap();
    // infoZone.append_child(&canvas0).unwrap();
    paletteZone.append_child(&paletteZone2).unwrap();
    paletteZone.append_child(&paletteZone3).unwrap();
    paletteZone.set_id("paletteZone");
    paletteZone2.set_id("controlPaletteZone");
    paletteZone3.set_id("controlPaletteZone_slot");

    paletteZone2.set_inner_html("<div style='border: thin solid red'>PALETTEZONE</div>");
    document.body().unwrap().append_child(&paletteZone).unwrap();



   // drawColorChooser(&document,state.clone(),&"Unknow".to_string(),Color{c1:100,c2:100,c3:100});
   //  drawColorChooser(&document,state.clone(),&"Red".to_string(),Color{c1:255,c2:0,c3:0});
   //   drawColorChooser(&document,state.clone(),&"Blue".to_string(),Color{c1:0,c2:0,c3:255});
  //   drawColorChooser(&document,state.clone(),&"Lime".to_string(),Color{c1:0,c2:255,c3:0});
    // drawTestTool(state.clone());
    // let state = &*(rc0.read().unwrap().deref().clone());

    drawOrRedrawSave(&document.clone(), state.clone());

    //drawOrRedrawReset(&document.clone(), state.clone(), senders00.clone() );
    drawOrRedrawReset(&document.clone(), state.clone(), dc.clone(), rcElementsPalette.clone(),  updators04.clone()); //

    drawRefresh(&document.clone(), state.clone(), dc.clone(), rcElementsPalette.clone(), updators02.clone());

    drawCallUpdate(&document.clone(), state.clone(), dc.clone(), rcElementsPalette.clone(),updators03.clone() );

    drawElements(&document.clone(), state.clone(), dc.clone(), rcElementsPalette.clone(),   updators03.clone()  );

    let mut cca = 0;
    (Box::new(move || {
        // console_log!("update_palette {:?}",cca);
        cca = cca + 1;
          let rt = receiver.try_recv();
            if let Ok(res) = rt {
                // console_log!("update_palette event received {:?}",cca);

                update(&document02.clone(), state2.clone(), rcDocumentStorage03.clone(), rcElementsPalette03.clone(), BlockEvent { code: EventTypes::RedrawCanvas, subcode: 0 }, updators03.clone());
            }
    }),sender,receiver0)
    // drawElements1(&document,rc0.clone());
}

 pub fn drawElements(document: &Document, state000: Rc<RwLock<Istate>>,
                     dc: Rc<RwLock<DocumentStorage>>, rcElementsPalette: Rc<Mutex<PaletteElements>>,
                     uc: Rc<RwLock<UpdatorsStorage>>)
                    {
     console_log!("draw palette7_1");
     let uc0=uc.clone();
     let mut state = rcElementsPalette.lock().unwrap();
     // let mut all: Vec<String> = vec![];
     // let state011 = state000.clone();
     // let rcDocStorage1 = dc.clone();
     // let rcPalette1 = rcElementsPalette.clone();
     /* let cc5 = Closure::wrap(Box::new(
         move |event: web_sys::MouseEvent| {
             let res = aSender.clone().try_send(BlockEvent { code: 1, subcode: 0 });
             let res2 = aCSender.clone().try_send(BlockEvent { code: 1, subcode: 0 });
             parseEvent(event, state011.clone(), rcDocStorage1.clone(), rcPalette1.clone(), BlockEvent { code: 1, subcode: 0 });
         }) as Box<dyn FnMut(_)>);
     let button = document.create_element("button").unwrap();
     button.set_inner_html("Rectangles");
     button.set_id("dadada");
     document.body().unwrap().append_child(&button).unwrap();
     button.add_event_listener_with_callback("mousedown", cc5.as_ref().unchecked_ref()).unwrap();
     cc5.forget(); */

     for  val0 in state.iter() {
         let state01=state000.clone();
         let rcDocStorage=dc.clone();
         let rcPalette=rcElementsPalette.clone();
         let mut valval = val0.lock().unwrap();
         let mut rendered = valval.htmlRender(1);
         let subcode = valval.GetKind();
         let button = document.create_element("button").unwrap();
         button.set_inner_html(&rendered);
         document.body().unwrap().append_child(&button).unwrap();

         let iUC=uc0.clone();

         let cc4 = Closure::wrap(Box::new(
             move |event: web_sys::MouseEvent| {
                 console_log!("element clicjked!");


              parseEvent(event, state01.clone(), rcDocStorage.clone(), rcPalette.clone(), BlockEvent {code:EventTypes::ActiveElementChanged,subcode}, iUC.clone());
             }) as Box<dyn FnMut(_)>);
         button.add_event_listener_with_callback("mousedown", cc4.as_ref().unchecked_ref()).unwrap();
         cc4.forget();

     }
     // let libe = all.concat();
     // console_log!("total!+ libe:{:?}  ", libe);
     /* let cc2 = Closure::wrap(Box::new(
           move |event: web_sys::MouseEvent| {
               mousedown(event, state02.clone(), context0.clone(),rcDocumentStorage01.clone(), rcElementsPalette01.clone());
           }) as Box<dyn FnMut(_)>);
       canvas.add_event_listener_with_callback("mousedown", cc2.as_ref().unchecked_ref()).unwrap();
       cc2.forget();*/
 }
pub fn drawTestTool( rc: Rc<RwLock<Istate>>) {
    console_log!("drawTestTool");
    let mut state = rc.try_write().unwrap();
    let closure2 = Box::new(move |mut rc0:Istate| -> String {
        // console_log!("isp {:?} ",  rc0.counter);
        // rc0.incCounter();
        format!("A{:?}",rc0.counter )
    }) as Box<dyn FnMut(Istate)->String>;

}
pub fn drawTool(document: &Document, rc: Rc<RwLock<Istate>>, code:i32, name:String, value:i32) {
    let button = document.create_element("button").unwrap();
    button.set_inner_html(&name);
    document.body().unwrap().append_child(&button).unwrap();

    let cc1 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            log("drawTool");
              let mut state = rc.write().unwrap();
            (*state).setActiveTool(code);
        }) as Box<dyn FnMut(_)>);
    let cc_temo = cc1.as_ref().unchecked_ref();
    button.add_event_listener_with_callback("click", cc_temo).unwrap();
    cc1.forget();
}

pub fn drawOrRedrawSave(document: &Document, rc: Rc<RwLock<Istate>>) {
    let element=document.get_element_by_id("palette_save_button");
    if let Some(el)=element {
       // console_log!("Sv1");
        let state=rc.read().unwrap();
        // console_log!("need to redraw {:?}", state.activeElement);
        let line=format!("Save {:?}",state.activeElement);
        el.set_inner_html(&line);
    } else {
         //       console_log!("Sv2");

        let button = document.create_element("button").unwrap();
        button.set_inner_html(&"Save");
        button.set_id("palette_save_button");
        let insertzone = document.get_element_by_id("controlPaletteZone_slot").unwrap();
        //document.body().unwrap().append_child(&button).unwrap();
        insertzone.append_child(&button).unwrap();

        let cc1 = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                log("save");
                // let mut state = rc.write().unwrap();

                // let data = state.documentStorageAsStr();
                // state.sendMessage(&data);
                // rc.set(state);
                // log(&format!("drawTool: {:?}",  state));

                // getOnClick(event, state01.clone());
            }) as Box<dyn FnMut(_)>);
        let cc_temo = cc1.as_ref().unchecked_ref();
        button.add_event_listener_with_callback("click", cc_temo).unwrap();
        cc1.forget();
        // console_log!("need to draw");
    }


}

pub fn drawRefresh(document: &Document, rc: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) {
    let state=rc.clone();
    let ds0 =ds.clone();
    let ps0 =ps.clone();
    let us0=us.clone();
    let document0=document.clone();
        let element = document.get_element_by_id("palette_refresh");
    if let Some(el) = element { }
    else {
         let button = document.create_element("button").unwrap();
        button.set_inner_html(&"Refresh");
        button.set_id("palette_refresh");
        let insertzone = document.get_element_by_id("controlPaletteZone_slot").unwrap();
        insertzone.append_child(&button).unwrap();
         let cc1 = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {


                console_log!("state {:?}", state.clone());
              update(&document0.clone(), state.clone(), ds0.clone(), ps0.clone(), BlockEvent {code:EventTypes::RefreshAll,subcode:3},  us0.clone())

            }) as Box<dyn FnMut(_)>);
        let cc_temo = cc1.as_ref().unchecked_ref();
        button.add_event_listener_with_callback("click", cc_temo).unwrap();
        cc1.forget();
     }
    }
pub fn drawCallUpdate(document: &Document, rc: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) {
    let state=rc.clone();
    let ds0 =ds.clone();
    let ps0 =ps.clone();
    let us0=us.clone();
    let document0=document.clone();
        let element = document.get_element_by_id("palette_updator");
    if let Some(el) = element { }
    else {
         let button = document.create_element("button").unwrap();
        button.set_inner_html(&"Update");
        button.set_id("palette_updator");
        let insertzone = document.get_element_by_id("controlPaletteZone_slot").unwrap();
        insertzone.append_child(&button).unwrap();
         let cc1 = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                console_log!("drawCallUpdate {:?}", state.clone());

             updateupdate(&document0.clone(), state.clone(), ds0.clone(), ps0.clone(), BlockEvent {code:EventTypes::RefreshAll,subcode:3}, vec![], us0.clone())

            }) as Box<dyn FnMut(_)>);
        let cc_temo = cc1.as_ref().unchecked_ref();
        button.add_event_listener_with_callback("click", cc_temo).unwrap();
        cc1.forget();
     }
    }


pub fn drawOrRedrawReset(document: &Document, rc: Rc<RwLock<Istate>>, ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) {
    let element = document.get_element_by_id("palette_reset");
    let us0_1=us.clone();
    if let Some(el) = element {
        let rc00=rc.clone();
        let state = rc00.read().unwrap();
        let line = format!("\
        Reset!! {:?}", state.activeElement);
        // console_log!("{:?}", line);
        el.set_inner_html(&line);
    } else {
        let rc00=rc.clone();

        let button = document.create_element("button").unwrap();
        button.set_inner_html(&"Reset");
        button.set_id("palette_reset");
        let insertzone = document.get_element_by_id("controlPaletteZone_slot").unwrap();
        insertzone.append_child(&button).unwrap();
        // document.body().unwrap().append_child(&button).unwrap();
        let cc1 = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                {
                    {
                        let mut stateA = rc00.write().unwrap();
                        // console_log!("current:{:?} ", stateA.activeTool);
                        (*stateA).setActiveElement(0);
                        // update(&document0.clone(), state.clone(), ds0.clone(), ps0.clone(), BlockEvent{code:32,subcode:3}, vec![])
                        // console_log!("current2:{:?} {:?}", stateA.activeTool, stateA);
                    }
                    if let Ok(mut updators) = us0_1.write() {
                        updators.sendOnlySignalByEvent(BlockEvent { code: EventTypes::ActiveElementChanged, subcode: 0 });
                        updators.updateAll();
                       // updators.updateAllButOne(UpdatorChannels::PaletteMain);
                    }

                }
             //   allSenders.clone().into_iter().all(|x| {x.event.try_send(BlockEvent { code: EventTypes::ActiveElementChanged, subcode: 0 }); true});

            }) as Box<dyn FnMut(_)>);
        let cc_temo = cc1.as_ref().unchecked_ref();
        button.add_event_listener_with_callback("click", cc_temo).unwrap();
        cc1.forget();
    }
}

pub fn drawOrRedrawScale(document: &Document, rc: Rc<RwLock<Istate>>) {
    let element = document.get_element_by_id("palette_scale");
   // let allSenders=vec.clone();
    if let Some(el) = element {
        let state = rc.read().unwrap();
        // console_log!("need to redraw {:?}", state.activeElement);
        let line = format!("Reset {:?}", state.activeElement);
        el.set_inner_html(&line);
    } else {
        let button = document.create_element("button").unwrap();
        button.set_inner_html(&"Reset");
        button.set_id("palette_scale");
        document.body().unwrap().append_child(&button).unwrap();
        let cc1 = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                log("ResAAet");
                let mut state = rc.write().unwrap();

                console_log!("currenWWt:{:?} |", state.activeTool);

                (*state).setActiveTool(0);
                // allSenders.clone().into_iter().all(|x| {x.event.try_send(BlockEvent { code: EventTypes::Rescale, subcode: 0 }); true});
            }) as Box<dyn FnMut(_)>);
        let cc_temo = cc1.as_ref().unchecked_ref();
        button.add_event_listener_with_callback("click", cc_temo).unwrap();
        cc1.forget();
    }
}


/*
pub fn drawButton(document: &Document, rc: Rc<Cell<Istate>>) {
    let button = document.create_element("button").unwrap();
    button.set_inner_html("TESTTEST");
    document.body().unwrap().append_child(&button).unwrap();
    let cc1 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
          //  let mut state = rc.get();
          //  state.paletteClicked = state.paletteClicked + 1;
          //  rc.set(state);
          //  log(&format!("clickedinside: {:?} {:?}",  state.paletteClicked, state));
        }) as Box<dyn FnMut(_)>);
    let cc_temo = cc1.as_ref().unchecked_ref();
    button.add_event_listener_with_callback("click", cc_temo).unwrap();
    cc1.forget();
} */
pub fn drawColorChooser(document: &Document, rc: Rc<RwLock<Istate>>, name: &String, value: Color) {
    let button = document.create_element("button").unwrap();
    button.set_inner_html(name);
    document.body().unwrap().append_child(&button).unwrap();

    let cc1 = Closure::wrap(Box::new(
        move |event: web_sys::MouseEvent| {
            console_log!("click on color chooser");
            let mut state = rc.write().unwrap();
            state.setActiveColor(value);
        }) as Box<dyn FnMut(_)>);
    let cc_temo = cc1.as_ref().unchecked_ref();
    button.add_event_listener_with_callback("click", cc_temo).unwrap();
    cc1.forget();
}
