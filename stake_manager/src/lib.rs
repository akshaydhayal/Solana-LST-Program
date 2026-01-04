use solana_program::{entrypoint};

pub mod processor;
pub mod instruction;
pub mod instructions;
pub mod state;
pub mod error;

use processor::process_instruction;
entrypoint!(process_instruction);