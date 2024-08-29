
use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex}, thread, time::Duration};

use block::{constants::const_bool::{self, BlockConstBool}, general::{generic_block::{self, GenericBlock}, in_out_exposes::InOutExposes}, logic_ports::{and_port::AndPort, nor_port::NOrPort, not_port::NotPort}, TExecute};
use terminal::{
    terminal_in::{self, TerminalIn},
    terminal_out::TerminalOut,
};

pub mod block;
mod languages;
mod learn;
pub mod terminal;

// fn main() {
//     let mut block_bool_d = Box::new(BlockConstBool::new(false));
//     let mut block_bool_e = Box::new(BlockConstBool::new(true));

//     let mut block_not = Box::new(NotPort::new());

//     let mut block_and = Box::new(AndPort::new());
//     let mut block_and2 = Box::new(AndPort::new());

//     let mut block_nor = Box::new(NOrPort::new());
//     let mut block_nor2 = Box::new(NOrPort::new());

//     /*=========================================================
//               Connecting port const D
//     ==========================================================*/
//     if let Err(e) = block_bool_d.connect_out_to_in::<bool>(0, block_not.get_block_mut(), 0) {
//         println!("Erro D to Not: {}", e)
//     }

//     if let Err(e) = block_bool_d.connect_out_to_in::<bool>(0, block_and2.get_block_mut(), 1) {
//         println!("Erro D to AND 2: {}", e)
//     }

//     block_bool_d.set_name("D");

//     /*=========================================================
//                 Connecting port NOT
//     ==========================================================*/
//     if let Err(e) = block_not.connect_out_to_in::<bool>(0, block_and.get_block_mut(), 0) {
//         println!("Erro NOT to AND: {}", e)
//     }

//     /*=========================================================
//                 Connecting port const E
//     ==========================================================*/
//     if let Err(e) = block_bool_e.connect_out_to_in::<bool>(0, block_and.get_block_mut(), 1) {
//         println!("Erro E to AND: {}", e)
//     }

//     if let Err(e) = block_bool_e.connect_out_to_in::<bool>(0, block_and2.get_block_mut(), 0) {
//         println!("Erro E to AND 2: {}", e)
//     }

//     block_bool_e.get_block_mut().set_name("E");

//     /*=========================================================
//               Connecting port AND
//     ==========================================================*/
//     if let Err(e) = block_and.connect_out_to_in::<bool>(0, block_nor.get_block_mut(), 0) {
//         println!("Erro AND to NOR: {}", e)
//     }

//     /*=========================================================
//               Connecting port AND 2
//     ==========================================================*/
//     if let Err(e) = block_and2.connect_out_to_in::<bool>(0, block_nor2.get_block_mut(), 1) {
//         println!("Erro AND 2 to NOR 2: {}", e)
//     }

//     block_and2.get_block_mut().set_name("AND 2");

//     /*=========================================================
//                 Connecting port NOR
//     ==========================================================*/
//     if let Err(e) = block_nor.connect_out_to_in::<bool>(0, block_nor2.get_block_mut(), 0) {
//         println!("Erro NOR to NOR 2: {}", e)
//     }

//     block_nor.set_name("Q");

//     /*=========================================================
//                 Connecting port NOR 2
//     ==========================================================*/
//     if let Err(e) = block_nor2.connect_out_to_in::<bool>(0, block_nor.get_block_mut(), 1) {
//         println!("Erro NOR 2 to NOR: {}", e)
//     }

//     block_nor2.set_name("!Q");

//     let blocks: Arc<Mutex<Vec<Box<dyn TExecute>>>> = Arc::new(Mutex::new(vec![
//         block_bool_d,
//         block_bool_e,
//         block_and,
//         block_and2,
//         block_nor,
//         block_nor2,
//         block_not,
//     ]));

//     let blocks_thread = Arc::clone(&blocks);

//     let bl_thread = thread::spawn(move || loop {
//         thread::yield_now();

//         for _number in 0..100 {
//             let mut blc_un = blocks_thread.lock().unwrap();

//             if _number == 0 {
//                 for block in blc_un.iter_mut() {
//                     (*block).get_block_mut().reset();
//                 }
//             }

//             for block in blc_un.iter_mut() {
//                 (*block).get_block_mut().new_pass();
//             }

//             let mut executed = false;
//             for block in blc_un.iter_mut() {
//                 if block.execute() {
//                     executed = true;
//                 } 
//             }

//             if !executed {
//                 break;
//             }
//         }

//         print_out_by_name(&blocks_thread, "Q".to_string());
//         // print_out_by_name(&blocks_thread, "!Q".to_string());
//     });

//     thread::spawn(move || loop {
//         thread::yield_now();
//         thread::sleep(Duration::from_secs(1));
//         let mut blc_un = blocks.lock().unwrap();

//         for block in blc_un.iter_mut() {
//             if (*block).get_block().get_name() == "D" {
//                 let out_terminal_result = (*block).get_block().get_out_terminal_by_index(0);
//                 match out_terminal_result {
//                     Ok(out) => {
//                         let mut out_lock = out.lock().unwrap();
//                         if let Some(out_dc) =
//                             out_lock.as_any_mut().downcast_mut::<TerminalOut<bool>>()
//                         {
//                             (*out_dc).set_value(!out_dc.get_value());

//                             // println!("Valor Setado: {}", out_dc.get_value());
//                         }
//                     }
//                     Err(_) => (),
//                 }
//             }
//         }
//     });

//     bl_thread.join().unwrap();
// }


fn main() {

    let mut const_bool = Box::new(BlockConstBool::new(true));
    let mut in_block = Box::new(InOutExposes::new(block::general::in_out_exposes::EExposeType::InTerminal));
    let mut and_port = Box::new(AndPort::new());
    let mut out_block = Box::new(InOutExposes::new(block::general::in_out_exposes::EExposeType::OutTerminal));

    const_bool.connect_out_to_in::<bool>(0, &mut and_port, 0).unwrap();
    in_block.connect_out_to_in::<bool>(0, &mut and_port, 1).unwrap();
    and_port.connect_out_to_in::<bool>(0, &mut out_block, 0).unwrap();

    let blocks: Vec<Box<dyn TExecute>> = vec![const_bool, 
        in_block, 
        and_port,
        out_block
    ];

    let mut generic_block = GenericBlock::new(Arc::new(Mutex::new(blocks)));

    generic_block.execute();

    if let Ok(value) = generic_block.get_out_terminal_value_by_index::<bool>(0) {
        println!("Valor de saída: {}", value);
    }

    let mut const_bool = BlockConstBool::new(true);
    
    const_bool.connect_out_to_in::<bool>(0, &mut generic_block, 0).unwrap();

    generic_block.execute();

    if let Ok(value) = generic_block.get_out_terminal_value_by_index::<bool>(0) {
        println!("Valor de saída: {}", value);
    }



    // let mut const_bool = BlockConstBool::new(true);

    // const_bool.connect_out_to_in::<bool>(0, block.deref_mut(), 0);

    // match block.get_out_terminal_value_by_index::<bool>(0) {
    //     Ok(value) =>  println!("Out Value: {}", value),
    //     Err(err) =>  println!("ERRO: {}", err),
    // };

    // block.execute();

    // match block.get_out_terminal_value_by_index::<bool>(0) {
    //     Ok(value) =>  println!("Out Value: {}", value),
    //     Err(err) =>  println!("ERRO: {}", err),
    // };


}