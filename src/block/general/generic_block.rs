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

use super::in_out_exposes::{self, InOutExposes};

#[derive(Debug)]
pub struct GenericBlock {
    block: Block,
    blocks: Executer,
}

impl GenericBlock {
    pub fn new(blocks: Executer) -> Self {
        let mut block = Block::new("Generic Block");

        block.changed = false;

        let blocks_copy = Arc::clone(&blocks);

        let mut generic_block = GenericBlock { block, blocks };

        GenericBlock::create_exposed_terminals(&mut generic_block, &blocks_copy);

        generic_block
    }

    fn create_exposed_terminals(&mut self, blocks: &Executer) {
        let mut locked_blocks = blocks.lock().unwrap();

        for block in locked_blocks.iter_mut() {
            let casted = block.as_any_mut().downcast_mut::<InOutExposes>();
            if let Some(in_out_block) = casted {
                match in_out_block.get_type() {
                    in_out_exposes::EExposeType::InTerminal => {
                        if let Ok(terminal) = in_out_block.get_in_terminal_by_index(0) {
                            self.add_in_terminal(terminal);
                        }
                    }
                    in_out_exposes::EExposeType::OutTerminal => {
                        if let Ok(terminal) = in_out_block.get_out_terminal_by_index(0) {
                            self.add_out_terminal(terminal);
                        }
                    }
                }
            }
        }
    }

    pub fn get_out_value_by_index<T: 'static + Ord + Copy>(
        &mut self,
        out_index: usize,
    ) -> Result<&T, &str> {
        self.block.get_out_terminal_value_by_index(out_index)
    }

    fn execute_blocks(&mut self) -> bool {
        let mut blocks_lock = self.blocks.lock().unwrap();
        let mut executed = false;

        for block in blocks_lock.iter_mut() {
            executed |= block.execute();
        }

        executed
    }

    pub fn get_blocks(&self) -> Executer {
        Arc::clone(&self.blocks)
    }
}

impl TExecute for GenericBlock {
    fn execute(&mut self) -> bool {
        let changed = self.execute_blocks();

        if (changed) {
            self.set_changed(changed);
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

        let mut blocks = self.blocks.lock().unwrap();
        for block in blocks.iter_mut() {
            block.reset();
        }
    }

    fn new_pass(&mut self) {
        self.block.new_pass();

        let mut blocks = self.blocks.lock().unwrap();
        for block in blocks.iter_mut() {
            block.new_pass();
        }
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

impl Deref for GenericBlock {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for GenericBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block
    }
}
