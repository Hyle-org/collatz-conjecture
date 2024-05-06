#![no_std]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub initial_state: u32,
    pub suggested_number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyleOutput {
    pub initial_state: u32,
    pub next_state: u32,
}
