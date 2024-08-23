#![allow(dead_code, unused)]

use std::{
    any::Any,
    default,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

pub trait TTerminalOut: Send {
    fn reset(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone)]
pub struct TerminalOut<T: Ord + Copy> {
    name: String,
    uuid: Uuid,
    value: T,
    last_value: T,
    is_new_value: bool,
}

impl<T: 'static + Ord + Copy> TerminalOut<T> {
    pub fn new(name: String, start_value: T) -> Self {
        TerminalOut {
            name,
            uuid: Uuid::new_v4(),
            value: start_value,
            last_value: start_value,
            is_new_value: false,
        }
    }

    pub fn new_arc(name: String, default_value: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(TerminalOut::new(name, default_value)))
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: T) {
        if (self.value != value) {
            self.value = value;
            self.is_new_value = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_new_value = false;
        self.last_value = self.value;
    }
}

impl<T: 'static + Ord + Copy + Send> TTerminalOut for TerminalOut<T> {
    fn reset(&mut self) {
        self.reset();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
