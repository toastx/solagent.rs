// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]

use lazy_static::lazy_static;
use mpl_token_metadata::types::Creator;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// Metadata for deploying an NFT/Collection.
///
/// # Fields
///
/// - `name`: The name of the NFT.
/// - `uri`: The URI for the collection's metadata.
/// - `basis_points`: Optional. The basis points for the NFT.
/// - `creators`: Optional. A list of creators associated with the NFT.
///
// #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NFTMetadata {
    pub(crate) name: String,
    pub(crate) uri: String,
    pub(crate) basis_points: Option<u16>,      // Optional basis points
    pub(crate) creators: Option<Vec<Creator>>, // Optional list of creators
}

impl NFTMetadata {
    pub fn new(name: &str, uri: &str, basis_points: Option<u16>, creators: Option<Vec<(Pubkey, u8)>>) -> Self {
        let creators = creators.map(|creator_tuples| {
            creator_tuples
                .into_iter()
                .map(|(pubkey, share)| Creator { address: pubkey, verified: true, share })
                .collect::<Vec<Creator>>()
        });

        NFTMetadata { name: name.to_string(), uri: uri.to_string(), basis_points, creators }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployedData {
    pub mint: String,      // mint address
    pub signature: String, // Tx hash
}

impl DeployedData {
    pub fn new(mint: String, signature: String) -> Self {
        DeployedData { mint, signature }
    }
}

/// USDC

lazy_static! {
    /// Common token addresses used across the toolkit
    pub static ref TOKENS: HashMap<&'static str, Pubkey> = {
        let mut m = HashMap::new();
        m.insert("USDC", Pubkey::from_str_const(USDC));
        m.insert("USDT", Pubkey::from_str_const("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"));
        m.insert("USDS", Pubkey::from_str_const("USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA"));
        m.insert("SOL", Pubkey::from_str_const("So11111111111111111111111111111111111111112"));
        m.insert("jitoSOL", Pubkey::from_str_const("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"));
        m.insert("bSOL", Pubkey::from_str_const("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1"));
        m.insert("mSOL", Pubkey::from_str_const("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"));
        m.insert("BONK", Pubkey::from_str_const("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"));
        m
    };
}

/// Default configuration options
pub const DEFAULT_OPTIONS: Options = Options { slippage_bps: 300, token_decimals: 9, referral_fee: 200 };

#[derive(Debug)]
pub struct Options {
    pub slippage_bps: u32,   // Default slippage tolerance in basis points (300 = 3%)
    pub token_decimals: u32, // Default number of decimals for new tokens
    pub referral_fee: u32,   // Referral fee
}



/// Pyth API URL
pub const PYTH_API: &str = "https://hermes.pyth.network/v2";

/// Rugcheck API URL

