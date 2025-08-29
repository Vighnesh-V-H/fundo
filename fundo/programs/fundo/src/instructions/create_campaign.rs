use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DEFAULT_SPACE;
 use crate::states::{Campaign, ProgramState};
 use crate::errors::Errors::*;


pub fn create_campaign(
    ctx :Context<CreateCampaignCtx>, 
    title:String,
    description:String,
    image_url :String,
    goal:u64
)->Result<()>{
    let program_state = &mut ctx.accounts.program_state;
    let campaign = &mut ctx.accounts.campaign;

    if title.len() > 64 {
        return Err(MaxTitleLengthExceeded.into());
    }
    if description.len() > 512 {
        return Err(MaxDescriptionLengthExceeded.into());
    }
    if image_url.len() > 200 {
        return Err(MaxImageUrlLengthExceeded.into());
    }
    if goal < 1_000_000_000 {
        return Err(InvalidGoalAmount.into());
    }
    program_state.campaign_count+=1;

    campaign.cid = program_state.campaign_count;
    campaign.creator = *ctx.accounts.creator.key;
    campaign.title = title;
    campaign.description = description;
    campaign.goal = goal;
    campaign.image_url = image_url;
    
    campaign.amount_raised = 0;
    campaign.donors = 0;
    campaign.withdrawals = 0;
    campaign.timestamp = Clock::get()?.unix_timestamp as u64;
    campaign.is_active = true;

    Ok(())
}


#[derive(Accounts)]
pub struct CreateCampaignCtx<'info> {

    #[account(mut)]
    pub program_state : Account<'info , ProgramState>,

    #[account(
        init,
        payer = creator,
        space = ANCHOR_DEFAULT_SPACE + Campaign::INIT_SPACE,
        seeds = [
            b"campaign",
            (program_state.campaign_count + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}