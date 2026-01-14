use anchor_lang::{
    prelude::*,
    solana_program::{ program::invoke, system_instruction::transfer},
};

mod constants;
use constants::MASTER_SEED;
use constants::LOTTERY_SEED;
use constants::TICKET_SEED;

declare_id!("2X4c4bynxsEGVHMJTtPauRa8RRxbjw7PugorBYw8KWZo");

#[program]
pub mod lottery_contract {

    use super::*;

    pub fn init_master(_ctx: Context<InitMaster>) -> Result<()> {
        
        Ok(())
    }

    pub fn create_lottery(ctx: Context<CreateLottery>, ticket_price: u64, max_tickets: u32) -> Result<()>{

        let master = &mut ctx.accounts.master;
        let lottery = &mut ctx.accounts.lottery;

        master.last_id += 1;

        lottery.id = master.last_id;
        lottery.last_ticket_id = 0;
        lottery.authority = *ctx.accounts.authority.key;
        lottery.ticket_price = ticket_price;
        lottery.max_tickets = max_tickets;
        lottery.tickets_sold = 0;
        lottery.prize_pool = 0;
        lottery.is_active = true;
        lottery.winner_id = None;
        lottery.claimed = false;

        msg!("Lottery: {} created by: {} ticket_price: {} max_tickets: {}", lottery.id, lottery.authority, lottery.ticket_price, lottery.max_tickets);

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let ticket = &mut ctx.accounts.ticket;
        let buyer = &ctx.accounts.buyer;
        require!(lottery.is_active, LotteryError::LotteryNotActive);
        require!(lottery.claimed == false, LotteryError::LotteryClaimed);
        require!(lottery.tickets_sold < lottery.max_tickets, LotteryError::LotterySoldOut);
        // Transfer SOL from buyer to lottery prize pool
        invoke(
            &transfer(
                &buyer.key(),
                &lottery.key(),
                lottery.ticket_price
            ),
            &[buyer.to_account_info(), lottery.to_account_info()]
        )?;

        lottery.tickets_sold += 1;
        lottery.prize_pool += lottery.ticket_price;
        lottery.last_ticket_id += 1;

        ticket.id = lottery.last_ticket_id;
        ticket.lottery_id = lottery.id;
        ticket.buyer = *buyer.key;

        msg!("Ticket: {} bought by: {} for lottery: {}", ticket.id, ticket.buyer, lottery.id);

        Ok(())
    }

    pub fn draw_winner(ctx: Context<PickWinner>, _lottery_id: u32) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        require!(lottery.is_active, LotteryError::LotteryNotActive);
        require!(lottery.tickets_sold > 0, LotteryError::NoTicketsSold);
        require!(lottery.winner_id.is_none(), LotteryError::WinnerAlreadyPicked);

        // Pick a random winner
        let winner_id = lottery.last_ticket_id;
        lottery.winner_id = Some(winner_id);

        msg!("Winner picked for lottery: {}", lottery.id);


        Ok(())
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
    #[account(mut, seeds = [MASTER_SEED], bump)]
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
    pub last_ticket_id: u32,
    pub prize_pool: u64,
    pub is_active: bool,
    pub winner_id: Option<u32>,
    pub claimed: bool,
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct BuyTicket<'info> {
    #[account(mut, seeds = [LOTTERY_SEED, &[lottery_id as u8]], bump)]
    pub lottery: Account<'info, Lottery>,
    #[account(
        init,
        payer = buyer,
        space = 8 + 8 + 4 + 32,
        seeds = [TICKET_SEED, lottery.key().as_ref(), &[lottery.last_ticket_id as u8 + 1]],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Ticket {
    pub id: u32,
    pub lottery_id: u32,
    pub buyer: Pubkey,
}

#[error_code]
pub enum LotteryError {
    #[msg("The lottery is not active.")]
    LotteryNotActive,
    #[msg("The lottery is sold out.")]
    LotterySoldOut,
    #[msg("The lottery is claimed.")]
    LotteryClaimed,
    #[msg("No tickets have been sold for this lottery.")]
    NoTicketsSold,
    #[msg("A winner has already been picked for this lottery.")]
    WinnerAlreadyPicked,
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct PickWinner<'info> {
    #[account(mut, seeds = [LOTTERY_SEED, &[lottery_id as u8]], has_one = authority, bump)]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}