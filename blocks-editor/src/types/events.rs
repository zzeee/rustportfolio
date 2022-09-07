use std::fmt;
use crossbeam_channel::Sender;
use crate::blocks::blocks::AllMasks;
use            serde::Serialize;

#[derive(Clone,  Copy)]
pub struct BlockEvent {
    pub(crate) code:EventTypes,
    pub(crate) subcode: u32
}

#[derive( Clone,Copy, PartialEq)]
pub enum EventTypes {
    RedrawCanvas, RefreshAll, Rescale, MoveCanvas, MouseDraw,MouseMove, OpenSettings,
    ActiveElementChanged,
    DeleteLayer, HideLayer, UpLayer, DownLayer, Nop
}


#[derive(Clone, Debug)]
pub struct CollisionZone {
    pub(crate) element_id:u32,
    pub(crate) element_x:i32,
    pub(crate) element_zone:AllMasks
}

#[derive(Clone, Debug, Serialize)]
pub struct OutputInput {
    pub(crate) output_element_id:u32,
    pub(crate) input_element_id:u32,
}

#[derive(Clone, Debug)]
pub enum MouseActions {
    NoAction,
    CollisionDetected,
    CollisionInputOutputDetected,
    EmptySpace,
    ElementMoved,
    ElementHovered,
}

#[derive(Clone, Debug)]
pub struct MouseResult {
    pub(crate) actionCode: MouseActions,
    pub(crate) payload: Option<Vec<OutputInput>>

}


#[derive( Clone)]
pub struct EventUpdator {
    pub(crate) event: Sender<BlockEvent>,
    pub(crate) sender_code:i32
}

pub type EventUpdators=Vec<EventUpdator>;

impl Default for BlockEvent {
    fn default() -> Self {
        BlockEvent { code: EventTypes::Nop, subcode: 0 }
    }

}

impl BlockEvent {
    fn newnew(code:EventTypes, subcode:u32) -> Self {
        BlockEvent { code, subcode }
    }
}
impl fmt::Debug for BlockEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Event")
              .field("code", &match &self.code {
                   EventTypes::RedrawCanvas=>&"RedrawCanvas",
                   EventTypes::RefreshAll=>&"RefreshAll",
                   EventTypes::Rescale=>&"Rescale",
                   EventTypes::MoveCanvas=>&"MoveCanvas",
                   EventTypes::MouseDraw=>&"MouseDraw",
                   EventTypes::MouseMove=>&"MouseMove",
                   EventTypes::ActiveElementChanged=>&"ActiveElementChanged",
                   EventTypes::DeleteLayer=>&"DeleteLayer",
                   EventTypes::HideLayer=>&"HideLayer",
                   EventTypes::DownLayer=>&"DownLayer",
                   EventTypes::UpLayer=>&"UpLayer",
                   EventTypes::OpenSettings=>&"OpenSettings",
                  _=>&"etc evt"
              })
            .field("subcode", &self.subcode)
            .finish()
    }
}
