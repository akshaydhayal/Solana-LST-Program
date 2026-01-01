use borsh::{BorshSerialize,BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize,BorshDeserialize)]
pub enum InstructionType{
    CreateManager{allowed_validators:Vec<Pubkey>, manager_bump:u8},
    CreateStakeAccount{stake_amount:u64, manager_bump:u8},
    // DelegateStake{delegate_stake_amount:u64},
    // UnStake,
    // Withdraw
}