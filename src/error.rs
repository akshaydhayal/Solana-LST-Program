use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StakeManagerErrors{
    #[error("user provided wrong manager pda account")]
    ManagerPdaMismatch
}

impl From<StakeManagerErrors> for ProgramError{
    fn from(e:StakeManagerErrors)->Self{
        return ProgramError::Custom(e as u32)
    }
}