use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use crossbeam_channel::{Sender,Receiver};
use crate::utils::main::{log};

use crate::console_log;
use crate::types::layersStorage::LayersActions;
use crate::types::events::{BlockEvent, EventTypes, EventUpdator, EventUpdators};

pub struct UpdatorsLine {
    code: UpdatorChannels,
    action: Box<dyn FnMut()>,
}


type UpdatorsLine2=HashMap<UpdatorChannels, Sender<BlockEvent>>;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum UpdatorChannels {
    CanvasDraw, CanvasPainter, SettingsPainter,
    PaletteMain,
    LayersDraw, LayersUpdator,
    InfoZone,
    EndCycle

}

pub struct UpdatorsStorage {
   // pub(crate) senders: EventUpdators,
    pub(crate) updators2: HashMap<UpdatorChannels, Box<dyn FnMut()>>, //  Rc<RwLock<Vec<Rc<Mutex<UpdatorsLine>>>>>,
    pub(crate) updators: Rc<RwLock<Vec<Rc<Mutex<UpdatorsLine>>>>>,
    pub(crate) senderSignals: HashMap<UpdatorChannels, Sender<BlockEvent>>,
    pub(crate) receiverSignals: HashMap<UpdatorChannels, Receiver<BlockEvent>>,
}

impl Default for UpdatorsStorage {
    fn default() -> Self {
        Self {
            updators: Rc::new(RwLock::new(vec![])),
            senderSignals: HashMap::default(),
            receiverSignals: HashMap::default(),
            updators2: HashMap::default(),
            // senders: vec![]
        }
    }
}

impl UpdatorsStorage {
    /*pub fn addEventUpdator2(&mut self, event: EventUpdator) {
        self.senders.push(event);
    }*/
    // pub fn addUpdatorAndSignal(&mut self, code: UpdatorChannels, action: Box<dyn FnMut()>, signal: Sender<BlockEvent>, receiverSignal: Receiver<BlockEvent> ) {
    pub fn addUpdatorAndSignal(&mut self, code: UpdatorChannels, action: Box<dyn FnMut()>, signal: Sender<BlockEvent>, receiverSignal: Receiver<BlockEvent> ) {
        self.senderSignals.insert(code, signal);
        self.receiverSignals.insert(code, receiverSignal);
        self.updators2.insert(code,action);
        /* let mut updators = self.updators.write().unwrap();
        let uline = UpdatorsLine { code, action };
        updators.push(Rc::new(Mutex::new(uline)));*/
    }

    pub fn addEventUpdator(&mut self, code: UpdatorChannels, action: Sender<BlockEvent>) {
        self.senderSignals.insert(code,action);
    }
    pub fn addUpdator(&mut self, code: UpdatorChannels, action: Box<dyn FnMut()>) {
        self.updators2.insert(code,action);
       /* let mut updators = self.updators.write().unwrap();
        // let mut updators2 = self.updators2.write().unwrap();
         let uline = UpdatorsLine { code, action };
         updators.push(Rc::new(Mutex::new(uline))); */
       //  updators2.push(UpdatorsLine2{code, action:Rc::new(RwLock::new(action))})
    }


    pub fn sendOnlySignalByEvent(&mut self, event: BlockEvent) {
        match event.code {

             EventTypes::UpLayer | EventTypes::DownLayer | EventTypes::DeleteLayer=> {
               self.sendOnlySignalByCode(UpdatorChannels::LayersUpdator,event);
               self.sendOnlySignalByCode(UpdatorChannels::LayersDraw,event);
               self.sendOnlySignalByCode(UpdatorChannels::CanvasPainter,event);
             },
            EventTypes::RedrawCanvas | EventTypes::RefreshAll | EventTypes::MoveCanvas=>{
                self.sendOnlySignalByCode(UpdatorChannels::CanvasDraw,event);
                self.sendOnlySignalByCode(UpdatorChannels::CanvasPainter,event);
                self.sendOnlySignalByCode(UpdatorChannels::LayersDraw,event)

            },
            EventTypes::OpenSettings=>{
            self.sendOnlySignalByCode(UpdatorChannels::SettingsPainter,event);
            self.sendOnlySignalByCode(UpdatorChannels::LayersDraw,event);
               self.sendOnlySignalByCode(UpdatorChannels::InfoZone,event);

            },
            EventTypes::ActiveElementChanged=>{
                self.sendOnlySignalByCode(UpdatorChannels::InfoZone,event);
                self.sendOnlySignalByCode(UpdatorChannels::PaletteMain,event);
                self.sendOnlySignalByCode(UpdatorChannels::LayersDraw,event);
                self.sendOnlySignalByCode(UpdatorChannels::CanvasDraw,event);
            },
            EventTypes::MouseMove => {
               self.sendOnlySignalByCode(UpdatorChannels::InfoZone,event);

            },
            EventTypes::MouseDraw => {
                self.sendOnlySignalByCode(UpdatorChannels::CanvasDraw,event);
                self.sendOnlySignalByCode(UpdatorChannels::CanvasPainter,event);
                self.sendOnlySignalByCode(UpdatorChannels::LayersDraw,event);
                self.sendOnlySignalByCode(UpdatorChannels::LayersUpdator,event);

            }
            _=>{}
         //   RedrawCanvas, RefreshAll, Rescale, MoveCanvas, MouseDraw,MouseMove,
   // ActiveElementChanged,
   // DeleteLayer, HideLayer, UpLayer, DownLayer, Nop
        }

    }
     fn sendSignalByCode(&mut self, updator: UpdatorChannels, event: BlockEvent) {
        self.sendOnlySignalByCode(updator,event);
        self.updateAll();
    }
     fn sendOnlySignalByCode(&mut self, updator: UpdatorChannels, event: BlockEvent) {
        // console_log!("sendOnlySignalByCode {:?}", event);
        let updatorF0=self.getSignalByCode(updator);
        if let Some(updatorF)=updatorF0 {
           let res=updatorF.clone().try_send(event);
            if let Err(res0)=res {
                console_log!("err ress {:?}", res0);
            }
            // console_log!("res: {:?}",res);
            //.expect(&*format!("Can not send  message "));
        }
    }

    pub fn getSignalByCode(&mut self, parameter: UpdatorChannels) -> Option<&Sender<BlockEvent>> {
        self.senderSignals.get(&parameter)
    }

     pub fn updateAllButOne(&mut self,codecode:UpdatorChannels)  {
          for (code,action) in self.updators2.iter_mut() {
              if code != &codecode {
                  if let Some(receiver) = self.receiverSignals.get(code) {
                      if receiver.len() > 0 {
                          action();
                      } // else { console_log!("skipped due to empty queue"); }
                  }
              }
          }
     }
     pub fn updateAll(&mut self)  {
         for (code,action) in self.updators2.iter_mut() {
             if let Some(receiver) = self.receiverSignals.get(code) {
                 if receiver.len() > 0 {
                     action();
                 }
             }
         }
     }
}

