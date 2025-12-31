use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize,BorshDeserialize)]
pub struct Manager{
    pub admin:Pubkey,
    pub total_staked:u64,
    pub allowed_validators:Vec<Pubkey>
}
impl Manager{
    pub const MANAGER_SIZE:usize=32 + 8 + 4 + 10*32;
}