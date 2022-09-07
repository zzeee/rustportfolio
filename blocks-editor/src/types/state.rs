use std::fmt;
use crate::utils::main::{log};
use std::string::String;
use std::rc::{Rc};
use std::sync::{ Mutex, RwLock};
use web_sys::WebSocket;
use crate::blocks::blocks::HoverStatus;
use crate::types::color::{Color};
use crate::types::layersStorage::{  UIElement};
use crate::console_log;

pub trait RenderedElement {
    fn render(&mut self)->&str {
        log("rendered!");
        "wer"
    }
}

pub type RenderFnInside = &'static dyn Fn();

pub type RenderFn=Box<RenderFnInside>;



// #[derive( Clone)]

#[derive( Clone)]
pub struct Istate  {
    pub(crate) mousePressed: bool,
    pub(crate) buttonPressed: bool,
    pub(crate) moveMode: bool,
    pub(crate) movingElementId: u32,
    pub(crate) openSettingsElement: u32,
    pub(crate) stickyMode: bool,
    pub(crate) dragged: bool,
    pub(crate) mouseXclicked: i32,
    pub(crate) mouseYclicked: i32,
    pub(crate) canvasWidth:i32,
    pub(crate) canvasHeight:i32,
    pub(crate) scrollX: f32,
    pub(crate) scrollY: f32,
    pub(crate) dragElement: Option<u32>,
    pub(crate) paletteClicked: i32,
    pub(crate) chosenColor: Color,
    pub(crate) activeTool: i32,
    pub(crate) activeElement: u32,
    pub(crate) activeElementWidth: i32,
    pub(crate) activeElementHeight: i32,
    pub(crate) ws: Option<WebSocket>,
    pub(crate) counter: i8,
    pub(crate) hoveredStatus: HoverStatus,
   // pub(crate) rendered: Rc<Cell<Vec<RenderFn>>>
   // pub(crate) rendered: Rc<Mutex<Vec<RenderFnInside>>>
   // pub(crate) rendered: Rc<Mutex<Vec<fn()>>>
   // pub(crate) rendered: Rc<Mutex<Vec<Box<dyn Fn()->String>>>>
    pub(crate) rendered: Rc<RwLock<Vec<Box<dyn FnMut(Istate)->String>>>>,
    //let mut stack = DynStack::<dyn Debug>::new();

}


pub struct ElementsPalette {
    data: Vec<UIElement>,
}

impl Default for Istate {
    fn default() -> Self {
        Self {
            rendered: Rc::new(RwLock::new(vec![])),
           stickyMode:true,
           //  updators: Rc::new(RwLock::new(vec![])),
            chosenColor: Color { c1: 0, c2: 0, c3: 0 },
            scrollX:0.0,
            hoveredStatus:(0,0,0,0,0,0,0),
            dragElement:None,
            scrollY:0.0,
            moveMode:false,
            dragged:false,
            canvasWidth:800,
            canvasHeight:600,
            ws: None,
            mousePressed: false,
            movingElementId:0,
            paletteClicked: 0,
            buttonPressed: false,
            counter: 0,
            mouseXclicked: 0,
            mouseYclicked: 0,
            activeTool: 0,
            activeElement:0,
            openSettingsElement:0,

            activeElementWidth:0,
            activeElementHeight:0
        }
    }
}

impl Istate  {
    pub fn setActiveTool(&mut self, value: i32) {
        self.activeTool = value;
        console_log!("set active tool {:?}", self.activeTool );

        self.render();
    }

   /* pub fn incCounter(&mut self) {
        // console_log!("sds {:?}", self.counter);
       // self.counter=self.counter+1;
    } */
    // fn wrap(c: impl Fn()) {
    //c()

    pub fn openSettings(&mut self, element: u32) {
        self.openSettingsElement=element;
    }
    pub fn switchToMoveMode(&mut self, mode: bool) {
        self.moveMode = mode;
        //if !mode { self.movingElementId=0; }
    }

