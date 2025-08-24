use anchor_lang::prelude::*;

use crate::states::Campaign;
use crate::errors::Errors::*;


pub fn update_campaign(
    ctx:Context<UpdateCampaignCtx> ,
    cid: u64,
    title: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    goal: Option<u64>,
)->Result<()>{
    let campaign = &mut ctx.accounts.campaign;
    let creator = &mut ctx.accounts.creator;

       if campaign.creator != creator.key() {
        return Err(Unauthorized.into());
    }

        if campaign.cid != cid {
        return Err(CampaignNotFound.into());
    }


    if let Some(t) = title {
    if t.len() > 64 {
        return Err(MaxTitleLengthExceeded.into());
    }
    campaign.title = t;
}

if let Some(d) = description {
    if d.len() > 512 {
        return Err(MaxDescriptionLengthExceeded.into());
    }
    campaign.description = d;
}

if let Some(i) = image_url {
    if i.len() > 200 {
        return Err(MaxImageUrlLengthExceeded.into());
    }
    campaign.image_url = i;
}

if let Some(g) = goal {
    if g < 1_000_000_000 {
        return Err(InvalidGoalAmount.into());
    }
    campaign.goal = g;
}



    Ok(())
}


#[derive(Accounts)]
#[instruction(cid : u64)]
pub struct  UpdateCampaignCtx<'info>{

#[account(
    mut , seeds =[b"campaign" , cid.to_le_bytes().as_ref()  ], bump
)]
pub campaign :Account<'info , Campaign>,


    #[account(mut)]
    pub  creator : Signer<'info>,
    pub system_program: Program<'info, System>,
    
}