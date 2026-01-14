use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, program::invoke, system_instruction::transfer},
};

mod constants;
use constants::MASTER_SEED;
use constants::LOTTERY_SEED;

declare_id!("2X4c4bynxsEGVHMJTtPauRa8RRxbjw7PugorBYw8KWZo");

#[program]
pub mod lottery_contract {
    use super::*;

    pub fn init_master(_ctx: Context<InitMaster>) -> Result<()> {
        
        Ok(())
    }

    pub fn create_lottery(ctx: Context<CreateLottery>) -> Result<()>{

    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(
        init,
        payer = payer,
        space = 4 + 8,
        seeds = [MASTER_SEED],
        bump
    )]
    pub master: Account<'info, Master>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}


#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(
        init,
        payer = authority,
        space = 106,
        seeds = [LOTTERY_SEED, &[master.last_id as u8 + 1]],
        bump
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub master: Account<'info, Master>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Lottery {
    pub id: u32,
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub max_tickets: u32,
    pub tickets_sold: u32,
    pub prize_pool: u64,
    pub is_active: bool,
    pub winner_id: Option<u32>,
    pub claimed: bool,
}