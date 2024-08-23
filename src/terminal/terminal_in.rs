#![allow(dead_code, unused)]

use std::{
    any::Any,
    default,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use super::terminal_out::{TTerminalOut, TerminalOut};

pub trait TTerminalIn: Send {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone)]
pub struct TerminalIn<T: Ord + Copy> {
    name: String,
    uuid: Uuid,
    connector: Option<Arc<Mutex<dyn TTerminalOut>>>,
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
        // &self.value
        panic!("Not implemented")
    }

    pub fn set_connector(&mut self, terminal: Arc<Mutex<dyn TTerminalOut>>) {
        self.connector = Some(terminal);
    }
}

impl<T: 'static + Ord + Copy + Send> TTerminalIn for TerminalIn<T> {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

}
