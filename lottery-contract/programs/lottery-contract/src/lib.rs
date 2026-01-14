use anchor_lang::prelude::*;

mod constants;
use constants::*;

declare_id!("byZRyLeRpJZNEmHA1FgdsfYu77VC1NJ9GBbm9yLANUg");

#[program]
pub mod lottery_contract {
    use super::*;

    pub fn init_master(_ctx: Context<InitMaster>) -> Result<()> {
        Ok(())
    }

    pub fn create_lottery(
        ctx: Context<CreateLottery>,
        ticket_price: u64,
        max_tickets: u32,
    ) -> Result<()> {
        let master = &mut ctx.accounts.master;
        let lottery = &mut ctx.accounts.lottery;

        master.last_id += 1;

        lottery.id = master.last_id;
        lottery.authority = ctx.accounts.authority.key();
        lottery.ticket_price = ticket_price;
        lottery.max_tickets = max_tickets;
        lottery.tickets_sold = 0;
        lottery.prize_pool = 0;
        lottery.is_active = true;
        lottery.winner_id = None;
        lottery.claimed = false;

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;

        require!(lottery.is_active, LotteryError::LotteryNotActive);
        require!(!lottery.claimed, LotteryError::LotteryClaimed);
        require!(
            lottery.tickets_sold < lottery.max_tickets,
            LotteryError::LotterySoldOut
        );

        // Transfer SOL
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &lottery.key(),
            lottery.ticket_price,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                lottery.to_account_info(),
            ],
        )?;

        lottery.tickets_sold += 1;
        lottery.prize_pool += lottery.ticket_price;

        let ticket = &mut ctx.accounts.ticket;
        ticket.id = lottery.tickets_sold;
        ticket.lottery_id = lottery.id;
        ticket.buyer = ctx.accounts.buyer.key();

        Ok(())
    }

    pub fn draw_winner(ctx: Context<PickWinner>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;

    require!(lottery.is_active, LotteryError::LotteryNotActive);
    require!(lottery.tickets_sold > 0, LotteryError::NoTicketsSold);
    require!(lottery.winner_id.is_none(), LotteryError::WinnerAlreadyPicked);

    let clock = Clock::get()?;

    // Pseudo-random number using slot
    let winner = (clock.slot % lottery.tickets_sold as u64) as u32 + 1;

    lottery.winner_id = Some(winner);
    lottery.is_active = false;

    Ok(())
}
}


#[account]
pub struct Master {
    pub last_id: u32,
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

#[account]
pub struct Ticket {
    pub id: u32,
    pub lottery_id: u32,
    pub buyer: Pubkey,
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 4,
        seeds = [MASTER_SEED],
        bump
    )]
    pub master: Account<'info, Master>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 80,
        seeds = [LOTTERY_SEED, &master.last_id.checked_add(1).unwrap().to_le_bytes()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(mut, seeds = [MASTER_SEED], bump)]
    pub master: Account<'info, Master>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(
        init,
        payer = buyer,
        space = 8 + 40,
        seeds = [
            TICKET_SEED,
            lottery.key().as_ref(),
            &lottery.tickets_sold.checked_add(1).unwrap().to_le_bytes()
        ],
        bump
    )]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct PickWinner<'info> {
    #[account(mut, has_one = authority)]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}


#[error_code]
pub enum LotteryError {
    #[msg("The lottery is not active.")]
    LotteryNotActive,
    #[msg("The lottery has already been claimed.")]
    LotteryClaimed,
    #[msg("The lottery is sold out.")]
    LotterySoldOut,
    #[msg("No tickets have been sold for this lottery.")]
    NoTicketsSold,  
    #[msg("A winner has already been picked for this lottery.")]
    WinnerAlreadyPicked,
}