    pub fn setStickMode(&mut self, stickymode:bool) {
        self.stickyMode=stickymode;
    }
    pub fn setMoveElementId(&mut self, element_id: u32) {
        self.movingElementId = element_id
    }

    pub fn addElementToRender(&mut self, element: Box<dyn FnMut(Istate)->String>) {
        let mut rendered=self.rendered.write().unwrap();
        rendered.push(element);
    }

    /*
  pub fn addUpdator(&mut self, code:i32, action: Box<dyn FnMut()>) {
        let mut updators=self.updators.write().unwrap();
      let uline=UpdatorsLine{code, action};
        updators.push(Rc::new(Mutex::new(uline)));
    }

    pub fn callUpdators(&mut self)  {
        console_log!("from updators! {:?}", self);
        let mut rs=self.updators.write().unwrap();
         let  elements_iter = rs.iter_mut();

        //let mut rc=vec![];
        let mut counter=0;
        for val0 in elements_iter {
            console_log!("cc {:?}",counter);
            counter=counter+1;

            let mut el=val0.lock().unwrap();
            console_log!("cc2 {:?}",counter);
            console_log!("cc2 {:?}",(*el).code);

            // let {code,action}=el.code;
            let action=&mut el.action;
            action();
        }
       // (*rs).into_iter().all(|e|{ console_log!("e"); true}) ;
    } */
    /* pub fn callUpdators(&mut self) {
        let mut rs=self.updators.read().unwrap();
        let  elements_iter = rs.iter();

        //let mut rc=vec![];
        let mut counter=0;
        for val0 in elements_iter {
            // (*val0)();
            let  val = (val0) as & Box<dyn FnMut()>;
            val();
            //let line = val();
            // rc.push((line));
            counter = counter + 1;

        }
    } */



    pub fn setActiveElement(&mut self, value: u32) {
        self.activeElement = value;
        self.render();
    }

    pub fn setActiveColor(&mut self, value: Color) {
        self.chosenColor = value;
        self.render();
    }

    pub fn setMouseHovered (&mut self, status:HoverStatus) {
       // console_log!("set fov {:?}", status);
       self.hoveredStatus=status
    }
    pub fn setCoordinates(&mut self, kind: i8, value: i32) {
        match kind {
            0 => self.mouseXclicked = value,
            1 => self.mouseYclicked = value,
            _ => log("no"),
        }
        self.render();
    }

    pub fn setMousePressed(&mut self, value: bool) {
        self.mousePressed = value
    }
    pub fn setDragged(&mut self, value: bool) {
        self.dragged = value
    }
 pub fn setDraggedElement(&mut self, element: u32) {
        self.dragElement = Some(element)
    }
    pub fn clearDraggedElement(&mut self) {
        self.dragElement = None
    }

    pub fn getActiveColor(&mut self)->Color {
        self.chosenColor
    }
  pub fn getActiveElement(&mut self)->u32 {
        self.activeElement
    }


    fn render (&mut self) {
//        let mut str= vec!["".to_string()];
        // self.incCounter();
        let mut rs=self.rendered.write().unwrap();

       // let mut rs=self.rendered.read().unwrap();
       //  let mut state=rc.try_lock().unwrap();
         let mut elements_iter = rs.iter_mut();

        let mut rc=vec![];
        let mut counter=0;
        for val0 in elements_iter {
            let mut val = (val0) as &mut Box<dyn FnMut(Istate) -> std::string::String>;
            let line = val(self.clone());
            rc.push(line);
            counter = counter + 1;

        }

    }



}

impl fmt::Debug for Istate  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Istate")
            .field("activeElement", &self.activeElement)
           // .field("documentStorage", &self.documentStorage)
            .field("mousePressed", &self.mousePressed)
            .field("buttonPressed", &self.buttonPressed)
            .field("mouseXclicked", &self.mouseXclicked)
            .field("mouseYclicked", &self.mouseYclicked)
            .field("paletteClicked", &self.paletteClicked)
            .field("chosenColor", &self.chosenColor)
            .field("counter", &self.counter)
            .finish()
    }
}

