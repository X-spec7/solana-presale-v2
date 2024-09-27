use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{UserInfo, PresaleInfo};
use crate::constants::{USER_SEED, PRESALE_SEED, PRESALE_VAULT};
use crate::errors::PresaleError;

pub fn require_refund(ctx: Context<Refund>) -> Result<()> {
    let user_info = &mut ctx.accounts.user_info;
    let presale_info = &mut ctx.accounts.presale_info;
    let presale_vault = &ctx.accounts.presale_vault;

    // Check if the user has any funds to refund
    if user_info.buy_quote_amount_in_lamports == 0 {
        return Err(PresaleError::NoFundsToRefund.into());
    }

    // Check if the presale is still ongoing
    let clock = Clock::get()?;
    if clock.unix_timestamp * 1000 < presale_info.end_time {
        return Err(PresaleError::PresaleStillOngoing.into());
    }

    // Check if the softcap was not reached
    if presale_info.is_soft_capped {
        return Err(PresaleError::SoftcapReached.into());
    }

    let refund_amount = user_info.buy_quote_amount_in_lamports;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.presale_vault.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
            }
        ),
        refund_amount
    )?;

    // Update user info
    user_info.buy_quote_amount_in_lamports = 0;
    user_info.buy_token_amount = 0;

    // Update presale info
    presale_info.sold_token_amount -= user_info.buy_token_amount;

    msg!("Refund of {} lamports processed successfully", refund_amount);

    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED],
        bump
    )]
    pub presale_info: Account<'info, PresaleInfo>,

    #[account(
        mut,
        seeds = [USER_SEED, user.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        seeds = [PRESALE_VAULT],
        bump
    )]
    pub presale_vault: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
