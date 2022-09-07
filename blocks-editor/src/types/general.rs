use crossbeam_channel::{Receiver, Sender};
use crate::types::events::BlockEvent;

pub type ActiveElement=(Box<dyn FnMut()>, Sender<BlockEvent>, Receiver<BlockEvent>);
