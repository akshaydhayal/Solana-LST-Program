use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, epoch_stake::{get_epoch_stake_for_vote_account,get_epoch_total_stake}, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, stake::{
        state::StakeState,
        tools::get_minimum_delegation
    }, system_instruction::create_account, sysvar::Sysvar
};

use crate::{error::StakeManagerErrors, state::Manager};

pub fn create_manager(program_id:&Pubkey, accounts:&[AccountInfo], allowed_validators:Vec<Pubkey>, manager_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    let system_prog=next_account_info(&mut accounts_iter)?;
    // let vote_acc=next_account_info(&mut accounts_iter)?;

    if !user.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !manager_pda.data_is_empty(){
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    let manager_seeds=&[b"manager",user.key.as_ref(),&manager_bump.to_le_bytes()];
    let manager_derived_pda=Pubkey::create_program_address(manager_seeds, program_id)?;
    if manager_derived_pda!=*manager_pda.key{
        return Err(StakeManagerErrors::ManagerPdaMismatch.into());
    }

    let rent=Rent::get()?;
    let manager_pda_rent=rent.minimum_balance(Manager::MANAGER_SIZE);
    let create_manager_pda_ix=create_account(user.key, manager_pda.key,
        manager_pda_rent, Manager::MANAGER_SIZE as u64, program_id);
    invoke_signed(&create_manager_pda_ix,
        &[user.clone(), manager_pda.clone(), system_prog.clone()],
        &[manager_seeds])?;

    let manager_pda_data=Manager{admin:*user.key, total_staked:0,allowed_validators};
    manager_pda_data.serialize(&mut *manager_pda.data.borrow_mut())?;
    Ok(())
}