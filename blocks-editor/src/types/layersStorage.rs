use std::collections::HashMap;
// use std::cell::{Cell, RefCell};
use crate::types::state::{ Istate};

use crate::types::color::{Color};
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use crossbeam_channel::Sender;
use ndarray::Axis;
use serde::Serialize;
use serde_json::json;
use crate::console_log;
use crate::blocks::blocks::{AllMasks, BlockControl, HoverStatus};
use crate::utils::main::log;
use rand::Rng;
use crate::types::events::{BlockEvent, EventTypes, OutputInput};
use crate::types::updators::UpdatorsStorage;

#[derive(Clone, Serialize)]
pub struct CollisionsLine {
    pub(crate) id: u32,
     pub(crate) element_kind: u32,
       pub(crate) startx:i32,
       pub(crate) starty:i32,
}


impl fmt::Debug for CollisionsLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CollisionsLine:")
            .field("id:", &self.id)
            .field("kind_id:", &self.element_kind)
            .field("x", &self.startx)
            .field("y", &self.starty)

            .finish()
    }
}


#[derive(Clone, Serialize)]
pub struct UIElementObject {
    pub(crate) id: u32,
    pub(crate) element_kind: u32,
    pub(crate) color: Color,
    pub(crate) sorder: i32,
    pub(crate) startx:i32,
    pub(crate) starty:i32,
    pub(crate) width:i32,
    pub(crate) height:i32,
    pub(crate) active:bool,
    pub(crate) grouped:bool,
    pub(crate) hovered:bool,
    pub(crate) owner:i32,
    pub(crate) show_status:i32,
    pub(crate) data: Vec<i32>,
    pub (crate) parameters: Vec<(u32, String)>
  //  pub(crate) elementKinds: HashMap<u32,i32>,

}

impl UIElementObject {

    fn setHover(&mut self, hover: bool) {
        self.hovered = hover;
    }
    fn setShowStatus(&mut self, showStatus: i32) {
        self.show_status = showStatus;
    }
    fn setShowOrder(&mut self, sorder: i32) {
        self.sorder = sorder;
    }
    fn getShowStatus(&self)->i32{self.show_status}
    fn getShowOrder(&self)->i32{self.sorder}
    fn getHover(&self) -> bool {
        self.hovered
    }
}

impl Default for UIElementObject {
    fn default() -> Self {
        Self { id:0, color: Color::default(),parameters:vec![], grouped:false, active:false, show_status:0,sorder:0, hovered:false, element_kind: 0, data: vec![], startx:0,starty:0, width:0, height:0, owner:0 }
    }
}


impl fmt::Debug for UIElementObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UIElementObject:")
            .field("id:", &self.id)
            .field("type:", &self.element_kind)
            .field("data", &self.data)
            .field("x", &self.startx)
            .field("y", &self.starty)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("owner", &self.owner)
            .field("color", &self.color)
            .finish()
    }
}

#[derive(Clone, Serialize)]
pub enum SOrder {
    Up,
    Down
}
#[derive(Clone, Serialize)]
pub struct DocumentStorage {
    pub(crate) onehovered:bool,
    data: Vec<UIElementObject>,
    connections: HashMap<u32,u32>, // TO STORE connection
    connections_reversed: HashMap<u32,u32>, // TO STORE connection
    data2: HashMap<String, UIElementObject>,// future(!)
    elementKinds:HashMap<u32,u32>,
    layers:HashMap<u32,UIElementObject> // future(!)
}

pub type PaletteElements= Vec<Arc<Mutex<dyn BlockControl>>>;

