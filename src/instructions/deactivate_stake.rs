use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult,
    program::invoke_signed, program_error::ProgramError,
    pubkey::Pubkey, msg, 
    stake::instruction::{delegate_stake, deactivate_stake },
    config::program::ID
};

use crate::error::StakeManagerErrors;

pub fn deactivate_stake_fn(program_id:&Pubkey, accounts:&[AccountInfo] , manager_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    let stake_acc=next_account_info(&mut accounts_iter)?;
    // let vote_acc=next_account_info(&mut accounts_iter)?;
    let stake_prog=next_account_info(&mut accounts_iter)?;
    let sysvar_clock=next_account_info(&mut accounts_iter)?;
    // let sysvar_stake_history=next_account_info(&mut accounts_iter)?;
    // let stake_config_id=next_account_info(&mut accounts_iter)?;

    msg!("a");
    if !user.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    let manager_seeds=&["manager".as_bytes(),&[manager_bump]];
    let manager_derived_pda=Pubkey::create_program_address(manager_seeds, program_id)?;
    if manager_derived_pda!=*manager_pda.key{
        return Err(StakeManagerErrors::ManagerPdaMismatch.into());
    }
    msg!("b");
    //  let account_metas = vec![
    //     AccountMeta::new(*stake_pubkey, false),
    //     AccountMeta::new_readonly(CLOCK_ID, false),
    //     AccountMeta::new_readonly(*authorized_pubkey, true),
    // ];
    let create_deactivate_stake_ix=deactivate_stake(stake_acc.key,manager_pda.key);
    invoke_signed(&create_deactivate_stake_ix,
        &[stake_acc.clone(), sysvar_clock.clone(), manager_pda.clone()],
        &[manager_seeds])?;
    // let create_delegate_stake_ix=delegate_stake(stake_acc.key,
    //     manager_pda.key, vote_acc.key);
    // msg!("c");
    // invoke_signed(&create_delegate_stake_ix,
    //     &[stake_acc.clone(), vote_acc.clone(), sysvar_clock.clone(),sysvar_stake_history.clone(),
    //     stake_config_id.clone(), manager_pda.clone(), stake_prog.clone()],
    //     &[manager_seeds])?;
    msg!("deactivated stake success!!");
    Ok(())
}