#![allow(dead_code, unused)]

use std::{any::Any, ops::{Deref, DerefMut}, sync::{Arc, Mutex}};

use crate::{block::{Block, TExecute}, terminal::{TTerminal, Terminal}};


pub struct BlockConstBool {
    block: Block,
    out_const: Arc<Mutex<dyn TTerminal>>
}

impl BlockConstBool {
    pub fn new(default_value: bool) -> Self {
        let mut block = Block::new("Constant Bool");
        let out_const:Arc<Mutex<dyn TTerminal>> = Arc::new(Mutex::new(Terminal::new("Out 1".to_string(), default_value)));
        
        block.add_out_terminal(Arc::clone(&out_const));
        block.changed = false;

        BlockConstBool {
            out_const,
            block
        }
    }

    pub fn get_out_value_by_index<T: 'static + Ord + Copy>(&mut self, out_index: usize) -> Result<&T,&str> {
        self.block.get_out_terminal_value_by_index(out_index)
    }
}

impl TExecute for BlockConstBool {
    fn execute(&mut self) -> &bool {
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
        Ok(())
    }

    fn connect_to_in_terminal(
        &mut self,
        in_index: usize,
        out_terminal: Arc<Mutex<dyn TTerminal>>,
    ) -> Result<(), &str> {
        self.block_connect_to_in_terminal::<bool>(in_index, out_terminal)
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

impl Deref for BlockConstBool {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for BlockConstBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block
    }
}

