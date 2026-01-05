use borsh::{BorshSerialize, BorshDeserialize};
#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionType{
    InitialiseLST{ total_lst_supply:u64, lst_manager_bump:u8, lst_manager_vault_bump:u8},
    DepositSOL{ deposit_amount:u64, lst_manager_bump:u8},
    // WithdrawSOL
}