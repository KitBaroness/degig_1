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
pub fn mint_collection_verify_at(
    ctx: Context<AMintCollectionVerifyAt>,
) -> Result<()> {

    {
        //NOTE: created mint collection verifiaction
        ctx.accounts.verify_collection_item(ctx.program_id)?;
    }
    Ok(())
}



#[derive(Accounts)]
pub struct AMintCollectionVerifyAt<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub parent_main_state: Box<Account<'info, MainState>>,

    ///CHECK:
    #[account()]
    pub profile: Box<Account<'info, Mint>>,

    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            profile.key().as_ref(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub profile_metadata: AccountInfo<'info>,
    

    ///CHECK: //PERF:
    #[account(mut)]
    pub collection: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            collection.key().as_ref(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub collection_metadata: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            collection.key().as_ref(),
            "edition".as_bytes(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub collection_edition: AccountInfo<'info>,

    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,

}

impl<'info> AMintCollectionVerifyAt<'info> {
    pub fn verify_collection_item(&mut self, program_id: &Pubkey) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.profile_metadata.to_account_info();
        let main_state = &mut self.parent_main_state;
        let collection = self.collection.to_account_info();
        let collection_metadata = self.collection_metadata.to_account_info();
        let collection_edition = self.collection_edition.to_account_info();
        // let collection_authority_record = self.collection_authority_record.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();

        let ix = Verify {
            collection_metadata: Some(collection_metadata.key()),
            metadata: metadata.key(),
            authority: main_state.key(),
            collection_mint: Some(collection.key()),
            collection_master_edition: Some(collection_edition.key()),
            system_program: system_program.key(),
            sysvar_instructions: sysvar_instructions.key(),
            delegate_record: None,
        }
        .instruction(VerifyInstructionArgs {verification_args:mpl_token_metadata::types::VerificationArgs::CollectionV1});
    
    
        invoke_signed(
            &ix,
            &[
                metadata,
                main_state.to_account_info(),
                collection,
                collection_metadata,
                collection_edition,
                mpl_program,
                system_program,
                sysvar_instructions,
            ],
            &[&[SEED_MAIN_STATE, &[main_state.bump]]],
        )?;
        Ok(())
    }

}

