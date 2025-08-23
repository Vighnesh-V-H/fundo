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
}
