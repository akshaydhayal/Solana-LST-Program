use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, 
    epoch_stake::{get_epoch_stake_for_vote_account,get_epoch_total_stake}, 
    msg, program::{invoke, invoke_signed}, program_error::ProgramError, 
    pubkey::Pubkey, rent::Rent, 
    stake::{
        instruction::create_account, state::{Authorized, Lockup, StakeState}, tools::get_minimum_delegation
    }, 
    sysvar::Sysvar
};

use crate::{error::StakeManagerErrors, state::Manager};

pub fn create_stake_account(program_id:&Pubkey, accounts:&[AccountInfo], stake_amount:u64, manager_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    let stake_acc=next_account_info(&mut accounts_iter)?;
    let system_prog=next_account_info(&mut accounts_iter)?;
    let stake_prog=next_account_info(&mut accounts_iter)?;
    let system_rent_prog=next_account_info(&mut accounts_iter)?;
    msg!("create_stake_account ckp1");
    if !user.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    msg!("create_stake_account ckp1.1");
    // if !manager_pda.data_is_empty(){
    //     return Err(ProgramError::AccountAlreadyInitialized);
    // }
    if *stake_prog.key!=solana_program::stake::program::ID{
        return Err(StakeManagerErrors::WrongStakeProgramId.into());
    }
    let manager_seeds=&[b"manager",user.key.as_ref(),&manager_bump.to_le_bytes()];
    let manager_derived_pda=Pubkey::create_program_address(manager_seeds, program_id)?;
    if manager_derived_pda!=*manager_pda.key{
        return Err(StakeManagerErrors::ManagerPdaMismatch.into());
    }
    
    let rent=Rent::get()?;
    let stake_acc_rent=rent.minimum_balance(solana_program::stake::state::StakeStateV2::size_of());
    
    let ix1=solana_program::system_instruction::create_account(user.key,
        stake_acc.key, 
        stake_acc_rent + stake_amount,
        solana_program::stake::state::StakeStateV2::size_of() as u64,
        &solana_program::stake::program::ID
    );
    invoke(&ix1,
        &[user.clone(), stake_acc.clone(), system_prog.clone()])?;
    msg!("stake account created");

    let ix2=solana_program::stake::instruction::initialize(stake_acc.key,
        &Authorized { staker: *manager_pda.key, withdrawer: *manager_pda.key },
        &Lockup::default());
    invoke(&ix2,
        &[stake_acc.clone(), system_rent_prog.clone(), stake_prog.clone()])?;
    msg!("stake account initialised");
            
    // let ix2=solana_program::stake::instruction::initialize(stake_pubkey, authorized, lockup)
    // let rent=Rent::get()?;
    // let manager_pda_rent=rent.minimum_balance(Manager::MANAGER_SIZE);
    // let create_manager_pda_ix=create_account(user.key, manager_pda.key,
    //     manager_pda_rent, Manager::MANAGER_SIZE as u64, program_id);
    // invoke_signed(&create_manager_pda_ix,
    //     &[user.clone(), manager_pda.clone(), system_prog.clone()],
    //     &[manager_seeds])?;

    let mut manager_pda_data=Manager::try_from_slice(&manager_pda.data.borrow())?;
    manager_pda_data.total_staked+=stake_amount;
    manager_pda_data.serialize(&mut *manager_pda.data.borrow_mut())?;
    Ok(())
}