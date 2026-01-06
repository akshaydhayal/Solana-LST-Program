use borsh::{BorshSerialize, BorshDeserialize};
#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionType{
    InitialiseLST{ lst_manager_bump:u8, lst_manager_vault_bump:u8, lst_mint_bump:u8},
    DepositSOL{ deposit_amount:u64, lst_manager_bump:u8, lst_mint_bump:u8},
    // WithdrawSOL
}