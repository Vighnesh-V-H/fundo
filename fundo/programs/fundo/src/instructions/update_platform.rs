use crate::constants::seeds;
use crate::errors::Errors::*;
use crate::states::ProgramState;
use anchor_lang::prelude::*;

pub fn update_platform_fee(
    ctx: Context<UpdatePlatformFeeCtx>,
    new_platform_fee: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.program_state;
    let updater = &ctx.accounts.updater;

    if updater.key() != state.platform_address {
        return Err(Unauthorized.into());
    }

    if !(1..=10).contains(&new_platform_fee) {
        return Err(InvalidPlatformFee.into());
    }

    state.platform_fee = new_platform_fee;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePlatformFeeCtx<'info> {
    #[account(mut)]
    pub updater: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::PROGRAM_STATE],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
}