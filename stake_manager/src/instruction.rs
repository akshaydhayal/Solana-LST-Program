use borsh::{BorshSerialize,BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize,BorshDeserialize)]
pub enum InstructionType{
    CreateManager{allowed_validators:Vec<Pubkey>, manager_bump:u8},
    CreateStakeAccount{stake_amount:u64, manager_bump:u8, user_position_bump:u8},
    DelegateStake{manager_bump:u8},
    DeactivateStake{manager_bump:u8},
    WithdrawStake{manager_bump:u8, user_position_bump:u8},
    MergeStakeAccounts{manager_bump:u8},
    SplitStakeAccount{split_amount:u64, manager_bump:u8}, 
}

// 🔹 Redelegate (composed instruction)
// Not a Stake Program ix, but a protocol ix.
// Flow: Deactivate --> Wait epoch --> Delegate to new validator
// You already have primitives — this just orchestrates them.

// 🔹 AutoRotate
// RotateStake { from: Pubkey, to: Pubkey }
// Internally: Deactivate --> Wait epoch --> Delegate elsewhere
// Often cron-driven / keeper-driven.