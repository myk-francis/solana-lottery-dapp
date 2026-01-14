use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash, program::invoke, system_instruction},
};

declare_id!("2X4c4bynxsEGVHMJTtPauRa8RRxbjw7PugorBYw8KWZo");

#[program]
pub mod lottery_contract {
    use super::*;

    pub fn init_master(ctx: Context<Initialize>) -> Result<()> {
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
