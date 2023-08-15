use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use crate::error::MyError;
use crate::other_states::{MintingRoyaltyInfo, TradingRoyaltyInfo};

#[account]
pub struct MainState {
    pub owner: Pubkey,
    //It's genesis NFT(First GENESIS NFT)
    // pub genesis_fake_id: Pubkey,
    // pub activation_token_collection_id: Pubkey,
    pub usdc_mint: Pubkey,
    pub profile_minting_usdc_price: u64,
    pub royalty_for_minting: MintingRoyaltyInfo,
    pub royalty_for_trading: TradingRoyaltyInfo,
    pub seller_fee_basis_points: u16, //NOTE: may be later change
    pub _bump: u8,
    pub total_minted_profile: u64,
}

impl MainState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
    // pub fn verify_profile<'info>(&self, metadata_account_info: &'info AccountInfo) -> Result<()> {
    //     let metadata =
    //         Metadata::from_account_info(metadata_account_info).map_err(|_| MyError::UnknownNft)?;
    //     let collection_info = metadata.collection.ok_or_else(|| MyError::UnknownNft)?;
    //     Ok(())
    // }

    pub fn verify_activation_token<'info>(
        &self,
        metadata_account_info: &'info AccountInfo,
    ) -> Result<()> {
        let metadata =
            Metadata::from_account_info(metadata_account_info).map_err(|_| MyError::UnknownNft)?;
        let collection_info = metadata.collection.ok_or_else(|| MyError::UnknownNft)?;
        // require!(
        //     collection_info.key == self.activation_token_collection_id && collection_info.verified,
        //     MyError::UnknownNft
        // );
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct MainStateInput {
    pub profile_minting_usdc_price: u64,
    pub usdc_mint: Pubkey,
    pub royalty_for_minting: MintingRoyaltyInfo,
    pub royalty_for_trading: TradingRoyaltyInfo,
    // pub activation_token_collection_id: Pubkey,
}

impl MainStateInput {
    pub fn set_value(&self, mut state: &mut MainState) {
        // state.activation_token_collection_id = self.activation_token_collection_id;
        state.usdc_mint = self.usdc_mint;
        state.royalty_for_minting = self.royalty_for_minting;
        state.royalty_for_trading = self.royalty_for_trading;
        state.profile_minting_usdc_price = self.profile_minting_usdc_price;
    }
}
