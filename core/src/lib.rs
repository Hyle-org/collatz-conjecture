#![no_std]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub initial_state: u32,
    pub suggested_number: u32,
}

pub use hyle_verifier::HyleOutput;