impl Default for DocumentStorage {
    fn default() -> Self {
        Self {
            data: vec![],
            connections:HashMap::new(),
            connections_reversed:HashMap::new(),
            onehovered: false,
            data2: HashMap::new(),
            layers: HashMap::new(),
            elementKinds: HashMap::new()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum LayersActions {
    Hide, Unhide, Delete, Up, Down, Group, Ungroup
}

impl DocumentStorage {

    pub fn saveParameters(&mut self, element_id:u32, results:Vec<(u32, String)>) {
        console_log!("results {:?} {:?}", element_id, results);
         for element in self.data.iter_mut() {
             let mut newElement = element.clone();
             if element.id == element_id {
                 newElement.parameters=results.clone();
                 *element=newElement;
             }
         }
    }

    pub fn addInputOutputConnection(&mut self, connections: Vec<OutputInput>) {
        for  connection in connections.iter() {
            self.connections.insert(connection.input_element_id,connection.output_element_id);
            self.connections_reversed.insert(connection.output_element_id,connection.input_element_id);
        }
       // console_log!("current {:?} {:?}", self.connections, connections);


    }
    pub fn getKindByElementId(&self, element_id: u32) -> Option<&u32> {
        /*for (k, v) in
        self.elementKinds.iter() {
            console_log!("key={}, value={}", k, v);
        }*/
        console_log!("key={:?}",           self.elementKinds.get(&element_id));
        self.elementKinds.get(&element_id)
    }

    pub fn getLayerById(&self, element_id: u32) -> Option<UIElementObject> {
        for element in self.data.iter() {
            //
            if element.id == element_id {
                let mut newElement = element.clone();
                return Some(newElement)
            }
        }
        None
    }
    pub fn documentStorageAsStr(&self) -> String {
        json!(self.data).to_string()
    }
    pub fn getLayersCount(&self)->i32 {
        self.data.len() as i32
    }
    pub fn connectionsStorage(&mut self) -> (HashMap<u32, u32>,HashMap<u32, u32>) { (self.connections.clone(),self.connections_reversed.clone()) }
    pub fn documentStorageSorted(&mut self) -> Vec<UIElementObject> {
        self.data.sort_by(|a,b|a.sorder.cmp(&b.sorder));
         self.data.clone()
    }
    pub fn documentStorageAsIs(&self) -> Vec<UIElementObject> {
        self.data.clone()
    }

    pub fn addElementToStorage(&mut self, element: UIElementObject) {
        self.elementKinds.insert(element.id,element.element_kind);
        self.data.push(element)
    }

    pub fn changeElementOrder(&mut self, element_id: u32, operation:SOrder) {
        for element in self.data.iter_mut() {
             if element.id == element_id {
                  let mut newElement = element.clone();

                 console_log!("sorder of el {:?} {:?}",element_id, element.sorder );

                 newElement.sorder=match operation{
                     SOrder::Up=> element.sorder+1,
                     SOrder::Down=>element.sorder-1
                 };
              console_log!("sorder of elafter {:?} {:?}",element_id, element.sorder );
 *element = newElement;
                 // newElement.setShowStatus(-1);
             } else {
                //  newElement.setShowStatus(0);
             }

         }
        self.data.sort_by(|a,b|a.sorder.cmp(&b.sorder));

    }

    pub fn deleteElementFromStorage(&mut self, element_id: u32) {
        console_log!("need to delete element {:?}", element_id);
        // to add: clear hasmap
        for element in self.data.iter_mut() {
             let mut newElement = element.clone();
             if element.id == element_id {
                 newElement.setShowStatus(-1);
             } else {
                 newElement.setShowStatus(0);
             }
             *element = newElement;
         }
            self.data.retain(|el|el.show_status==0);
    }

    pub fn setHoveredStatusToNothing(&mut self) {
        for element in self.data.iter_mut() {
            let mut newElement = element.clone();
            newElement.setHover(false);
            *element = newElement;
        }
        self.onehovered=false;

    }

    pub fn parseIsInside (&mut self, element_id:u32, x: i32, y: i32, elementsPalette: Rc<Mutex<PaletteElements>>, us: Rc<RwLock<UpdatorsStorage>>) -> HoverStatus {
        let mut resultresult=false;
        for element in self.data.iter() {
            // let mut newElement = element.clone();
            if element.id == element_id {
                let absoluteX=x-element.startx-1;
                let absoluteY=y-element.starty-1;
                let kind=element.element_kind;
                let cElement=elementsPalette.lock().unwrap();
                for fElement in cElement.iter() {
                    let aElement = fElement.lock().unwrap();
                    if aElement.GetKind() == kind {
                        if let Some(result) = aElement.getHoverInfo(absoluteX as usize, absoluteY as usize, element_id) {
                            let subcode = result.6 as i32 + result.5 as i32 + result.4 as i32 * 2 + result.3 as i32 * 4 + result.2 as i32 * 8 + result.1 as i32 * 16;
                            resultresult = true;
                            return result;
                            // console_log!("parseIsInside resultresult {:?}",result);
                        }
                    }
                }
            } else {
                // newElement.setHover(false);
            }

        }
        (0,0,0,0,0,0,0)
    }

    pub fn setHoveredStatusToOne (&mut self, element_id:u32, status:bool) {
         for element in self.data.iter_mut() {
             let mut newElement = element.clone();
             if element.id == element_id {
                 newElement.setHover(status);
                 self.onehovered=true;
             } else {
                 newElement.setHover(false);
             }
             *element = newElement;
             // console_log!("setHover {:?} {:?} {:?}",element.id == element_id, element.id, element.getHover());
         }
    }
    pub fn detectCollision(&mut self, element_id: u32,x: i32, y: i32)->Option<(Vec<CollisionsLine>,i32,i32,i32,i32)> {

        if let Some(celement)=self.data.iter().find(|e|e.id==element_id) {
            // console_log!("collision detecote for {:?} {:?} {:?}", element_id, x,y);
            let mut collisions=vec![];
            let mut collisions2:Vec<CollisionsLine>=vec![];
            let mut minx:i32=celement.startx;
            let mut miny:i32=celement.starty;
            let mut maxx:i32=celement.startx+celement.width;
            let mut maxy:i32=celement.starty+celement.height;

            for element in self.data.iter() {
                if element.id != element_id {
                    // console_log!("check {:?} {:?} {:?} {:?} {:?} {:?}",element.startx>celement.startx && element.startx<celement.startx+celement.width ,element.startx<celement.startx && element.startx+element.width> celement.startx,  element.startx, element.startx+element.width, celement.startx,   celement.startx+celement.width );
                    if (element.startx > celement.startx && element.startx < celement.startx + celement.width ||
                        element.startx < celement.startx && element.startx + element.width > celement.startx)
                        &&
                        (element.starty>celement.starty && element.starty<celement.starty+celement.height
                            || element.starty<celement.starty && element.starty+element.height>celement.starty) {
                        //                            console_log!("x+y -collision");
                        if minx==0 || element.startx<minx { minx=element.startx; }
                        if miny==0 || element.starty<miny { miny=element.starty; }
                        if element.startx+element.width>maxx { maxx=element.startx+element.width; }
                        if element.starty+element.height>maxy { maxy=element.starty+element.height; }

                        collisions.push(element.id);
                        collisions2.push(CollisionsLine{id:element.id, startx:element.startx, starty:element.starty, element_kind:element.element_kind})
                    }
                    //console_log!("el {:?} {:?} {:?} {:?}  {:?} {:?} ", x, element.startx, element.width,  celement.startx,  celement.width, element.height);
                }
            }
            if collisions.len() > 0 { collisions.push(celement.id);}
            if collisions2.len() > 0 { collisions2.push(CollisionsLine{id:celement.id, startx:celement.startx, starty:celement.starty, element_kind:celement.element_kind});}
            // console_log!("collisions arr {:?} {:?}", collisions,(minx,miny, maxx,maxy));
            return Some((collisions2,minx,miny, maxx,maxy))
        }
        None
    }
    pub fn moveElement(&mut self, element_id: u32, moveX: i32, moveY: i32) {
        for element in self.data.iter_mut() {
             if element.id == element_id {
                 let mut newElement = element.clone();
                 newElement.startx = element.startx + moveX;
                 newElement.starty = element.starty + moveY;
                 *element = newElement;
             }
        }
    }

    pub fn addCurrentElementToStorage(&mut self, element_kind: u32, startx: i32, starty: i32, width: i32, height: i32, color: Color, data: Vec<i32>, sorder: i32) {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen();
        let newElement = UIElementObject { id, element_kind,parameters:vec![], data, active: false, grouped: false, show_status: 0, hovered: false, sorder, startx, starty, width, height, color, owner: 0 };
        // if self.data.len()<1 { // todo delete
        self.elementKinds.insert(newElement.id, newElement.element_kind);

        self.data.push(newElement);
        // console_log!("newElement {:?}", self)
        // }
    }

}



#[derive( Clone)]
pub struct UIElement {
    pub(crate)  id: i32,
    pub(crate)  kind: i32,
    pub(crate)  text: String,
    pub(crate)  renderer: fn(data:&Istate)->String,
}



impl fmt::Debug for UIElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UIElement:")
            .field("id", &self.id)
            .finish()
    }
}

impl fmt::Debug for DocumentStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DocumentStorage:")
            .field("id", &self.data)
            .finish()
    }
}

impl Default for UIElement {
    fn default() -> Self {
        Self {id: 0, kind:0, text:"".to_string(),renderer:|data:&Istate|->String {return "".to_string();} }
    }
}

