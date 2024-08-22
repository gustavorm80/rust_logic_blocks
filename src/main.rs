use std::{
    borrow::BorrowMut,
    fmt,
    io::{self, Read, Write},
    sync::{Arc, Mutex},
    thread::{self, Scope},
    time::Duration,
};

use block::{constants::const_bool::BlockConstBool, logic_ports::{and_port::AndPort, or_port::OrPort}, TExecute};
use terminal::Terminal;

pub mod block;
mod languages;
mod learn;
pub mod terminal;

// trait TConta: Send {
//     fn add(&mut self, amount: i32);
//     fn sub(&mut self, amount: i32);

//     fn get_owner_mut(&mut self) -> &mut Owner;
//     fn get_owner(&self) -> &Owner;

//     fn get_total(&self) -> &i32;
// }

// #[derive(Debug)]
// struct Owner {
//     name: String,
//     conta: Option<Arc<Mutex<dyn TConta>>>,
// }

// #[derive(Debug)]
// struct Conta {
//     pub total: i32,
//     pub owner: Owner,
// }

// impl fmt::Debug for dyn TConta {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Owner {} has a total of {} in account",
//             (*self).get_owner().name,
//             self.get_total()
//         )
//     }
// }

// impl TConta for Conta {
//     fn add(&mut self, amount: i32) {
//         self.total += amount;
//     }

//     fn sub(&mut self, amount: i32) {
//         self.total -= amount;
//     }

//     fn get_owner(&self) -> &Owner {
//         &self.owner
//     }

//     fn get_owner_mut(&mut self) -> &mut Owner {
//         &mut self.owner
//     }

//     fn get_total(&self) -> &i32 {
//         &self.total
//     }
// }

// fn main() {
//     let mut conta: Arc<Mutex<dyn TConta>> = Arc::new(Mutex::new(Conta {
//         total: 0,
//         owner: Owner {
//             name: "Gustavo".to_string(),
//             conta: None,
//         },
//     }));

//     let t = conta.lock();
//     t.unwrap().get_owner_mut().conta = Some(Arc::clone(&conta));

//     let t = Arc::clone(&conta);
//     let mt = thread::spawn(move || {
//         for _i in 0..10000 {
//             let value = t.lock();
//             let mut m = value.unwrap();
//             m.add(100);
//             println!("Thread soma {}", m.get_total());
//         }
//     });

//     let t2 = Arc::clone(&conta);
//     let mt2 = thread::spawn(move || {
//         for _i in 0..10000 {
//             let value = t2.lock();
//             let mut m = value.unwrap();
//             m.sub(100);
//             println!("Thread subtração {}", m.get_total());
//         }
//     });

//     mt.join().unwrap();
//     mt2.join().unwrap();

//     println!(
//         "Valor total: {}",
//         conta.lock().as_ref().unwrap().get_total()
//     );
// }

fn main() {
    let block_bool = Box::new(BlockConstBool::new(true)) as Box<dyn TExecute>;
    let block_bool2 = Box::new(BlockConstBool::new(true)) as Box<dyn TExecute>;
    let mut block_and = Box::new(AndPort::new()) as Box<dyn TExecute>;
    let mut block_or = Box::new(OrPort::new()) as Box<dyn TExecute>;

    let block_bool_name = block_bool.get_block().get_name().to_string();

    /*=========================================================
               Connecting ports of AND PORT
     ==========================================================*/
    match block_and.connect_to_in_terminal_block(0, block_bool.get_block(), 0) {
        Ok(_) => (),
        Err(e) => println!("Erro: {}", e),
    }

    match block_and.connect_to_in_terminal_block(1, block_bool2.get_block(), 0) {
        Ok(_) => (),
        Err(e) => println!("Erro: {}", e),
    }

    /*=========================================================
               Connecting ports of OR PORT
     ==========================================================*/
     match block_or.connect_to_in_terminal_block(0, block_bool.get_block(), 0) {
        Ok(_) => (),
        Err(e) => println!("Erro: {}", e),
    }

    match block_or.connect_to_in_terminal_block(1, block_bool2.get_block(), 0) {
        Ok(_) => (),
        Err(e) => println!("Erro: {}", e),
    }


    let blocks = Arc::new(Mutex::new(vec![block_bool, block_bool2, block_and, block_or]));

    let blocks_thread = Arc::clone(&blocks);

    let bl_thread = thread::spawn(move || loop {
        thread::yield_now();
        let mut blc_un = blocks_thread.lock().unwrap();

        for block in blc_un.iter_mut() {
            (*block).reset();
        }

        for block in blc_un.iter_mut() {
            // println!("Executando bloco {}", (*block).get_block().get_name());
            if *block.execute() {

                let result = block.get_block().get_out_terminal_value_by_index::<bool>(0);

                match result {
                    Ok(x) => {
                        println!("Out of \"{}\": {}",block.get_block().get_name(),  x);
                    }
                    Err(err) => println!("Erro: {}", err),
                };
            } else {
            }
        }
    });

    thread::spawn(move || loop{
        thread::yield_now();
        thread::sleep(Duration::from_secs(1));
        let mut blc_un = blocks.lock().unwrap();

        for block in blc_un.iter_mut() {
            if (*block).get_block().get_name() == block_bool_name ||  (*block).get_block().get_name() == "Or Port" {
                let in_terminal_result = (*block).get_block().get_out_terminal_by_index(0);
                match in_terminal_result {
                    Ok(x) => {
                        let mut in_term = (*x).lock().unwrap();
                        let downcast = in_term.as_any_mut().downcast_mut::<Terminal<bool>>();
                        match downcast {
                            Some(t) => {
                                t.set_value(!(*t.get_value()));
                            }
                            None => (),
                        }
                    }
                    Err(_) =>  println!("Erro"),
                }
            }
        }
    });

    bl_thread.join().unwrap();
}
