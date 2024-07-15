use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use mpl_token_metadata::accounts::Metadata;

use crate::error::MyError;
use crate::other_states::{MintingCostDistribution, TradingPriceDistribution};

#[account]
pub struct MainState {
    pub owner: Pubkey,
    pub opos_token: Pubkey,
    pub profile_minting_cost: u64,
    pub invitation_minting_cost: u64,
    pub minting_cost_distribution: MintingCostDistribution,
    pub trading_price_distribution: TradingPriceDistribution,
    pub seller_fee_basis_points: u16, //NOTE: may be later change
    pub _bump: u8,
    pub total_minted_profile: u64,
    pub profile_collection: Pubkey,
    pub genesis_profile: Pubkey,
    pub common_lut: Pubkey,
}

impl MainState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct MainStateInput {
    pub profile_minting_cost: u64,
    pub invitation_minting_cost: u64,
    pub opos_token: Pubkey,
    pub minting_cost_distribution: MintingCostDistribution,
    pub trading_price_distribution: TradingPriceDistribution,
    // pub activation_token_collection_id: Pubkey,
}

impl MainStateInput {
    pub fn set_value(&self, mut state: &mut MainState) {
        // state.activation_token_collection_id = self.activation_token_collection_id;
        state.opos_token = self.opos_token;
        state.minting_cost_distribution = self.minting_cost_distribution;
        state.trading_price_distribution = self.trading_price_distribution;
        state.profile_minting_cost = self.profile_minting_cost;
        state.invitation_minting_cost = self.invitation_minting_cost;
    }
}
