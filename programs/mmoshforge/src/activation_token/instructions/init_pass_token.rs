use std::env::args;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::{Create, CreateBuilder, Verify, VerifyInstructionArgs}, types::Creator, types::CreateArgs, ID as MPL_ID
};
use solana_program::program::{invoke, invoke_signed};

use crate::{
    _main::MainState,
    activation_token::ActivationTokenState,
    constants::{SEED_ACTIVATION_TOKEN_STATE, SEED_MAIN_STATE, SEED_PROFILE_STATE},
    error::MyError,
    other_states::LineageInfo,
    profile::profile_state::ProfileState,
    utils::{init_ata_if_needed, verify_collection_item_by_main},
};

/// This is the function which used to create instruction for initalize pass token
pub fn init_pass_token(
    ctx: Context<AInitPassToken>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    {
        //NOTE: setup and validation
        let main_state = &mut ctx.accounts.main_state;
        let activation_token_state = &mut ctx.accounts.activation_token_state;
        let profile_state = &mut ctx.accounts.profile_state;
        let profile_metadata = ctx.accounts.profile_metadata.to_account_info();

        profile_state.activation_token = Some(ctx.accounts.activation_token.key());
        activation_token_state.parent_profile = ctx.accounts.profile.key();
        activation_token_state.creator = ctx.accounts.user.key();
        //TODO: update some main state if fiels are avaible (may be in future)
    }
    {
        //NOTE: minting
        ctx.accounts.init_token(name, symbol, uri)?;
    }
    {
        ctx.accounts.verify_collection_item(ctx.program_id)?;
    }
    Ok(())
}

