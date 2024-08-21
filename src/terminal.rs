#![allow(dead_code, unused)]

use std::{any::Any, default, sync::{Arc, Mutex}};

use uuid::Uuid;

pub trait TTerminal: Send {
    fn reset(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub enum TerminalType {
    Boolean
}

#[derive(Clone)]
pub struct Terminal<T: Ord + Copy> {
    name: String,
    uuid: Uuid,
    connector: Option<Arc<Mutex<dyn TTerminal>>>,
    value: T,
    default_value: T,
    is_new_value: bool,
    is_persistent: bool,
    
}

impl<T: 'static + Ord + Copy> Terminal<T> {
    pub fn new(name: String, default_value: T) -> Self {
        Terminal {
            name,
            connector: None,
            uuid: Uuid::new_v4(),
            value: default_value,
            default_value,
            is_new_value: false,
            is_persistent: false,
        }
    }

    pub fn new_persistent(name: String, default_value: T) -> Self {
        Terminal {
            name,
            connector: None,
            uuid: Uuid::new_v4(),
            value: default_value,
            default_value,
            is_new_value: false,
            is_persistent: true,
        }
    }

    pub fn new_arc(name: String, default_value: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Terminal::new(name, default_value)))
    }

    pub fn new_persistent_arc(name: String, default_value: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Terminal::new_persistent(name, default_value)))
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }   

    pub fn set_connector(&mut self, terminal: Arc<Mutex<dyn TTerminal>>) {
        self.connector = Some(terminal);
    }

    pub fn reset(&mut self) {
        if !self.is_persistent {
            self.is_new_value = false;
            self.value = self.default_value;
        }
    }

    pub fn read_connector(&mut self) {
        match &self.connector {
            Some(x) => {
                let con = (*x).lock().unwrap();
                let downcast = con.as_any().downcast_ref::<Terminal<T>>();
                match downcast {
                    Some(x) => {
                        let val = *x.get_value();
                        self.value = val;
                    } ,
                    None => (),
                }
            }
            None => (),
        };
    }
}

impl<T: 'static + Ord + Copy + Send> TTerminal for Terminal<T> {
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
