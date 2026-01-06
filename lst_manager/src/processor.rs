use solana_program::{
    account_info::{AccountInfo, next_account_info},
    pubkey::Pubkey, entrypoint::ProgramResult,
    program_error::ProgramError, msg
};
use borsh::{BorshDeserialize};
use crate::instruction::InstructionType;
use crate::instructions::{
    initialise_lst::initialise_lst,
    deposit_sol::deposit_sol
};

pub fn process_instruction(program_id:&Pubkey, accounts:&[AccountInfo], instruction_data:&[u8])->ProgramResult{
    let ix=InstructionType::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    match ix{
        InstructionType::InitialiseLST { lst_manager_bump, lst_manager_vault_bump,lst_mint_bump}=>{
            msg!("initialise LST ix called");
            initialise_lst(program_id, accounts, lst_manager_bump, lst_manager_vault_bump,lst_mint_bump)?;
        },
        InstructionType::DepositSOL { deposit_amount ,lst_manager_bump,lst_mint_bump}=>{
            msg!("deposit SOL ix called");
            deposit_sol(program_id, accounts, deposit_amount, lst_manager_bump,lst_mint_bump)?;
        }
    }
    Ok(())
}