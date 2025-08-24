use anchor_lang:: prelude::*;

use crate::constants::ANCHOR_DEFAULT_SPACE; 
use crate::states::ProgramState;
use crate::errors::Errors::*;

pub fn init(ctx:Context<InitCtx>)->Result<()>{
    let account_state = &mut ctx.accounts.program_state;
    let owner = &ctx.accounts.owner;

    if account_state.is_initialized {
        return Err(ProgramAlreadyInitialized.into());
    }

    account_state.is_initialized = true;
    account_state.campaign_count = 0;
    account_state.platform_fee = 7;
    account_state.platform_address = *owner.key;

    Ok(())
}


#[derive(Accounts)]
pub struct InitCtx<'info> {
    #[account(init ,payer=owner , space = ANCHOR_DEFAULT_SPACE + ProgramState::INIT_SPACE , seeds=[b"program_state"] , bump)]
    pub program_state :Account<'info , ProgramState>,

    #[account(mut)]
    owner : Signer<'info>,
    pub system_program: Program<'info, System>,

}