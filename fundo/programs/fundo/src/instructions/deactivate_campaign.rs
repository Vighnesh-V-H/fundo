use anchor_lang::prelude::*;

use crate::states::Campaign;
use crate::errors::Errors::*;

pub fn deactivate_campaign(ctx:Context<DeActivateCampaignCtx> , cid:u64)->Result<()>{
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

    campaign.is_active = false;

    Ok(())
}


#[derive(Accounts)]
#[instruction(cid :u64)]
pub struct DeActivateCampaignCtx<'info>{

#[account(mut , seeds=[b"campaign" , cid.to_le_bytes().as_ref()] , bump)]
pub campaign : Account<'info ,Campaign>,



#[account(mut)]
pub creator :Signer<'info>,
pub system_program: Program<'info, System>,

}