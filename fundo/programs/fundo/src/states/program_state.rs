use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub is_initialized :bool,
    pub platform_fee : u64,
    pub campaign_count : u64,
    pub platform_address : Pubkey
}

