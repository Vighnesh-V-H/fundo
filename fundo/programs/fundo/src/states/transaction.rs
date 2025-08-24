use anchor_lang::prelude::*;

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub owner: Pubkey,
    pub cid: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub is_credit: bool,
}