use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize,BorshDeserialize)]
pub struct Manager{
    pub admin:Pubkey,
    pub total_staked:u64,
    //max 10 validators allowed for now
    pub allowed_validators:Vec<Pubkey>          
}
impl Manager{
    pub const MANAGER_SIZE:usize=32 + 8 + 4 + 10*32;
}

#[derive(BorshSerialize,BorshDeserialize)]
pub struct UserPosition{
    pub owner:Pubkey,
    pub stake_acc:Pubkey,
    pub deposited_amount:u64
}
impl UserPosition{
    pub const USER_POSITION_SIZE:usize=72;
}