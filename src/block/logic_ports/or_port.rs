#![allow(dead_code, unused)]

use std::{
    any::Any,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::{
    block::{Block, TExecute},
    terminal::{
        terminal_in::{TTerminalIn, TerminalIn},
        terminal_out::{TTerminalOut, TerminalOut},
    },
};

pub struct OrPort {
    block: Block,
    out_or: Arc<Mutex<dyn TTerminalOut>>,
}

impl OrPort {
    pub fn new() -> Self {
        let mut block = Block::new("Or Port");
        let out_or: Arc<Mutex<dyn TTerminalOut>> =
            Arc::new(Mutex::new(TerminalOut::new("Out 1".to_string(), false)));

        let in_a: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 1".to_string());
        let in_b: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 2".to_string());

        block.add_out_terminal(Arc::clone(&out_or));
        block.add_in_terminal(in_a);
        block.add_in_terminal(in_b);
        block.changed = false;

        OrPort { block, out_or }
    }

    pub fn get_name(&self) -> &str {
        self.block.get_name()
    }

    pub fn get_out_value_by_index<T: 'static + Ord + Copy>(
        &mut self,
        out_index: usize,
    ) -> Result<&T, &str> {
        self.block.get_out_terminal_value_by_index(out_index)
    }
}

impl TExecute for OrPort {
    fn execute(&mut self) -> bool {
        let mut result = false;

        for in_terminal in self.block.in_terminals.iter() {
            result |= TerminalIn::get_value_tterminal_in::<bool>(&in_terminal, false);
        }

        if let Ok(terminal) = self.get_out_terminal_by_index(0) {
            if TerminalOut::<bool>::set_value_tterminal_if_diff(&terminal, result) {
                self.deref_mut().set_changed(true);
            }
        }

        self.block.changed
    }

    fn is_changed(&self) -> &bool {
        &self.block.changed
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Deref for OrPort {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for OrPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block
    }
}
