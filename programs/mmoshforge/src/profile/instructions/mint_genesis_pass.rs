use std::collections::HashMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::{
        ApproveCollectionAuthority, Create, CreateBuilder, Verify, VerifyInstructionArgs, VerifySizedCollectionItem
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
        SEED_ACTIVATION_TOKEN_STATE, SEED_COLLECTION_STATE, SEED_MAIN_STATE, SEED_PROFILE_STATE, SEED_VAULT, TOTAL_SELLER_BASIS_POINTS
    },
    error::MyError,
    other_states::LineageInfo,
    profile_state::ProfileState,
    utils::{
        get_vault_pda, init_ata_if_needed, transfer_tokens,
        verify_collection_item_by_main,
    }, CollectionState, MainStateInput,
};

/// This is the function which used to create instruction for mint gensis pass nft
pub fn mint_genesis_pass(
    ctx: Context<AMintPassByAdmin>,
    name: Box<String>,
    symbol: Box<String>,
    uri_hash: Box<String>,
    input: MainStateInput,
) -> Result<()> {
    let name = *name;
    let symbol = *symbol;
    let uri_hash = *uri_hash;
    {
        let main_state = &mut ctx.accounts.main_state;
        let user = ctx.accounts.user.to_account_info();
        input.set_value(main_state);
        main_state.owner = user.key();
        main_state._bump = ctx.bumps.main_state;

        //NOTE: setup and validation
        let main_state = &mut ctx.accounts.main_state;
        let profile_state = &mut ctx.accounts.profile_state;
        let collection_state = &mut ctx.accounts.collection_state;

        //state changes
        let profile = ctx.accounts.profile.key();
        profile_state.mint = profile;
        profile_state.lineage.creator = ctx.accounts.user.key();
        profile_state.lineage.parent = profile;
        profile_state.lineage.grand_parent = profile;
        profile_state.lineage.great_grand_parent = profile;
        profile_state.lineage.ggreat_grand_parent = profile;

        profile_state.lineage.generation = 1;
        profile_state.lineage.total_child = 0;

        collection_state.genesis_profile = ctx.accounts.profile.key();

        //TODO: update some main state if fiels are avaible (may be in future)
        main_state.total_minted_profile += 1;
        main_state.genesis_profile = ctx.accounts.profile.key();
    }

    {
        //NOTE: minting
        ctx.accounts.mint(name, symbol, uri_hash)?;
    }
    {
        //NOTE: created mint collection verifiaction
        ctx.accounts.verify_collection_item(ctx.program_id)?;
    }

    Ok(())
}



/// Struct used to create context while preparing instruction for create new gensis pass nft
#[derive(Accounts)]
#[instruction(
    name: Box<String>,
    symbol: Box<String>,
    uri: Box<String>,
)]
pub struct AMintPassByAdmin<'info> {
    /// owner publickey and mandatory signer while pushing the instruction to solana
    #[account(mut)]
    pub user: Signer<'info>,

    /// mpl program public key
    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    /// token program public key
    pub token_program: Program<'info, Token>,
    /// associated token program public key
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// sytem program public key
    pub system_program: Program<'info, System>,

    /// project public key
    ///CHECK:
    #[account(
        mut,
        mint::decimals = 0,
        mint::authority = user,
        mint::freeze_authority = user,
    )]
    pub profile: Box<Account<'info, Mint>>,

    /// load project mainstate account with seed for update purpose
    #[account(
        init,
        payer = user,
        seeds = [SEED_MAIN_STATE, profile.key().as_ref()],
        bump,
        space = 8 + MainState::MAX_SIZE, 
    )]
    pub main_state: Box<Account<'info, MainState>>,

    /// load mainstate account with seed for update purpose
    #[account(
        mut,
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub parent_main_state: Box<Account<'info, MainState>>,

    /// signer token assoicated account public key
    #[account(
        mut,
        associated_token::mint = profile,
        associated_token::authority = user,
    )]
    pub user_profile_ata: Box<Account<'info, TokenAccount>>,

    /// load profile state account public key 
    #[account(
        init,
        payer =  user,
        seeds = [SEED_PROFILE_STATE, profile.key().as_ref()],
        bump,
        space= 8 + ProfileState::MAX_SIZE
    )]
    pub profile_state: Box<Account<'info, ProfileState>>,

    /// load profile metadata account public key 
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

    /// load profile edition account public key 
    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            profile.key().as_ref(),
            "edition".as_bytes(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub profile_edition: AccountInfo<'info>,

    /// collection public key
    ///CHECK: //PERF:
    #[account(mut)]
    pub collection: AccountInfo<'info>,

    /// collection state account from publickey 
    #[account(
        mut,
        seeds = [SEED_COLLECTION_STATE, collection.key().as_ref()],
        bump,
    )]
    pub collection_state: Account<'info, CollectionState>,

    /// collection metdata account public key 
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

    /// collection edition account public key 
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

    /// system var instruction public key
    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>
}

