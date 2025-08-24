use anchor_lang::prelude::*;

pub mod errors;
pub mod constants;
pub mod instructions;
pub mod states;

use errors::*;
use constants::*;
use instructions::*;
use states::*;



declare_id!("DiahVJcBEGSAYhSTTrFLM1GTWiWx9QepJAb5JKraiu6u");

#[program]
pub mod fundo {
    use crate::instructions::InitCtx;

    use super::*;

    pub fn initialize(ctx: Context<InitCtx>) -> Result<()> {
      instructions::init(ctx)
    }

    pub fn create_campaign(
      ctx :Context<CreateCampaignCtx> ,
      title:String,
      description:String,
      image_url:String,
      goal:u64
      ) ->Result<()>{

      instructions::create_campaign(ctx, title, description, image_url, goal)
    }


    pub fn update_campaign(
    ctx:Context<UpdateCampaignCtx> ,
    cid: u64,
    title: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    goal: Option<u64>,
)->Result<()>{

instructions::update_campaign(
   ctx, 
   cid ,
    title,
    description,
    image_url,
    goal,
)

}


}
