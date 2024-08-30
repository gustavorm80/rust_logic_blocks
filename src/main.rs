use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use block::{
    constants::const_bool::BlockConstBool,
    general::{generic_block::GenericBlock, in_out_exposes::InOutExposes},
    logic_ports::and_port::AndPort,
    state_ports::flip_flop_jk,
    Block, TExecute,
};
use terminal::terminal_out::{TTerminalOut, TerminalOut};

pub mod block;
mod languages;
mod learn;
pub mod terminal;

fn main() {
    let generic = Box::new(flip_flop_jk::new());
    let const_bool = Box::new(BlockConstBool::new(true));

    let const_clk = Box::new(BlockConstBool::new(false));

    let generic_m = &*generic as *const GenericBlock;

    println!("Entradas: {}", generic.get_in_terminals_size());

    println!("Saidas: {}", generic.get_out_terminals_size());

    let blocks: Arc<Mutex<Vec<Box<dyn TExecute>>>> =
        Arc::new(Mutex::new(vec![generic, const_bool, const_clk]));

    let blocks_thread = Arc::clone(&blocks);



    let bl_thread = thread::spawn(move || loop {
        thread::yield_now();

        for _number in 0..100 {
            let mut blc_un = blocks_thread.lock().unwrap();

            if _number == 0 {
                for block in blc_un.iter_mut() {
                    Block::reset_texecute(block.deref_mut());
                }
            }

            for block in blc_un.iter_mut() {
                Block::new_pass_texecute(block.deref_mut());
            }

            let mut executed = false;
            for block in blc_un.iter_mut() {
                if block.execute() {
                    executed = true;
                }
            }

            if !executed {
                break;
            }
        }

        for b in blocks_thread.lock().unwrap().iter() {
            if b.get_name() == "JK" {
                if let Some(block) = b.as_any().downcast_ref::<GenericBlock>(){
                    for terminal_index in 0..block.get_out_terminals_size() {
                        if let Ok(terminal) = block.get_out_terminal_by_index(terminal_index) {
                            let tlock = terminal.lock().unwrap();
                            if tlock.is_new_value() {
                                if let Some(value) = tlock.as_any().downcast_ref::<TerminalOut<bool>>() {
                                    println!("{}: {}", value.get_name(), value.get_value());
                                }
                            }
                        }
                    }
                }
            }
        }
        // print_out_by_name(&blocks_thread, "Q".to_string());
        // print_out_by_name(&blocks_thread, "!Q".to_string());
    });

    thread::spawn(move || loop {
        thread::yield_now();
        thread::sleep(Duration::from_secs(1));
        let mut blc_un = blocks.lock().unwrap();

        // for block in blc_un.iter_mut() {
        //     if (*block).deref().get_name() == "D" {
        //         let out_terminal_result = (*block).deref().get_out_terminal_by_index(0);
        //         match out_terminal_result {
        //             Ok(out) => {
        //                 let mut out_lock = out.lock().unwrap();
        //                 if let Some(out_dc) =
        //                     out_lock.as_any_mut().downcast_mut::<TerminalOut<bool>>()
        //                 {
        //                     (*out_dc).set_value(!out_dc.get_value());

        //                     // println!("Valor Setado: {}", out_dc.get_value());
        //                 }
        //             }
        //             Err(_) => (),
        //         }
        //     }
        // }
    });

    bl_thread.join().unwrap();
}