impl<'info> AMintPassByAdmin<'info> {
    pub fn mint(&mut self, name: String, symbol: String, uri_hash: String) -> Result<()> {
        let mint = self.profile.to_account_info();
        let user = self.user.to_account_info();
        let user_profile_ata = self.user_profile_ata.to_account_info();
        // let user_profile_ata = self.user_profile_ata.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.profile_metadata.to_account_info();
        let edition = self.profile_edition.to_account_info();
        // let associated_token_program = self.associated_token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();
        let main_state = &mut self.main_state;

        //mint a token
        // let cpi_acounts = MintTo {
        //     mint: mint.to_account_info(),
        //     to: user_profile_ata,
        //     authority: user.to_account_info(),
        // };
        // token::mint_to(
        //     CpiContext::new(token_program.to_account_info(), cpi_acounts),
        //     1,
        // )?;

        // Creators Setup for royalty
        let asset_data = CreateArgs::V1 {
            name,
            symbol,
            uri: uri_hash,
            collection: Some(mpl_token_metadata::types::Collection {
                verified: false,
                key: self.collection.key(),
            }),
            uses: None,
            creators: Some(vec![Creator {
                verified: false,
                share: 100,
                address: self.user.key(),
            }]),
            // creators: None,
            collection_details: Some(mpl_token_metadata::types::CollectionDetails::V1 { size: 0 }),
            is_mutable: true, //NOTE: may be for testing
            rule_set: None,
            token_standard: mpl_token_metadata::types::TokenStandard::NonFungible,
            primary_sale_happened: true,
            seller_fee_basis_points: main_state.seller_fee_basis_points,
            decimals: Some(0),
            print_supply: Some(mpl_token_metadata::types::PrintSupply::Zero),
        };


        let ix = CreateBuilder::new()
        .metadata(metadata.key())
        .master_edition(Some(edition.key()))
        .mint( mint.key(), true)
        .authority(user.key())
        .payer(user.key())
        .update_authority(main_state.key(),true)
        .spl_token_program(Some(token_program.key()))
        .sysvar_instructions(sysvar_instructions.key())
        .system_program(system_program.key())
        .create_args(asset_data)
        .instruction();


        invoke_signed(
            &ix,
            &[
                mint.clone(),
                user,
                main_state.to_account_info(),
                metadata,
                edition,
                mpl_program,
                token_program,
                system_program,
                sysvar_instructions,
            ],
            &[
                &[SEED_MAIN_STATE, mint.key().as_ref(), &[self.main_state._bump]],
            ],
        )?;

        Ok(())
    }

    pub fn verify_collection_item(&mut self, program_id: &Pubkey) -> Result<()> {
        let mint = self.profile.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.profile_metadata.to_account_info();
        let parent_main_state = &mut self.parent_main_state;
        let collection = self.collection.to_account_info();
        let collection_metadata = self.collection_metadata.to_account_info();
        let collection_edition = self.collection_edition.to_account_info();
        // let collection_authority_record = self.collection_authority_record.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();

        let ix = Verify {
            collection_metadata: Some(collection_metadata.key()),
            metadata: metadata.key(),
            authority: parent_main_state.key(),
            collection_mint: Some(collection.key()),
            collection_master_edition: Some(collection_edition.key()),
            system_program: system_program.key(),
            sysvar_instructions: sysvar_instructions.key(),
            // delegate_record: Some(collection_authority_record.key()),
            delegate_record: None,
        }
        .instruction(VerifyInstructionArgs {verification_args:mpl_token_metadata::types::VerificationArgs::CollectionV1});
    
        invoke_signed(
            &ix,
            &[
                metadata,
                parent_main_state.to_account_info(),
                collection,
                collection_metadata,
                collection_edition,
                mpl_program,
                system_program,
                // collection_authority_record,
                sysvar_instructions,
            ],
            &[&[SEED_MAIN_STATE, &[parent_main_state._bump]]],
        )?;
        Ok(())
    }

}


