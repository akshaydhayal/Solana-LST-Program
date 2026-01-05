use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction::{SystemInstruction, create_account}, sysvar::Sysvar
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{error::LSTErrors, state::LSTManager};

pub fn initialise_lst(program_id:&Pubkey, accounts:&[AccountInfo], total_lst_supply:u64, lst_manager_bump:u8, lst_manager_vault_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let stake_manager_pda=next_account_info(&mut accounts_iter)?;
    let lst_manager_pda=next_account_info(&mut accounts_iter)?;
    let lst_manager_vault_pda=next_account_info(&mut accounts_iter)?;
    let lst_mint=next_account_info(&mut accounts_iter)?;
    let system_prog=next_account_info(&mut accounts_iter)?;

    if !user.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !lst_manager_pda.data_is_empty(){
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    if *system_prog.key!=solana_program::system_program::ID{
        return Err(ProgramError::IncorrectProgramId);
    }

    let lst_manager_seeds=&["lst_manager".as_bytes(), &lst_manager_bump.to_le_bytes()];
    let lst_manager_derived=Pubkey::create_program_address(lst_manager_seeds,program_id)?;
    if *lst_manager_pda.key!=lst_manager_derived{
        return Err(LSTErrors::LSTManagerPdaMismatch.into());
    }

    let lst_manager_vault_seeds=&["lst_manager_vault".as_bytes(), lst_manager_vault_pda.key.as_ref(), &lst_manager_vault_bump.to_le_bytes()];
    let lst_manager_vault_derived=Pubkey::create_program_address(lst_manager_vault_seeds,program_id)?;
    if *lst_manager_vault_pda.key!=lst_manager_vault_derived{
        return Err(LSTErrors::LSTManagerVaultPdaMismatch.into());
    }
    //@q do we need to derive stake manager pda
    // let stake_manager_seeds=&["manager".as_bytes(), &stake_manager_bump.to_le_bytes()];
    // let stake_manager_derived=Pubkey::create_program_address(stake_manager_seeds,program_id)?;
    // if *stake_manager_pda.key!=stake_manager_derived{
    //     return Err(LSTErrors::StakeManagerPdaMismatch.into());
    // }

    let rent=Rent::get()?;
    let create_lst_manager_ix=Instruction::new_with_bincode(
        *system_prog.key,
        &SystemInstruction::CreateAccount {
            lamports: rent.minimum_balance(LSTManager::LST_MANAGER_SIZE),
            space: LSTManager::LST_MANAGER_SIZE as u64,
            owner: *program_id
        },
        vec![
            AccountMeta{pubkey:*user.key, is_signer:true, is_writable:true},
            AccountMeta{pubkey:*lst_manager_pda.key, is_signer:true, is_writable:true}
        ]);
    invoke_signed(&create_lst_manager_ix,
        &[user.clone(), lst_manager_pda.clone(), system_prog.clone()],
        &[lst_manager_seeds])?;
    msg!("lst manager pda created");

    let create_lst_manager_vault_pda_ix=Instruction::new_with_bincode(
        *system_prog.key,
        &SystemInstruction::CreateAccount {
            lamports: rent.minimum_balance(0), space: 0, owner: *lst_manager_pda.key
        },
        vec![
            AccountMeta{pubkey:*user.key, is_signer:true, is_writable:true},
            AccountMeta{pubkey:*lst_manager_vault_pda.key, is_signer:true, is_writable:true}
        ]);
    invoke_signed(&create_lst_manager_vault_pda_ix,
        &[user.clone(), lst_manager_vault_pda.clone(), system_prog.clone()],
        &[lst_manager_seeds])?;  
    msg!("lst manager vault pda created");

    let lst_manager_pda_data=LSTManager{
        admin:*user.key,
        stake_manager:*stake_manager_pda.key,
        lst_mint:*lst_mint.key,
        total_sol_staked:0,
        total_lst_supply:total_lst_supply
    };
    lst_manager_pda_data.serialize(&mut *lst_manager_pda.data.borrow_mut())?;
    Ok(())
}