use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, borsh1::try_from_slice_unchecked, clock::{Clock, sysvar}, config::program::ID,
    entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, msg, program::{invoke, invoke_signed}, 
    program_error::ProgramError, pubkey::Pubkey, rent::Rent,
    stake::instruction::{StakeInstruction, deactivate_stake, delegate_stake, merge, move_stake, split, withdraw}, sysvar::Sysvar
};
use crate::{error::StakeManagerErrors, state::{Manager, UserPosition}};

pub fn split_stake_accounts(program_id:&Pubkey, accounts:&[AccountInfo] ,split_amount:u64, manager_bump:u8)->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let manager_pda=next_account_info(&mut accounts_iter)?;
    // let user_position_pda=next_account_info(&mut accounts_iter)?;
    let stake_acc=next_account_info(&mut accounts_iter)?;
    let new_split_stake_acc=next_account_info(&mut accounts_iter)?;
    let stake_prog=next_account_info(&mut accounts_iter)?;
    // let sysvar_clock=next_account_info(&mut accounts_iter)?;
    // let sysvar_stake_history=next_account_info(&mut accounts_iter)?;

    if !user.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    let manager_seeds=&["manager".as_bytes(),&[manager_bump]];
    let manager_derived_pda=Pubkey::create_program_address(manager_seeds, program_id)?;
    if manager_derived_pda!=*manager_pda.key{
        return Err(StakeManagerErrors::ManagerPdaMismatch.into());
    }

    let rent=Rent::get()?;
    let stake_acc_size=solana_program::stake::state::StakeStateV2::size_of();

    //create new split stake account
    let ix=Instruction::new_with_bincode(
        solana_program::system_program::ID,
        &solana_program::system_instruction::SystemInstruction::CreateAccount{
            lamports: rent.minimum_balance(stake_acc_size),
            space: stake_acc_size as u64,
            owner: solana_program::stake::program::ID
        },
        vec![
            AccountMeta{pubkey:*user.key , is_signer:true, is_writable:true},
            AccountMeta{pubkey:*new_split_stake_acc.key , is_signer:true, is_writable:true},
        ]
    ); 
    invoke(&ix, &[user.clone(), new_split_stake_acc.clone()])?;


    // let create_split_stake_account_ix=split(stake_pubkey, authorized_pubkey, lamports, split_stake_pubkey)
    let a=StakeInstruction::Split(split_amount);
    let mut split_amount_temp:u64=0;
    match a{
        StakeInstruction::Split(x)=>{
            split_amount_temp=x;
            msg!("split amount : {:?}",split_amount_temp);
            msg!("split amount : {:?}",split_amount_temp.to_le_bytes());
            msg!("split amount : {:?}",split_amount_temp.to_le_bytes().to_vec());
        },
        _=>{}
    }
    // # Account references
    //   0. `[WRITE]` Stake account to be split; must be in the Initialized or Stake state
    //   1. `[WRITE]` Uninitialized stake account that will take the split-off amount
    //   2. `[SIGNER]` Stake authority 
    let mut serialised_ix_data=split_amount_temp.to_le_bytes().to_vec();
    serialised_ix_data.insert(0, 3);
    msg!("serialised_ix_data : {:?}",serialised_ix_data);

    let create_split_stake_account_ix=Instruction{
        program_id:solana_program::stake::program::ID,
        accounts:vec![
            AccountMeta{pubkey:*stake_acc.key, is_signer:false, is_writable:true},
            AccountMeta{pubkey:*new_split_stake_acc.key, is_signer:false, is_writable:true},
            AccountMeta{pubkey:*manager_pda.key, is_signer:true, is_writable:false}
            ],
            // data:split_amount_temp.to_le_bytes().to_vec()
            data:serialised_ix_data
        };
        let create_split_stake_account_ix2=Instruction::new_with_bincode(
            solana_program::stake::program::ID,
            &solana_program::stake::instruction::StakeInstruction::Split(split_amount),
            vec![
                AccountMeta{pubkey:*stake_acc.key, is_signer:false, is_writable:true},
                AccountMeta{pubkey:*new_split_stake_acc.key, is_signer:false, is_writable:true},
                AccountMeta{pubkey:*manager_pda.key, is_signer:true, is_writable:false}
            ]
        );

    invoke_signed(&create_split_stake_account_ix2,
        &[stake_acc.clone(), new_split_stake_acc.clone(), manager_pda.clone()],
        &[manager_seeds])?;


    // //now close/delete user position pda and update manager pda total staked field 
    // let mut manager_pda_data:Manager=try_from_slice_unchecked(&manager_pda.data.borrow())?;
    // manager_pda_data.total_staked-=user_position_pda_data.deposited_amount;
    // user_position_pda_data.deposited_amount=0;

    // manager_pda_data.serialize(&mut *manager_pda.data.borrow_mut())?;
    // user_position_pda_data.serialize(&mut *user_position_pda.data.borrow_mut())?;
    Ok(())
}