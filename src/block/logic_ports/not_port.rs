#![allow(dead_code, unused)]

use std::{
    any::Any,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::{
    block::{Block, TExecute},
    terminal::{TTerminal, Terminal},
};

pub struct NotPort {
    block: Block,
    in_a: Arc<Mutex<dyn TTerminal>>,
    out_not: Arc<Mutex<dyn TTerminal>>,
}

impl NotPort {
    pub fn new() -> Self {
        let mut block = Block::new("Not Port");
        let out_not: Arc<Mutex<dyn TTerminal>> =
            Arc::new(Mutex::new(Terminal::new("Out 1".to_string(), false)));
        let in_a: Arc<Mutex<dyn TTerminal>> =
            Arc::new(Mutex::new(Terminal::new("In 1".to_string(), false)));

        block.add_out_terminal(Arc::clone(&out_not));
        block.add_in_terminal(Arc::clone(&in_a));
        block.changed = false;

        NotPort {
            in_a,
            out_not,
            block,
        }
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

impl TExecute for NotPort {
    fn execute(&mut self) -> &bool {
        let mut result = false;

        let mut term = (*self.in_a).lock().unwrap();
        let mut downcast = term.as_any_mut().downcast_mut::<Terminal<bool>>();

        result = match downcast {
            Some(x) => {
                x.read_connector();
                !(*x.get_value())
            }
            None => false,
        };

        let mut term = (*self.out_not).lock().unwrap();
        let downcast = term.as_any_mut().downcast_mut::<Terminal<bool>>().unwrap();

        let out_val = downcast.get_value();

        if (result != *out_val) {
            downcast.set_value(result);
            self.block.changed = true;
        }

        &self.block.changed
    }

    fn is_changed(&self) -> &bool {
        &self.block.changed
    }

    fn reset(&mut self) {
        self.block.changed = false;
    }

    fn connect_to_in_terminal_block<'a>(
        &'a mut self,
        in_index: usize,
        from_block: &'a Block,
        from_out_index: usize,
    ) -> Result<(), &str> {
        self.block_connect_to_in_terminal_block::<bool>(in_index, from_block, from_out_index)
    }

    fn connect_to_in_terminal(
        &mut self,
        in_index: usize,
        out_terminal: Arc<Mutex<dyn TTerminal>>,
    ) -> Result<(), &str> {
        Ok(())
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

impl Deref for NotPort {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for NotPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block
    }
}
