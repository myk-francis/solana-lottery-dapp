use anchor_lang::prelude::*;

declare_id!("2X4c4bynxsEGVHMJTtPauRa8RRxbjw7PugorBYw8KWZo");

#[program]
pub mod lottery_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
