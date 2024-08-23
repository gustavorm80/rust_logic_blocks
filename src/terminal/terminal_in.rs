#![allow(dead_code, unused)]

use std::{
    any::Any,
    default,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use super::terminal_out::{TTerminalOut, TerminalOut};

pub trait TTerminalIn {
    
}

#[derive(Clone)]
pub struct TerminalIn<T: Ord + Copy> {
    name: String,
    uuid: Uuid,
    connector: Option<Arc<Mutex<TerminalOut<T>>>>
}

impl<T: 'static + Ord + Copy> TerminalIn<T> {
    pub fn new(name: String) -> Self {
        TerminalIn {
            name,
            connector: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn new_arc(name: String) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(TerminalIn::new(name)))
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_connector(&mut self, terminal: Arc<Mutex<TTerminalOut<T>>>) {
        self.connector = Some(terminal);
    }
}