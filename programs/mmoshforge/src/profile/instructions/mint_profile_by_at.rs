use std::collections::HashMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::{
        ApproveCollectionAuthority, Create, CreateBuilder, Verify
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

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct MintProfileByAtInput {
    pub name: String,
    pub symbol: String,
    // pub uri: String,
    pub uri_hash: String,
}

///MINT FakeID by activation_token
pub fn mint_profile_by_at(
    ctx: Context<AMintProfileByAt>,
    name: Box<String>,
    symbol: Box<String>,
    uri_hash: Box<String>,
) -> Result<()> {
    let name = *name;
    let symbol = *symbol;
    let uri_hash = *uri_hash;
    {
        let user = ctx.accounts.user.to_account_info();
        let main_state = &mut ctx.accounts.main_state;
        let profile_state = &mut ctx.accounts.profile_state;
        let parent_profile_state = &mut ctx.accounts.parent_profile_state;
        // let parent_profile_metadata = ctx.accounts.parent_profile_metadata.to_account_info();
        let token_program = ctx.accounts.token_program.to_account_info();


        //state changes
        profile_state.mint = ctx.accounts.profile.key();
        profile_state.lineage.creator = ctx.accounts.user.key();
        profile_state.lineage.parent = parent_profile_state.mint;
        profile_state.lineage.grand_parent = parent_profile_state.lineage.parent;
        profile_state.lineage.great_grand_parent = parent_profile_state.lineage.grand_parent;
        profile_state.lineage.ggreat_grand_parent = parent_profile_state.lineage.great_grand_parent;
        profile_state.lineage.generation = parent_profile_state.lineage.generation + 1;
        parent_profile_state.lineage.total_child += 1;
    }
    {
        //NOTE: minting
        ctx.accounts.mint(name, symbol, uri_hash)?;
    }
    Ok(())
}



#[derive(Accounts)]
#[instruction(
    name: Box<String>,
    symbol: Box<String>,
    uri: Box<String>,
)]
pub struct AMintProfileByAt<'info> {
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
    pub main_state: Box<Account<'info, MainState>>,

    ///CHECK:
    #[account(
        init,
        signer,
        payer = user,
        mint::decimals = 0,
        mint::authority = user,
        mint::freeze_authority = user,
    )]
    pub profile: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = user,
        associated_token::mint = profile,
        associated_token::authority = user,
    )]
    pub user_profile_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer =  user,
        seeds = [SEED_PROFILE_STATE, profile.key().as_ref()],
        bump,
        space= 8 + ProfileState::MAX_SIZE
    )]
    pub profile_state: Box<Account<'info, ProfileState>>,

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

    #[account(
        mut,
        seeds = [SEED_PROFILE_STATE, parent_profile.key().as_ref()],
        bump,
    )]
    pub parent_profile_state: Box<Account<'info, ProfileState>>,

    ///CHECK: //PERF:
    #[account(mut)]
    pub collection: AccountInfo<'info>,

    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,

    //NOTE: profile minting cost distribution account
    // #[account(address = activation_token_state.parent_profile @ MyError::ProfileIdMissMatch)]
    pub parent_profile: Box<Account<'info, Mint>>,


}

impl<'info> AMintProfileByAt<'info> {
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
        let cpi_acounts = MintTo {
            mint: mint.to_account_info(),
            to: user_profile_ata,
            authority: user.to_account_info(),
        };
        token::mint_to(
            CpiContext::new(token_program.to_account_info(), cpi_acounts),
            1,
        )?;

        // Creators Setup for royalty
        let trading_price_distribution = main_state.minting_cost_distribution;
        let seller_fee_basis_points = TOTAL_SELLER_BASIS_POINTS;
        let creators = vec![
            //NOTE: currently not royalty info for creator
            Creator {
                address: user.key(),
                verified: false,
                share: 0,
            },
            Creator {
                address: get_vault_pda(&self.profile_state.lineage.parent).0,
                verified: false,
                share: (trading_price_distribution.parent as u64 * 100u64
                    / seller_fee_basis_points as u64) as u8,
            },
            Creator {
                address: get_vault_pda(&self.profile_state.lineage.grand_parent).0,
                verified: false,
                share: (trading_price_distribution.grand_parent as u64 * 100u64
                    / seller_fee_basis_points as u64) as u8,
            },
            Creator {
                address: get_vault_pda(&self.profile_state.lineage.great_grand_parent).0,
                verified: false,
                share: (trading_price_distribution.great_grand_parent as u64 * 100u64
                    / seller_fee_basis_points as u64) as u8,
            },
            Creator {
                address: get_vault_pda(&main_state.genesis_profile).0,
                verified: false,
                share: (trading_price_distribution.genesis as u64 * 100u64
                    / seller_fee_basis_points as u64) as u8,
            },
        ];

        let mut unique_creators = HashMap::<Pubkey, Creator>::new();
        for creator in creators.into_iter() {
            let res = unique_creators.get_mut(&creator.address);
            if let Some(value) = res {
                value.share += creator.share;
            } else {
                unique_creators.insert(creator.address, creator);
            }
        }

        let creators = Some(
            unique_creators
                .into_iter()
                .map(|(k, v)| v)
                .collect::<Vec<_>>(),
        );

        let entry_point = "https://shdw-drive.genesysgo.net/FuBjTTmQuqM7pGR2gFsaiBxDmdj8ExP5fzNwnZyE2PgC/".to_string();
        let uri = format!("{}{}", entry_point, uri_hash);

        let asset_data = CreateArgs::V1 {
            name,
            symbol,
            uri,
            collection: Some(mpl_token_metadata::types::Collection {
                verified: false,
                key: self.collection.key(),
            }),
            uses: None,
            creators,
            // creators: None,
            collection_details: Some(mpl_token_metadata::types::CollectionDetails::V1 { size: 0 }),
            is_mutable: true, //NOTE: may be for testing
            rule_set: None,
            token_standard: mpl_token_metadata::types::TokenStandard::NonFungible,
            primary_sale_happened: true,
            seller_fee_basis_points, //EX: 20% (80% goes to seller)
            decimals: Some(0),
            print_supply: Some(mpl_token_metadata::types::PrintSupply::Zero),
        };

        let ix = CreateBuilder::new()
        .metadata(metadata.key())
        .master_edition(Some(edition.key()))
        .mint( mint.key(), false)
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
                mint,
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
                &[SEED_MAIN_STATE, &[self.main_state.bump]],
            ],
        )?;

        Ok(())
    }

}
