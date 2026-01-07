use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    borsh1::try_from_slice_unchecked,
    clock::Clock, sysvar::Sysvar,clock::sysvar,
    config::program::ID, entrypoint::ProgramResult, instruction::Instruction, instruction::AccountMeta,
    msg, program::invoke_signed, program_error::ProgramError,
    pubkey::Pubkey, stake::instruction::{deactivate_stake, delegate_stake,
    withdraw, merge , move_stake, split}
};

use crate::{error::StakeManagerErrors, state::{Manager, UserPosition}};

pub fn merge_stake_accounts(program_id:&Pubkey, accounts:&[AccountInfo] , manager_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    // let user_position_pda=next_account_info(&mut accounts_iter)?;
    let dest_stake_acc=next_account_info(&mut accounts_iter)?;
    let src_stake_acc=next_account_info(&mut accounts_iter)?;
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

    // let user_position_seeds=&["position".as_bytes(), user.key.as_ref(), &[user_position_bump]];
    // let user_position_derived_pda=Pubkey::create_program_address(user_position_seeds, program_id)?;
    // if user_position_derived_pda!=*user_position_pda.key{
    //     return Err(StakeManagerErrors::UserPositionPdaMismatch.into());
    // }
    // let mut user_position_pda_data=UserPosition::try_from_slice(&user_position_pda.data.borrow())?;
    
    // // let create_merge_stake_accounts_ix=merge(destination_stake_pubkey, source_stake_pubkey, authorized_pubkey)

    //   0. `[WRITE]` Destination stake account for the merge
    //   1. `[WRITE]` Source stake account for to merge.  This account will be drained
    //   2. `[]` Clock sysvar
    //   3. `[]` Stake history sysvar that carries stake warmup/cooldown history
    //   4. `[SIGNER]` Stake authority
    // let a=solana_program::stake::instruction::StakeInstruction::Merge;
    let merge_stake_accounts_ix=Instruction{
        program_id:solana_program::stake::program::ID,
        accounts:vec![
            AccountMeta::new(*dest_stake_acc.key, false),
            AccountMeta::new(*src_stake_acc.key, false),
            AccountMeta::new_readonly(*sysvar_clock.key, false),
            AccountMeta::new_readonly(*sysvar_stake_history.key, false),
            AccountMeta::new(*manager_pda.key, true),
        ],
        data:vec![7,0,0,0]   // merge instruction is 8th variant in StakeInstruction enum 
    };
    invoke_signed(&merge_stake_accounts_ix,
        &[dest_stake_acc.clone(), src_stake_acc.clone(), sysvar_clock.clone(),
        sysvar_stake_history.clone(), manager_pda.clone()],
        &[manager_seeds])?;

    // //now close/delete user position pda and update manager pda total staked field 
    // let mut manager_pda_data:Manager=try_from_slice_unchecked(&manager_pda.data.borrow())?;
    // manager_pda_data.total_staked-=user_position_pda_data.deposited_amount;
    // user_position_pda_data.deposited_amount=0;

    // manager_pda_data.serialize(&mut *manager_pda.data.borrow_mut())?;
    // user_position_pda_data.serialize(&mut *user_position_pda.data.borrow_mut())?;
    Ok(())
}