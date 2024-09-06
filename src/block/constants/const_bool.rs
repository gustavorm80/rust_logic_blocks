#![allow(dead_code, unused)]

use std::{any::Any, default, ops::{Deref, DerefMut}, sync::{Arc, Mutex}};

use crate::{block::{Block, TExecute}, terminal::terminal_out::{self, TTerminalOut, TerminalOut}, };


pub struct BlockConstBool {
    block: Block,
    out_const: Arc<Mutex<dyn TTerminalOut>>
}

impl BlockConstBool {
    pub fn new(start_value: bool) -> Self {
        let mut block = Block::new("Constant Bool");
        let terminal = TerminalOut::new("Out 1".to_string(), start_value);

        let out_const:Arc<Mutex<dyn TTerminalOut>> = Arc::new(Mutex::new(terminal));
        
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

    pub fn toogle_output(&mut self) {
        // let value = self.get_out_value_by_index::<bool>(0).unwrap();
        if let Ok(terminal) = self.get_out_terminal_by_index(0) {
            let mut lt = terminal.lock().unwrap();
            let mut dc = lt.as_any_mut().downcast_mut::<TerminalOut<bool>>();
            if let Some(t) = dc {
                t.set_value(!t.get_value());
            }

        }

    }
}

impl TExecute for BlockConstBool {
    fn execute(&mut self) -> bool {
        self.block.changed
    }

    fn is_changed(&self) -> bool {
        self.block.changed
    }

    fn get_name(&self) -> &str {
        self.deref().get_name()
    }

    fn reset(&mut self) {
        // self.block.reset();
    }

    fn new_pass(&mut self){
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

