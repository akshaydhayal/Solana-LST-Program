use thiserror::{Error};
use solana_program::program_error::ProgramError;

#[derive(Error,Debug)]
pub enum LSTErrors{
    #[error("given lst manager pda seeds do not match with correct lst manager seeds")]
    LSTManagerPdaMismatch,
    #[error("given lst manager vault pda seeds do not match with correct lst manager vault seeds")]
    LSTManagerVaultPdaMismatch,
    #[error("given lst mint pda seeds do not match with correct lst mint seeds")]
    LSTMintPdaMismatch,
    #[error("given stake manager pda seeds do not match with correct stake manager seeds")]
    StakeManagerPdaMismatch
}

impl From<LSTErrors> for ProgramError{
    fn from(e:LSTErrors)->Self{
        return ProgramError::Custom(e as u32);
    }
}