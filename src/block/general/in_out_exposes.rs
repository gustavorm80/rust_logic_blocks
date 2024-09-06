#![allow(dead_code, unused)]

use std::{
    any::Any,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::{
    block::{Block, Executer, TExecute},
    terminal::{
        terminal_in::{TTerminalIn, TerminalIn},
        terminal_out::{TTerminalOut, TerminalOut},
    },
};

pub enum EExposeType {
    InTerminal = 0,
    OutTerminal,
}

pub struct InOutExposes {
    block: Block,
    expose_type: EExposeType,
}

impl InOutExposes {
    pub fn new(expose_type: EExposeType) -> Self {
        let mut block = Block::new("In Exposes Block");

        let out_a: Arc<Mutex<dyn TTerminalOut>> =
            Arc::new(Mutex::new(TerminalOut::new("Out 1".to_string(), false)));

        let in_a: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 1".to_string());

        block.add_in_terminal(in_a);
        block.add_out_terminal(out_a);
        block.changed = false;

        InOutExposes { block, expose_type }
    }

    pub fn get_type(&self) -> &EExposeType {
        &self.expose_type
    }
}

impl TExecute for InOutExposes {
    fn execute(&mut self) -> bool {
        let mut result = false;

        if let Ok(terminal) = self.get_in_terminal_by_index(0) {
            result = TerminalIn::get_value_tterminal_in::<bool>(&terminal, false);
        }

        if let Ok(terminal) = self.get_out_terminal_by_index(0) {
            if TerminalOut::<bool>::set_value_tterminal_if_diff(&terminal, result) {
                self.deref_mut().set_changed(true);
            }
        }

        self.block.changed
    }

    fn is_changed(&self) -> bool {
        self.block.changed
    }

    fn get_name(&self) -> &str {
        self.deref().get_name()
    }

    fn reset(&mut self) {
        self.block.reset();
    }

    fn new_pass(&mut self) {
        self.block.new_pass();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_block(&self) -> &dyn Any {
        &self.block
    }
}

impl Deref for InOutExposes {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for InOutExposes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block
    }
}
