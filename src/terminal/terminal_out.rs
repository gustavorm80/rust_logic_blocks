#![allow(dead_code, unused)]

use std::{
    any::Any,
    default,
    sync::{Arc, Mutex},
};

use core::fmt::Debug;

use uuid::Uuid;


pub trait TTerminalOut: Send {
    fn reset(&mut self);
    fn is_new_value(&self) -> bool;
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type TTerminalOutRef = Arc<Mutex<dyn TTerminalOut>>;
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

    pub fn get_value(&self) -> T {
        self.value
    }

    pub fn get_value_tterminal(terminal_out: &TTerminalOutRef, default: T) -> T {
        let tlock = terminal_out.lock().unwrap();
        let tdown = tlock.as_any().downcast_ref::<TerminalOut<T>>();
        match tdown {
            Some(terminal) => {
                terminal.get_value()
            },
            None => default,
        }
    }

    pub fn set_value_tterminal_if_diff(terminal_out: & TTerminalOutRef, new_value: T) -> bool {
        let mut tlock = (*terminal_out).lock().unwrap();
        let mut tdown = tlock.as_any_mut().downcast_mut::<TerminalOut<T>>();
        match tdown {
            Some(terminal) => {
                let current = terminal.get_value();
                if current != new_value {
                    terminal.set_value(new_value);
                    true
                }
                else {
                    false
                }
            },
            None => false
        }
    }

    pub fn set_value(&mut self, value: T) {
        if (self.value != value) {
            self.value = value;
            self.is_new_value = true;
        }
    }

    pub fn is_new_value(&self) -> bool {
        self.is_new_value
    }

    pub fn reset(&mut self) {
        self.is_new_value = false;
        self.last_value = self.value;
    }

    // fn into_bool<V: Into<bool>>(input: V) -> bool {
    //     input.into()
    // }

    // fn into_bool_mut<V: Into<bool>>(input: mut V) -> bool {
    //     input.into()
    // }
}


impl<T: 'static + Ord + Copy + Send> TTerminalOut for TerminalOut<T> {
    fn reset(&mut self) {
        self.reset();
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }


    fn is_new_value(&self) -> bool {
        self.is_new_value
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl Debug for dyn TTerminalOut {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}