/// Struct used to create context while preparing instruction for initialize pass token
#[derive(Accounts)]
pub struct AInitPassToken<'info> {
    /// owner publickey and mandatory signer while pushing the instruction to solana
    #[account(mut)]
    pub user: Signer<'info>,

    /// signer token assoicated account public key
    #[account(
        mut,
        token::mint = profile,
        token::authority = user,
        constraint = user_profile_ata.amount == 1 @ MyError::OnlyProfileHolderAllow,
    )]
    pub user_profile_ata: Box<Account<'info, TokenAccount>>,

    /// signer pass assoicated account public key
    ///CHECK:
    #[account(mut)]
    pub user_activation_token_ata: AccountInfo<'info>,

    /// project public key
    ///CHECK:
    pub project: Box<Account<'info, Mint>>,

    /// load project mainstate account public key
    #[account(
        mut,
        seeds = [SEED_MAIN_STATE, project.key().as_ref()],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    /// load mainstate account public key
    #[account(
        mut,
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub parent_main_state: Box<Account<'info, MainState>>,

    /// activation token public key
    ///CHECK:
    #[account(mut, signer)]
    pub activation_token: AccountInfo<'info>,


    /// load activiation state account public key 
    #[account(
        init,
        payer = user,
        seeds = [SEED_ACTIVATION_TOKEN_STATE,activation_token.key().as_ref()],
        bump,
        space = 8 + ActivationTokenState::MAX_SIZE,
    )]
    pub activation_token_state: Box<Account<'info, ActivationTokenState>>,

    /// load activiation metadata account public key 
    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            activation_token.key().as_ref(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub activation_token_metadata: AccountInfo<'info>,

    /// profile nft public key
    #[account()]
    pub profile: Box<Account<'info, Mint>>,

    /// load profile state account public key 
    #[account(
        mut,
        seeds = [SEED_PROFILE_STATE,profile.key().as_ref()],
        bump,
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

    /// load profile collection authority public key 
    ///CHECK:
    #[account(
        mut,
        seeds = [
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            profile.key().as_ref(),
            "collection_authority".as_bytes(),
            main_state.key().as_ref(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub profile_collection_authority_record: AccountInfo<'info>,

    /// collection nft to make nft as verified nft
    ///CHECK:
    #[account(mut)]
    pub parent_collection: AccountInfo<'info>,

    /// collection metadata publickey to make nft as verified nft
    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            parent_collection.key().as_ref(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub parent_collection_metadata: AccountInfo<'info>,

    /// load parent collection edition account public key 
    ///CHECK:
    #[account(
        mut,
        seeds=[
            "metadata".as_bytes(),
            MPL_ID.as_ref(),
            parent_collection.key().as_ref(),
            "edition".as_bytes(),
        ],
        bump,
        seeds::program = MPL_ID
    )]
    pub parent_collection_edition: AccountInfo<'info>,
    
    /// system var instruction public key
    ///CHECK:
    #[account()]
    pub sysvar_instructions: AccountInfo<'info>,

    /// mpl program public key
    ///CHECK:
    #[account(address = MPL_ID)]
    pub mpl_program: AccountInfo<'info>,
    /// associated token program public key
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// token program public key
    pub token_program: Program<'info, Token>,
    /// sytem program public key
    pub system_program: Program<'info, System>,
}

impl<'info> AInitPassToken<'info> {
    pub fn init_token(
        &mut self,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let mint = self.activation_token.to_account_info();
        let user = self.user.to_account_info();
        let user_activation_token_ata = self.user_activation_token_ata.to_account_info();
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.activation_token_metadata.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let ata_program = self.associated_token_program.to_account_info();
        let sysvar_instructions = self.sysvar_instructions.to_account_info();
        let main_state = &mut self.main_state;

        let asset_data = CreateArgs::V1 {
            name,
            symbol,
            uri,
            collection: Some(mpl_token_metadata::types::Collection {
                verified: false,
                key: self.parent_collection.key(),
            }),
            uses: None,
            creators: Some(vec![
                Creator {
                    address: main_state.key(),
                    verified: true,
                    share: 0,
                },
                Creator {
                    address: self.profile.key(),
                    verified: false,
                    share: 0,
                },
                Creator {
                    address: user.key(),
                    verified: false,
                    share: 100,
                },
            ]),
            collection_details: None,
            is_mutable: true, //NOTE: may be for testing
            rule_set: None,
            token_standard: mpl_token_metadata::types::TokenStandard::FungibleAsset,
            primary_sale_happened: false,
            seller_fee_basis_points: 100,
            decimals: Some(0),
            print_supply: None,
        };

        let ix = CreateBuilder::new()
        .metadata(metadata.key())
        .master_edition(None)
        .mint( mint.key(), true)
        .authority(main_state.key())
        .payer(user.key())
        .update_authority(main_state.key(),true)
        .spl_token_program(Some(token_program.key()))
        .sysvar_instructions(sysvar_instructions.key())
        .create_args(asset_data)
        .instruction();

        invoke_signed(
            &ix,
            &[
                mint.to_account_info(),
                user.to_account_info(),
                user_activation_token_ata,
                main_state.to_account_info(),
                metadata,
                mpl_program,
                token_program.to_account_info(),
                system_program.to_account_info(),
                sysvar_instructions,
            ],
            &[&[SEED_MAIN_STATE, self.project.key().as_ref(), &[main_state._bump]]],
        )?;
        
        Ok(())
    }

    pub fn verify_collection_item(&mut self, program_id: &Pubkey) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let token_program = self.token_program.to_account_info();
        let mpl_program = self.mpl_program.to_account_info();
        let metadata = self.activation_token_metadata.to_account_info();
        let main_state = &mut self.parent_main_state;
        let collection = self.parent_collection.to_account_info();
        let collection_metadata = self.parent_collection_metadata.to_account_info();
        let collection_edition = self.parent_collection_edition.to_account_info();
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
            // delegate_record: Some(collection_authority_record.key()),
            delegate_record: None,
        }
        .instruction(VerifyInstructionArgs{
            verification_args: mpl_token_metadata::types::VerificationArgs::CollectionV1
        });
    
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
                // collection_authority_record,
                sysvar_instructions,
            ],
            &[&[SEED_MAIN_STATE, &[main_state._bump]]],
        )?;
        
        Ok(())
    }

}
