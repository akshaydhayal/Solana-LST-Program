use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, borsh1::try_from_slice_unchecked, 
    entrypoint::ProgramResult,
    epoch_stake::{get_epoch_stake_for_vote_account,get_epoch_total_stake}, msg,
    program::{invoke, invoke_signed}, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent,
    stake::{
        instruction::{create_account},
        state::{Authorized, Lockup, StakeState}, tools::get_minimum_delegation
    },
    sysvar::Sysvar
};

use crate::{error::StakeManagerErrors, state::{Manager, UserPosition}};

pub fn create_stake_account(program_id:&Pubkey, accounts:&[AccountInfo], stake_amount:u64, manager_bump:u8, user_position_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    let user_position_pda=next_account_info(&mut accounts_iter)?;
    let stake_acc=next_account_info(&mut accounts_iter)?;
    let system_prog=next_account_info(&mut accounts_iter)?;
    let stake_prog=next_account_info(&mut accounts_iter)?;
    let system_rent_prog=next_account_info(&mut accounts_iter)?;

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
    let manager_seeds:&[&[u8]]=&[b"manager",&manager_bump.to_le_bytes()];
    let manager_derived_pda=Pubkey::create_program_address(manager_seeds, program_id)?;
    if manager_derived_pda!=*manager_pda.key{
        return Err(StakeManagerErrors::ManagerPdaMismatch.into());
    }
    
    //dont create user position pda is user has alreday created stake account before to remove previous data
    if !user_position_pda.data_is_empty(){
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    let user_position_seeds=&[b"position", user.key.as_ref(), &user_position_bump.to_le_bytes()];
    let user_position_derived_pda=Pubkey::create_program_address(user_position_seeds, program_id)?;
    if user_position_derived_pda!=*user_position_pda.key{
        return Err(StakeManagerErrors::UserPositionPdaMismatch.into());
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
            
    //create user_position Pda to track which stake account belongs to whom
    let user_position_pda_rent=rent.minimum_balance(UserPosition::USER_POSITION_SIZE);
    let create_user_position_pda_ix=solana_program::system_instruction::create_account(
        user.key, user_position_pda.key,
        user_position_pda_rent, UserPosition::USER_POSITION_SIZE as u64, program_id);
    msg!("user position_pda_rent : {}",user_position_pda_rent);
    invoke_signed(&create_user_position_pda_ix,
        &[user.clone(), user_position_pda.clone(), system_prog.clone()],
        &[user_position_seeds])?;
    msg!("user position_pda created");
    
    let user_position_data=UserPosition{owner:*user.key, stake_acc:*stake_acc.key, deposited_amount:stake_amount};
    user_position_data.serialize(&mut *user_position_pda.data.borrow_mut())?;

    // let mut manager_pda_data=Manager::try_from_slice(&manager_pda.data.borrow())?;
    let mut manager_pda_data:Manager=try_from_slice_unchecked(&manager_pda.data.borrow())?;
    manager_pda_data.total_staked+=stake_amount;
    manager_pda_data.serialize(&mut *manager_pda.data.borrow_mut())?;
    Ok(())
}