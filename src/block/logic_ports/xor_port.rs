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

pub struct XOrPort {
    block: Block,
    out_xor: Arc<Mutex<dyn TTerminalOut>>,
}

impl XOrPort {
    pub fn new() -> Self {
        let mut block = Block::new("Or Port");
        let out_xor: Arc<Mutex<dyn TTerminalOut>> =
            Arc::new(Mutex::new(TerminalOut::new("Out 1".to_string(), false)));

        let in_a: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 1".to_string());
        let in_b: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 2".to_string());

        block.add_out_terminal(Arc::clone(&out_xor));
        block.add_in_terminal(in_a);
        block.add_in_terminal(in_b);
        block.changed = false;

        XOrPort { block, out_xor }
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

impl TExecute for XOrPort {
    fn execute(&mut self) -> bool {
        let mut result = false;
        let mut first = true;

        for in_terminal in self.block.in_terminals.iter() {
            let mut term = (*in_terminal).lock().unwrap();

            let mut downcast = term.as_any_mut().downcast_mut::<TerminalIn>();

            if first {
                result = match downcast {
                    Some(x) => match (*x).get_value::<bool>() {
                        Some(val) => val,
                        None => false,
                    },
                    None => false,
                };
                first = false;
            } else {
                result ^= match downcast {
                    Some(x) => match (*x).get_value::<bool>() {
                        Some(val) => val,
                        None => false,
                    },
                    None => false,
                };
            }


        }

        let mut term = (*self.out_xor).lock().unwrap();
        let downcast = term
            .as_any_mut()
            .downcast_mut::<TerminalOut<bool>>()
            .unwrap();

        let out_val = downcast.get_value();

        if (result != out_val) {
            downcast.set_value(result);
            self.block.set_changed(true);
        }

        self.block.changed
    }

    fn is_changed(&self) -> &bool {
        &self.block.changed
    }

    fn get_block(&self) -> &Block {
        &self.block
    }

    fn get_block_mut(&mut self) -> &mut Block {
        &mut self.block
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Deref for XOrPort {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for XOrPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block
    }
}
