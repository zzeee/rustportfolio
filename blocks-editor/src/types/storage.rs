use std::fmt;
use crate::utils::main::{log};
use std::cell::RefCell;
use std::string::String;
use std::collections::HashMap;
use web_sys::WebSocket;
use crate::types::layersStorage::{UIElementObject};
use crate::types::color::{Color};
use crate::types::layersStorage::{  UIElement};
use serde_json::json;
use rand::{ Rng};


pub struct ElementsPalette {
    data: Vec<UIElement>,
}

impl Default for ElementsPalette {
    fn default() -> Self {
        Self {
            data:  vec![UIElement::default()]
        }
    }
}

impl ElementsPalette  {
    pub fn getElements(&mut self)->Vec<UIElement> { self.data.clone() }
pub fn addElement(&mut self, newElement: UIElement) {
        self.data.push(newElement)
    }

}

impl fmt::Debug for ElementsPalette  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Istate")
            .field("data", &self.data)
            .finish()
    }
}

