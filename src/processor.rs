use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, 
    msg, program_error::ProgramError, pubkey::Pubkey, 
};
use crate::instruction::InstructionType;
use crate::instructions::{
    create_manager::create_manager,
    create_stake_account::create_stake_account,
    delegate_stake::delegate_stake_to_validator,
    deactivate_stake::deactivate_stake_fn,
    withdraw_stake::withdraw_stake
};

pub fn process_instruction(program_id:&Pubkey, accounts:&[AccountInfo], instruction_data:&[u8])->ProgramResult{
    let ix=InstructionType::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    match ix{
        InstructionType::CreateManager { allowed_validators ,manager_bump}=>{
            msg!("create manager ix called");
            create_manager(program_id, accounts,allowed_validators,manager_bump)?;
        },
        InstructionType::CreateStakeAccount{stake_amount,manager_bump, user_position_bump}=>{
            msg!("create stake account ix called");
            create_stake_account(program_id, accounts, stake_amount, manager_bump,user_position_bump)?;
        },
        InstructionType::DelegateStake{manager_bump}=>{
            msg!("delegate stake ix called");
            delegate_stake_to_validator(program_id, accounts, manager_bump)?;
        },
        InstructionType::DeactivateStake{manager_bump}=>{
            msg!("deactivate stake ix called");
            deactivate_stake_fn(program_id, accounts, manager_bump)?;
        },
        InstructionType::WithdrawStake { manager_bump , user_position_bump}=>{
            msg!("withdraw stake ix called");
            withdraw_stake(program_id, accounts, manager_bump, user_position_bump)?;
        }
    }
    Ok(())
}