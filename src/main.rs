//use std::env;
//use std::io;
//use std::io::prelude::*;
//use std::fs::File;
use vm::VM;
mod vm;

fn main() {
    // match env::args().nth(1) {
    //     Some(bin_path) => {
    //         match File::open(bin_path){
    //             Ok(mut f) => {
    //                 let mut bytecode = [0; 1_000_000];
    //                 match f.read(&mut bytecode) {
    //                     Ok(_) => {
    //                         let mut vm = VM::new(&bytecode);
    //                         vm.run();
    //                     },
    //                     Err(_) => println!("Error reading from file")
    //                 }
    //             },
    //             Err(_) => println!("Cannot open file descriptor")
    //         }
    //     },
    //     None => {
    //         println!("Usage ./vm binary");
    //     }
    // }

    let code: [u8; 19] = [1, 1,0,0,0,0,0,0,0, 6, 0,0,0,0,0,0,0,0, 28];
    let mut vm = VM::new(&code);
    vm.run();
}