#![feature(default_free_fn)]
use blake2::digest::generic_array::GenericArray;
use blake2::digest::{FixedOutput, Output};
use blake2::{Blake2b512, Blake2s256, Digest};
use hex_literal::hex;
use std::default::default;
use std::rc::Rc;
use std::str::from_utf8;

static mut SERIAL: usize = 0;
#[derive(Debug)]
struct Block {
    serial: usize,
    data: Box<Vec<u8>>,
    next_block: Option<Box<Block>>,
    hash: Option<Box<Vec<u8>>>,
}

impl Block {
    unsafe fn new(data: Vec<u8>) -> Self {
        let mut hasher = Blake2b512::new_with_prefix(&data);
        let res_hashing = hasher.finalize().to_vec();
        let out = Block {
            serial: SERIAL,
            data: Box::from(data.to_owned()), // move here
            hash: Some(Box::from(res_hashing.to_owned())),
            next_block: None,
        };
        SERIAL += 1;
        out
    }
    unsafe fn append(&mut self, new_data: Vec<u8>) {
        match self.next_block {
            Some(ref mut nex_bl) => {
                println!("append a block with next null block, data");
                nex_bl.append(new_data);
            }
            None => unsafe { // assign nex block to Some
                let mut hasher = Blake2b512::new_with_prefix(&new_data);
                let rezult_hashing = hasher.finalize().to_vec();
                let nex_block = Block {
                    serial: SERIAL,
                    data: Box::from(new_data),
                    next_block: None,
                    hash: Some(Box::from(rezult_hashing)),
                };
                self.next_block = Some(Box::from(nex_block)); // assign
                println!("appended Some to next block");
            },
        }
        SERIAL += 1; // next
    }
    fn printout(&self) {
        match self.next_block {
            Some(ref bl) => {
                println!("hash is {:02X?}", (&self.hash.as_ref().unwrap().to_vec()));
                bl.printout();
            }
            None => {
                println!("nothing!");
            }
        }
    }
    fn count(&self) -> i32 {
        match self.next_block {
            None => 0,
            Some(ref nb) => 1 + nb.count(),
        }
    }
}

fn main() {
    let mut b0 = unsafe { Block::new(String::from("hello world").into_bytes()) };
    b0.printout();
    unsafe {
        b0.append(String::from("hello world2").into_bytes());
    }
    b0.printout();
    unsafe {
        b0.append(String::from("hello world 2").into_bytes());
    }
    b0.printout();
    println!("value is {}", b0.count());
  //  b0.printout();
    /*assert_eq!(res[..], hex!("
    021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbc
    c05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0
    ")[..]);*/
}
