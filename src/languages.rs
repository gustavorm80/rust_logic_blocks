#![allow(dead_code, unused)]

use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use crate::learn::TLearn;

pub struct Languages {
    learning: Vec<Arc<Mutex<dyn TLearn>>>
}

impl Languages {
    pub fn new() -> Self {
        Languages {
            learning: vec![]
        }
    }

    pub fn add_language(&mut self, language: Arc<Mutex<dyn TLearn>>) {
        self.learning.push(language);
    }

    pub fn get_language_by_index(&mut self, index: usize) -> Arc<Mutex<dyn TLearn>> {
        Arc::clone(&self.learning[index])
    }
}