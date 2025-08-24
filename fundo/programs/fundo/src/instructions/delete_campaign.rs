use anchor_lang::prelude::*;

use crate::states::Campaign;
use crate::errors::Errors::*;

pub fn delete_campaign(ctx:Context<DeleteCampaignCtx> , cid:u64)->Result<()>{
let campaign = &mut ctx.accounts.campaign;
let creator = &mut ctx.accounts.creator;

if campaign.creator != creator.key(){
    return Err(Unauthorized.into());
}
if campaign.cid != cid {
    return Err(CampaignNotFound.into());
}

    if !campaign.is_active {
        return Err(InactiveCampaign.into());
    }

  
    Ok(())
}


#[derive(Accounts)]
#[instruction(cid :u64)]
pub struct DeleteCampaignCtx<'info>{

#[account(mut ,close=creator ,  seeds=[b"campaign" , cid.to_le_bytes().as_ref()] , bump)]
pub campaign : Account<'info ,Campaign>,



#[account(mut)]
pub creator :Signer<'info>,
pub system_program: Program<'info, System>,

}