use anchor_lang::prelude::*;

declare_id!("Eu6oXy1qR4xBfxNphKzeBMA41tT3EycJRs7wVNCNSnDN");

#[program]
pub mod blocktrips_sc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
