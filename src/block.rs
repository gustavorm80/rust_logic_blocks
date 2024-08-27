use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::terminal::{
    terminal_in::{TTerminalIn, TerminalIn},
    terminal_out::{TTerminalOut, TerminalOut},
};

pub mod constants;
pub mod logic_ports;

pub trait TExecute: Send {
    fn execute(&mut self) -> bool;
    fn is_changed(&self) -> &bool;

    fn get_block(&self) -> &Block;

    fn get_block_mut(&mut self) -> &mut Block;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait TBlock {
    fn connect_out_to_in<'a>(
        &mut self,
        out_index: usize,
        to_block: &'a Block,
        to_in_index: usize,
    ) -> Result<(), &str>;
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

    pub fn get_in_terminals_size(&self) -> usize {
        self.in_terminals.len()
    }

    pub fn get_out_terminals_size(&self) -> usize {
        self.out_terminals.len()
    }

    pub fn get_out_terminal_value_by_index<T: 'static + Ord + Copy>(
        &self,
        out_index: usize,
    ) -> Result<T, &str> {
        if out_index >= self.out_terminals.len() {
            Err("Index is out of bound")
        } else {
            let terminal = (*self.out_terminals[out_index]).lock().unwrap();
            let downcast = terminal.as_any().downcast_ref::<TerminalOut<T>>();
            match downcast {
                Some(x) => {
                    let val = x.get_value();
                    Ok(val)
                }
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

    pub fn get_changed(&self) -> bool {
        self.changed
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn connect_to_in_terminal<'a, T: Ord + Copy + 'static>(
        &mut self,
        in_index: usize,
        out_terminal: Arc<Mutex<dyn TTerminalOut>>,
    ) -> Result<(), &str> {
        if in_index >= self.in_terminals.len() {
            Err("Index is out of bound")
        } else {
            let mut in_terminal = (*self.in_terminals[in_index]).lock().unwrap();
            let downcasted = in_terminal.as_any_mut().downcast_mut::<TerminalIn>();

            match downcasted {
                Some(term) => {
                    term.set_connector(out_terminal);
                    Ok(())
                }
                None => Err("Terminal type is not correct"),
            }
        }
    }

    pub fn connect_out_to_in<'a, T: Ord + Copy + 'static>(
        &'a mut self,
        out_index: usize,
        to_block: &'a mut Block,
        to_in_index: usize,
    ) -> Result<(), &str> {
        let out_terminal = self.get_out_terminal_by_index(out_index);

        match out_terminal {
            Ok(term) => {
                match to_block.connect_to_in_terminal::<T>(to_in_index, Arc::clone(&term)) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn new_pass(&mut self) {
        self.set_changed(false);
    }

    pub fn reset(&mut self) {
        for tout in self.out_terminals.iter_mut() {
            let mut tlock = tout.lock().unwrap();
            tlock.reset();
        }
    }

}
