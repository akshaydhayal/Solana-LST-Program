use borsh::{BorshSerialize,BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize,BorshDeserialize)]
pub enum InstructionType{
    CreateManager{allowed_validators:Vec<Pubkey>, manager_bump:u8},
    CreateStakeAccount{stake_amount:u64, manager_bump:u8, user_position_bump:u8},
    DelegateStake{manager_bump:u8},
    // UnStake,
    // Withdraw
}