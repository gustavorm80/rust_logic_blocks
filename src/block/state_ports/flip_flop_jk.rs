use std::{ops::Deref, sync::{Arc, Mutex}};

use crate::block::{
    general::{generic_block::GenericBlock, in_out_exposes::InOutExposes},
    logic_ports::nand_port::NAndPort,
    TExecute,
};

pub fn new() -> GenericBlock {
    let mut nand1 = Box::new(NAndPort::new());
    let mut nand2 = Box::new(NAndPort::new());
    let mut nand3 = Box::new(NAndPort::new());
    let mut nand4 = Box::new(NAndPort::new());

    let mut set = Box::new(InOutExposes::new(crate::block::general::in_out_exposes::EExposeType::InTerminal));
    let mut clk = Box::new(InOutExposes::new(crate::block::general::in_out_exposes::EExposeType::InTerminal));
    let mut reset =
    Box::new(InOutExposes::new(crate::block::general::in_out_exposes::EExposeType::InTerminal));

    let mut q = Box::new(InOutExposes::new(crate::block::general::in_out_exposes::EExposeType::OutTerminal));
    let mut not_q =
    Box::new(InOutExposes::new(crate::block::general::in_out_exposes::EExposeType::OutTerminal));

    nand1.set_name("IN NAND 1");
    nand2.set_name("IN NAND 2");
    nand3.set_name("NAND Q");
    nand4.set_name("NAND !Q");

    clk.set_name("EXP. CLK");
    set.set_name("SET");
    reset.set_name("RST");

    q.set_name("Block Q");
    not_q.set_name("Block Not Q");

    nand1.add_new_in_terminal();
    nand2.add_new_in_terminal();

    set.connect_out_to_in::<bool>(0, &mut nand1, 0).unwrap();

    clk.connect_out_to_in::<bool>(0, &mut nand1, 1).unwrap();
    clk.connect_out_to_in::<bool>(0, &mut nand2, 1).unwrap();

    reset.connect_out_to_in::<bool>(0, &mut nand2, 2).unwrap();

    nand1.connect_out_to_in::<bool>(0, &mut nand3, 0).unwrap();
    nand2.connect_out_to_in::<bool>(0, &mut nand4, 1).unwrap();

    nand3.connect_out_to_in::<bool>(0, &mut nand2, 0).unwrap();
    nand3.connect_out_to_in::<bool>(0, &mut nand4, 0).unwrap();
    nand3.connect_out_to_in::<bool>(0, &mut q, 0).unwrap();

    nand4.connect_out_to_in::<bool>(0, &mut nand1, 2).unwrap();
    nand4.connect_out_to_in::<bool>(0, &mut nand3, 1).unwrap();
    nand4.connect_out_to_in::<bool>(0, &mut not_q, 0).unwrap();

    {
        let mut to = q.out_terminals[0].lock().unwrap();
        to.set_name("Q".to_string());

        let mut to = not_q.out_terminals[0].lock().unwrap();
        to.set_name("!Q".to_string());

        let mut to = clk.in_terminals[0].lock().unwrap();
        to.set_name("CLK".to_string());

        let mut to = clk.out_terminals[0].lock().unwrap();
        to.set_name("OUT CLK".to_string());

        
        let mut to = set.in_terminals[0].lock().unwrap();
        to.set_name("SET".to_string());        

        let mut to = reset.in_terminals[0].lock().unwrap();
        to.set_name("RST".to_string());
    }

    let vectors: Vec<Box<dyn TExecute>> = vec![nand1, nand2, nand3, nand4, set, clk, reset, q, not_q];
    let blocks = Arc::new(Mutex::new(vectors));

    let mut block = GenericBlock::new(blocks);
    block.set_name("JK");

    block
}

// impl FlipFlopJK {
//     pub fn new() -> Self {
//         let mut block = GenericBlock::new("And Port");
//         let out_nand: Arc<Mutex<dyn TTerminalOut>> =
//             Arc::new(Mutex::new(TerminalOut::new("Out 1".to_string(), false)));

//         let in_a: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 1".to_string());
//         let in_b: Arc<Mutex<dyn TTerminalIn>> = TerminalIn::new("In 2".to_string());
//     }
// }
