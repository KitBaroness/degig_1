use std::collections::HashMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::{
        ApproveCollectionAuthority, Create, CreateBuilder, Verify, VerifyInstructionArgs
    },
    types::{CreateArgs, Creator},
    ID as MPL_ID,
};
use solana_address_lookup_table_program::{
    instruction::{create_lookup_table, extend_lookup_table, freeze_lookup_table},
    ID as ADDRESS_LOOKUP_TABLE_PROGRAM,
};
use solana_program::program::{invoke, invoke_signed};

use crate::{
    _main::MainState,
    activation_token::ActivationTokenState,
    constants::{
        SEED_ACTIVATION_TOKEN_STATE, SEED_MAIN_STATE, SEED_PROFILE_STATE, SEED_VAULT,
        TOTAL_SELLER_BASIS_POINTS,
    },
    error::MyError,
    other_states::LineageInfo,
    profile_state::ProfileState,
    utils::{
        get_vault_pda, init_ata_if_needed, transfer_tokens,
        verify_collection_item_by_main,
    },
};

///MINT FakeID by activation_token
pub fn mint_burn_at(
    ctx: Context<AMintBurnByAt>,
) -> Result<()> {
    {
        ctx.accounts.burn_activation_token(ctx.program_id)?;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct AMintBurnByAt<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    ///CHECK:
    #[account(
        mut,
        token::mint = activation_token,
        token::authority = user,
        constraint = user_activation_token_ata.amount >= 1 @ MyError::ActivationTokenNotFound,
    )]
    pub user_activation_token_ata: Box<Account<'info, TokenAccount>>,


    #[account(mut)]
    pub activation_token: Box<Account<'info, Mint>>,

    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,
}

impl<'info> AMintBurnByAt<'info> {
    pub fn burn_activation_token(&mut self, program_id: &Pubkey) -> Result<()> {
        let mint = self.activation_token.to_account_info();
        let user = self.user.to_account_info();
        let user_activation_token_ata = self.user_activation_token_ata.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();

        let cpi_accounts = Burn {
            mint,
            from: user_activation_token_ata,
            authority: user,
        };

        token::burn(CpiContext::new(token_program, cpi_accounts), 1)?;
        Ok(())
    }

}

