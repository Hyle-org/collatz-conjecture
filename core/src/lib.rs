#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub initial_state: u32,
    pub suggested_number: u32,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct HyleOutput {
    pub initial_state: Vec<u8>,
    pub next_state: Vec<u8>,
}
