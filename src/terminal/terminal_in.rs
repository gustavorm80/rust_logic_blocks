#![allow(dead_code, unused)]

use std::{
    any::Any,
    default,
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
};

use core::fmt::Debug;

use uuid::Uuid;

use super::terminal_out::{TTerminalOut, TerminalOut};

pub trait TTerminalIn: Send {
    fn get_connector_mut(&mut self) -> &Option<Arc<Mutex<dyn TTerminalOut>>>;
    fn get_name(&self) -> &str;
    fn set_name(&mut self,name: String);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}


type TerminalInRef = Arc<Mutex<TerminalIn>>;
type TTerminalInRef = Arc<Mutex<dyn TTerminalIn>>;


#[derive(Clone)]
pub struct TerminalIn {
    name: String,
    uuid: Uuid,
    connector: Option<Arc<Mutex<dyn TTerminalOut>>>,
}

impl TerminalIn {
    pub fn new(name: String) -> TerminalInRef {
        Arc::new(Mutex::new(TerminalIn {
            name,
            connector: None,
            uuid: Uuid::new_v4(),
        }))
    }

    pub fn get_value<T: 'static + Ord + Copy>(&self) -> Option<T> {
        match &self.connector {
            Some(con) => {
                let terminal = con.lock().unwrap();
                let downcast = terminal.as_any().downcast_ref::<TerminalOut<T>>();

                match downcast {
                    Some(x) => Some(x.get_value()),
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn get_value_tterminal_in<T: 'static + Ord + Copy>(terminal_in: &TTerminalInRef, default: T) -> T {
        let tlock = terminal_in.lock().unwrap();
        let tdown = tlock.as_any().downcast_ref::<TerminalIn>();
        match tdown {
            Some(terminal) => {
                match terminal.get_value() {
                    Some(value) => value,
                    None => default,
                }
            },
            None => default,
        }
    }

    pub fn set_connector(&mut self, terminal: Arc<Mutex<dyn TTerminalOut>>) {
        self.connector = Some(terminal);
    }
}

impl TTerminalIn for TerminalIn {
    fn get_connector_mut(&mut self) -> &Option<Arc<Mutex<dyn TTerminalOut>>> {
        &self.connector
    }

    fn set_name(&mut self, name: String){
        self.name = name;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for dyn TTerminalIn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}


