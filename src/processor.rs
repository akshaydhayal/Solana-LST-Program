use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, 
    msg, program_error::ProgramError, pubkey::Pubkey, 
};
use crate::instruction::InstructionType;
use crate::instructions::{
    create_manager::create_manager
};

pub fn process_instruction(program_id:&Pubkey, accounts:&[AccountInfo], instruction_data:&[u8])->ProgramResult{
    let ix=InstructionType::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    match ix{
        InstructionType::CreateManager { allowed_validators ,manager_bump}=>{
            msg!("create manager ix called");
            create_manager(program_id, accounts,allowed_validators,manager_bump)?;
        }
    }
    Ok(())
}