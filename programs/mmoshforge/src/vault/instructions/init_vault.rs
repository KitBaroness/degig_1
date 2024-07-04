use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{constants::SEED_VAULT, vault::VaultState};

pub fn init_vault(
    ctx: Context<InitVault>,
    lock_date: u64,
) -> Result<()> {
    {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.seller.key();
        vault.lock_date = lock_date;
        vault.mint = ctx.accounts.mint.key();
        vault._bump = ctx.bumps.vault;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        mut,
    )]
    pub seller: Signer<'info>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [SEED_VAULT, seller.key().as_ref() ,mint.key().as_ref()],
        bump,
        payer = seller,
        space= 8 + VaultState::MAX_SIZE
    )]
    pub vault: Box<Account<'info, VaultState>>,
    #[account(
        init,
        payer = seller, // minter, the one who is minting
        associated_token::mint = mint, // mint of the token
        associated_token::authority = vault //authority that should be a PDA account
    )]
    token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}