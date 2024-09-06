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
use print_block::print;
use terminal::terminal_out::{self, TTerminalOut, TerminalOut};

pub mod block;
mod languages;
mod learn;
mod print_block;
pub mod terminal;

fn execute_blocks(blocks: &Arc<Mutex<Vec<Box<dyn TExecute>>>>) {
    let mut blc_un = blocks.lock().unwrap();

    for _number in 0..100 {
        if _number == 0 {
            for block in blc_un.iter_mut() {
                block.reset();
            }
        }

        for block in blc_un.iter_mut() {
            block.new_pass();
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
}

fn main() {
    let mut generic = Box::new(flip_flop_jk::new());
    let mut const_bool = Box::new(BlockConstBool::new(true));

    let mut const_clk = Box::new(BlockConstBool::new(false));

    const_clk.set_name("CLK".into());

    if let Err(err) =
        const_bool.connect_out_to_in_by_name::<bool>("Out 1".into(), &mut generic, "SET".into())
    {
        println!("{}", err);
    }

    if let Err(err) =
        const_bool.connect_out_to_in_by_name::<bool>("Out 1".into(), &mut generic, "RST".into())
    {
        println!("{}", err);
    }

    if let Err(err) =
        const_clk.connect_out_to_in_by_name::<bool>("Out 1".into(), &mut generic, "CLK".into())
    {
        println!("{}", err);
    }

    let blocks: Arc<Mutex<Vec<Box<dyn TExecute>>>> =
        Arc::new(Mutex::new(vec![generic, const_bool, const_clk]));

    let blocks_thread: Arc<Mutex<Vec<Box<dyn TExecute>>>> = Arc::clone(&blocks);

    // let x = blocks.lock().unwrap();
    // for i in 0..x.len() {
    //     print(&x[i]);
    // }
    // drop(x);

    // let bl_thread = thread::spawn(move || loop {
    //     thread::yield_now();
    //     execute_blocks(&blocks_thread);
    // });

    let main_thread = thread::spawn(move || loop {
        thread::yield_now();
        thread::sleep(Duration::from_secs(1));
        let mut blc_un = blocks.lock().unwrap();

        for block in blc_un.iter_mut() {
            if (*block).deref().get_name() == "CLK".to_string() {
                let block_deref = (*block).deref_mut();
                let block_downcast = block_deref.as_any_mut().downcast_mut::<BlockConstBool>();

                if let Some(block_const) = block_downcast {
                    if let Ok(terminal) = block_const.get_out_terminal_by_index(0) {
                        let mut terminal_lock = terminal.lock().unwrap();
                        if let Some(terminal) = terminal_lock
                            .as_any_mut()
                            .downcast_mut::<TerminalOut<bool>>()
                        {
                            terminal.set_value(!terminal.get_value());
                            // println!("Valor Setado: {}", terminal.get_value());
                        }
                    }

                    // (*out_terminal).set_value(!out_terminal.get_value());
                } else {
                    println!("Not converted");
                }
            }
        }

        drop(blc_un);

        execute_blocks(&blocks);

        blc_un = blocks.lock().unwrap();

        for block in blc_un.iter_mut() {
            if (*block).deref().get_name() == "JK".to_string(){
                // print(&*block);
                let downcast = block.as_any().downcast_ref::<GenericBlock>();

                if let Some(gb) = downcast {
                    let blocks = gb.get_blocks();

                    let lock_blocks = blocks.lock().unwrap();
                    for ib in lock_blocks.iter() {
                        // println!("{}", ib.get_name());
                        if ib.get_name() == "IN NAND 1" || ib.get_name() == "IN NAND 2"  {
                            print(&ib);
                        }

                    }
                }
            }
        }

        drop(blc_un);


    });

    // bl_thread.join().unwrap();

    main_thread.join().unwrap();
}
