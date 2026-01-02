use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, borsh1::try_from_slice_unchecked, clock::sysvar, config::program::ID, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, stake::instruction::{deactivate_stake, delegate_stake, withdraw }
};

use crate::{error::StakeManagerErrors, state::{Manager, UserPosition}};

pub fn withdraw_stake(program_id:&Pubkey, accounts:&[AccountInfo] , manager_bump:u8, user_position_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    let user_position_pda=next_account_info(&mut accounts_iter)?;
    let stake_acc=next_account_info(&mut accounts_iter)?;
    let stake_prog=next_account_info(&mut accounts_iter)?;
    let sysvar_clock=next_account_info(&mut accounts_iter)?;
    let sysvar_stake_history=next_account_info(&mut accounts_iter)?;

    if !user.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    let manager_seeds=&["manager".as_bytes(),&[manager_bump]];
    let manager_derived_pda=Pubkey::create_program_address(manager_seeds, program_id)?;
    if manager_derived_pda!=*manager_pda.key{
        return Err(StakeManagerErrors::ManagerPdaMismatch.into());
    }

    let user_position_seeds=&["position".as_bytes(), user.key.as_ref(), &[user_position_bump]];
    let user_position_derived_pda=Pubkey::create_program_address(user_position_seeds, program_id)?;
    if user_position_derived_pda!=*user_position_pda.key{
        return Err(StakeManagerErrors::UserPositionPdaMismatch.into());
    }
    let mut user_position_pda_data=UserPosition::try_from_slice(&user_position_pda.data.borrow())?;
    
    // let mut account_metas = vec![
    //     AccountMeta::new(*stake_pubkey, false),
    //     AccountMeta::new(*to_pubkey, false),
    //     AccountMeta::new_readonly(CLOCK_ID, false),
    //     AccountMeta::new_readonly(STAKE_HISTORY_ID, false),
    //     AccountMeta::new_readonly(*withdrawer_pubkey, true),
    // ];
    let create_withdraw_stake_ix=withdraw(stake_acc.key,
        manager_pda.key, user.key,
        user_position_pda_data.deposited_amount, None);
    invoke_signed(&create_withdraw_stake_ix,
        &[stake_acc.clone(), user.clone(), sysvar_clock.clone(),
        sysvar_stake_history.clone(), manager_pda.clone()],
        &[manager_seeds])?;
    // let create_deactivate_stake_ix=deactivate_stake(stake_acc.key,manager_pda.key);
    // invoke_signed(&create_deactivate_stake_ix,
    //     &[stake_acc.clone(), sysvar_clock.clone(), manager_pda.clone()],
    //     &[manager_seeds])?;
    msg!("withdraw stake success!!");

    //now close/delete user position pda and update manager pda total staked field 
    let mut manager_pda_data:Manager=try_from_slice_unchecked(&manager_pda.data.borrow())?;
    manager_pda_data.total_staked-=user_position_pda_data.deposited_amount;
    user_position_pda_data.deposited_amount=0;

    manager_pda_data.serialize(&mut *manager_pda.data.borrow_mut())?;
    user_position_pda_data.serialize(&mut *user_position_pda.data.borrow_mut())?;
    Ok(())
}