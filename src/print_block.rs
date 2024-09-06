use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::{
    block::{Block, TExecute},
    terminal::{
        terminal_in::{TTerminalIn, TerminalIn},
        terminal_out::{TTerminalOut, TerminalOut},
    },
};

pub fn print(block: &Box<dyn TExecute>) {
    if let Some(downcasted) = block.deref().as_block().downcast_ref::<Block>() {
        // println!(
        //     "Nome: {} | Entradas: {} | Saídas: {}",
        //     downcasted.get_name(),
        //     downcasted.get_in_terminals_size(),
        //     downcasted.get_out_terminals_size()
        // );

        let len = 8 + downcasted.get_name().len();
        let mut init_len: usize = 0;

        let total_in = downcasted.get_in_terminals_size();
        let total_out = downcasted.get_out_terminals_size();

        // let mut line = 0;

        let bigger_len = if total_in > total_out {
            total_in
        } else {
            total_out
        };

        for t in downcasted.get_in_terminals().iter() {
            if t.lock().unwrap().get_name().len() > init_len {
                init_len = t.lock().unwrap().get_name().len();
            }
        }

        print_init(init_len + 3);
        print!("┌");
        for _ in 0..len {
            print!("─");
        }
        println!("┐");

        print_init(init_len + 3);
        print!("│");
        for _ in 0..4 {
            print!(" ");
        }
        print!("{}", downcasted.get_name());
        for _ in 0..4 {
            print!(" ");
        }
        println!("│");

        for i in 0..bigger_len {
            let mut space = len;
            if i < total_in {
                if let Ok(terminal_in) = downcasted.get_in_terminal_by_index(i) {
                    let lock = terminal_in.lock().unwrap();
                    print_conector(&init_len, lock.get_name().to_string());
                    print!("{}", printable_in_value(&*lock));
                    space -= 3;
                } else {
                    print_init(init_len + 3);
                    print!("│");
                }
            } else {
                print_init(init_len + 3);
                print!("│");
            }

            if i < total_out {
                if let Ok(terminal_out) = downcasted.get_out_terminal_by_index(i) {
                    let lock = terminal_out.lock().unwrap();
                    space -= 3;
                    for _ in 0..space {
                        print!(" ");
                    }
                    print!("{}", printable_out_value(&*lock));
                    println!("┼── {}", lock.get_name());
                    
                } else {
                    for _ in 0..space {
                        print!(" ");
                    }
                    println!("│");
                }
            } else {
                for _ in 0..space {
                    print!(" ");
                }
                println!("│");
            }

            print_init(init_len + 3);
            print!("│");
            for _ in 0..len {
                print!(" ");
            }
            println!("│");
        }

        print_init(init_len + 3);
        print!("└");
        for _ in 0..len {
            print!("─");
        }
        println!("┘");
    } else {
        println!("Erro");
    }
}

fn print_init(len: usize) {
    for _ in 0..len {
        print!(" ");
    }
}

fn print_conector(len: &usize, name: String) {
    let before_space = *len - name.len();

    for _ in 0..before_space {
        print!(" ");
    }

    if before_space != *len {
        print!("{} ", name);
        for _ in 0..2 {
            print!("─");
        }
        print!("┼");
    } else {
        print!("│");
    }
}

fn printable_in_value(terminal: &dyn TTerminalIn) -> &str {
    if let Some(ti) = (*terminal).as_any().downcast_ref::<TerminalIn>() {
        ////Try bool
        if let Some(value) = ti.get_value::<bool>() {
            if value {
                "[■]"
            } else {
                "[ ]"
            }
        } else {
            ""
        }
    } else {
        ""
    }
}

fn printable_out_value(terminal: &dyn TTerminalOut) -> &str {
    ////Try bool
    if let Some(to) = (*terminal).as_any().downcast_ref::<TerminalOut<bool>>() {
        if to.get_value() {
            "[■]"
        } else {
            "[ ]"
        }
    } else {
        ""
    }
}
