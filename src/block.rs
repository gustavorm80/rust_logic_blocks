
use std::{any::Any, sync::{Arc, Mutex}};

use crate::terminal::{terminal_in::{TTerminalIn, TerminalIn}, terminal_out::TTerminalOut};

pub mod constants;
pub mod logic_ports;


pub trait TExecute: Send {
    fn execute(&mut self) -> &bool;
    fn is_changed(&self) -> &bool;
    fn reset(&mut self);

    fn connect_to_in_terminal(
        &mut self,
        in_index: usize,
        out_terminal: Arc<Mutex<dyn TTerminalOut>>,
    ) -> Result<(), &str>;

    fn connect_to_in_terminal_block<'a>(
        &'a mut self,
        in_index: usize,
        from_block: &'a Block,
        from_out_index: usize,
    ) -> Result<(), &str>;

    fn get_block(&self) -> &Block;

    fn get_block_mut(&mut self) -> &mut Block;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Block {
    name: String,
    changed: bool,
    in_terminals: Vec<Arc<Mutex<dyn TTerminalIn>>>,
    out_terminals: Vec<Arc<Mutex<dyn TTerminalOut>>>,
}

impl Block {
    pub fn new(name: &str) -> Self {
        Block {
            name: name.to_string(),
            changed: false,
            in_terminals: vec![],
            out_terminals: vec![],
        }
    }

    pub fn add_in_terminal(&mut self, terminal: Arc<Mutex<dyn TTerminalIn>>) {
        self.in_terminals.push(terminal);
    }

    pub fn add_out_terminal(&mut self, terminal: Arc<Mutex<dyn TTerminalOut>>) {
        self.out_terminals.push(terminal);
    }

    pub fn get_out_terminal_by_index(
        &self,
        out_index: usize,
    ) -> Result<Arc<Mutex<dyn TTerminalOut>>, &str> {
        if out_index >= self.out_terminals.len() {
            Err("Index is out of bound")
        } else {
            Ok(Arc::clone(&self.out_terminals[out_index]))
        }
    }

    pub fn get_out_terminal_value_by_index<T: 'static + Ord + Copy>(
        &self,
        out_index: usize,
    ) -> Result<T, &str> {
        if out_index >= self.out_terminals.len() {
            Err("Index is out of bound")
        } else {
            let terminal = (*self.out_terminals[out_index]).lock().unwrap();
            let downcast = terminal.as_any().downcast_ref::<TTerminalOut<T>>();
            match downcast {
                Some(x) => {
                    let val = *x.get_value();
                    Ok(val)
                },
                None => Err("Terminal type is not correct"),
            }
        }
    }

    pub fn get_in_terminal_by_index(
        &self,
        in_index: usize,
    ) -> Result<Arc<Mutex<dyn TTerminalIn>>, &str> {
        if in_index >= self.in_terminals.len() {
            Err("Index is out of bound")
        } else {
            Ok(Arc::clone(&self.in_terminals[in_index]))
        }
    }

    pub fn set_changed(&mut self, changed: bool) {
        self.changed = changed;
    }

    pub fn get_changed(&self) -> &bool {
        &self.changed
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn block_connect_to_in_terminal<'a, T: Ord + Copy + 'static>(
        &mut self,
        in_index: usize,
        out_terminal: Arc<Mutex<dyn TTerminalOut>>,
    ) -> Result<(), &str> {
        if in_index >= self.in_terminals.len() {
            Err("Index is out of bound")
        } else {
            let mut in_terminal = (*self.in_terminals[in_index]).lock().unwrap();

            let downcasted = in_terminal.as_any_mut().downcast_mut::<TerminalIn<T>>();

            match downcasted {
                Some(term) => {
                    term.set_connector(out_terminal);
                    Ok(())
                }
                None => Err("Terminal type is not correct"),
            }
        }
    }

    fn block_connect_to_in_terminal_block<'a, T: Ord + Copy + 'static>(
        &'a mut self,
        in_index: usize,
        from_block: &'a Block,
        from_out_index: usize,
    ) -> Result<(), &str> {
        let terminal = from_block.get_out_terminal_by_index(from_out_index);

        match terminal {
            Ok(term) => match self.block_connect_to_in_terminal::<T>(in_index, Arc::clone(&term)) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

