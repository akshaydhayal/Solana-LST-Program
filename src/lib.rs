use solana_program::{
    entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,msg,
    account_info::{next_account_info, AccountInfo},
    epoch_stake::{get_epoch_stake_for_vote_account,get_epoch_total_stake},
    stake::{
        program::ID,
        state::StakeState,
        tools::get_minimum_delegation
    },
};
use borsh::{BorshDeserialize,BorshSerialize};

entrypoint!(process_instruction);

pub fn process_instruction(program_id:&Pubkey, accounts:&[AccountInfo], instruction_data:&[u8])->ProgramResult{
    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let vote_acc=next_account_info(&mut accounts_iter)?;

    let a=get_epoch_total_stake();
    let b=get_minimum_delegation()?;
    let c=get_epoch_stake_for_vote_account(vote_acc.key);

    msg!("get_epoch_total_stake : {}",a);
    msg!("get_minimum_delegation : {}",b);
    msg!("get_epoch_stake_for_vote_account : {}",c);
    Ok(())
}