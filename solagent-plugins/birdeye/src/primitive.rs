use serde::{Deserialize, Serialize};

pub const BIRDEYE_URL: &str = "https://public-api.birdeye.so";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenOverviewResponse {
    pub data: TokenData,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub extensions: Extensions,
    pub logo_uri: String,
    pub liquidity: f64,
    pub last_trade_unix_time: u64,
    pub last_trade_human_time: String,
    pub price: f64,
    pub history_30m_price: f64,
    pub price_change_30m_percent: f64,
    pub history_1h_price: f64,
    pub price_change_1h_percent: f64,
    pub history_2h_price: f64,
    pub price_change_2h_percent: f64,
    pub history_4h_price: f64,
    pub price_change_4h_percent: f64,
    pub history_6h_price: f64,
    pub price_change_6h_percent: f64,
    pub history_8h_price: f64,
    pub price_change_8h_percent: f64,
    pub history_12h_price: f64,
    pub price_change_12h_percent: f64,
    pub history_24h_price: f64,
    pub price_change_24h_percent: f64,
    pub unique_wallet_30m: u64,
    pub unique_wallet_history_30m: u64,
    pub unique_wallet_30m_change_percent: f64,
    pub unique_wallet_1h: u64,
    pub unique_wallet_history_1h: u64,
    pub unique_wallet_1h_change_percent: f64,
    pub unique_wallet_2h: u64,
    pub unique_wallet_history_2h: u64,
    pub unique_wallet_2h_change_percent: f64,
    pub unique_wallet_4h: u64,
    pub unique_wallet_history_4h: u64,
    pub unique_wallet_4h_change_percent: f64,
    pub unique_wallet_8h: u64,
    pub unique_wallet_history_8h: u64,
    pub unique_wallet_8h_change_percent: f64,
    pub unique_wallet_24h: u64,
    pub unique_wallet_history_24h: u64,
    pub unique_wallet_24h_change_percent: f64,
    pub supply: f64,
    pub mc: f64,
    pub circulating_supply: f64,
    pub real_mc: f64,
    pub holder: u64,
    pub trade_30m: u64,
    pub trade_history_30m: u64,
    pub trade_30m_change_percent: f64,
    pub sell_30m: u64,
    pub sell_history_30m: u64,
    pub sell_30m_change_percent: f64,
    pub buy_30m: u64,
    pub buy_history_30m: u64,
    pub buy_30m_change_percent: f64,
    pub v_30m: f64,
    pub v_30m_usd: f64,
    pub v_history_30m: f64,
    pub v_history_30m_usd: f64,
    pub v_30m_change_percent: f64,
    pub v_buy_30m: f64,
    pub v_buy_30m_usd: f64,
    pub v_buy_history_30m: f64,
    pub v_buy_history_30m_usd: f64,
    pub v_buy_30m_change_percent: f64,
    pub v_sell_30m: f64,
    pub v_sell_30m_usd: f64,
    pub v_sell_history_30m: f64,
    pub v_sell_history_30m_usd: f64,
    pub v_sell_30m_change_percent: f64,
    pub trade_1h: u64,
    pub trade_history_1h: u64,
    pub trade_1h_change_percent: f64,
    pub sell_1h: u64,
    pub sell_history_1h: u64,
    pub sell_1h_change_percent: f64,
    pub buy_1h: u64,
    pub buy_history_1h: u64,
    pub buy_1h_change_percent: f64,
    pub v_1h: f64,
    pub v_1h_usd: f64,
    pub v_history_1h: f64,
    pub v_history_1h_usd: f64,
    pub v_1h_change_percent: f64,
    pub v_buy_1h: f64,
    pub v_buy_1h_usd: f64,
    pub v_buy_history_1h: f64,
    pub v_buy_history_1h_usd: f64,
    pub v_buy_1h_change_percent: f64,
    pub v_sell_1h: f64,
    pub v_sell_1h_usd: f64,
    pub v_sell_history_1h: f64,
    pub v_sell_history_1h_usd: f64,
    pub v_sell_1h_change_percent: f64,
    pub trade_2h: u64,
    pub trade_history_2h: u64,
    pub trade_2h_change_percent: f64,
    pub sell_2h: u64,
    pub sell_history_2h: u64,
    pub sell_2h_change_percent: f64,
    pub buy_2h: u64,
    pub buy_history_2h: u64,
    pub buy_2h_change_percent: f64,
    pub v_2h: f64,
    pub v_2h_usd: f64,
    pub v_history_2h: f64,
    pub v_history_2h_usd: f64,
    pub v_2h_change_percent: f64,
    pub v_buy_2h: f64,
    pub v_buy_2h_usd: f64,
    pub v_buy_history_2h: f64,
    pub v_buy_history_2h_usd: f64,
    pub v_buy_2h_change_percent: f64,
    pub v_sell_2h: f64,
    pub v_sell_2h_usd: f64,
    pub v_sell_history_2h: f64,
    pub v_sell_history_2h_usd: f64,
    pub v_sell_2h_change_percent: f64,
    pub trade_4h: u64,
    pub trade_history_4h: u64,
    pub trade_4h_change_percent: f64,
    pub sell_4h: u64,
    pub sell_history_4h: u64,
    pub sell_4h_change_percent: f64,
    pub buy_4h: u64,
    pub buy_history_4h: u64,
    pub buy_4h_change_percent: f64,
    pub v_4h: f64,
    pub v_4h_usd: f64,
    pub v_history_4h: f64,
    pub v_history_4h_usd: f64,
    pub v_4h_change_percent: f64,
    pub v_buy_4h: f64,
    pub v_buy_4h_usd: f64,
    pub v_buy_history_4h: f64,
    pub v_buy_history_4h_usd: f64,
    pub v_buy_4h_change_percent: f64,
    pub v_sell_4h: f64,
    pub v_sell_4h_usd: f64,
    pub v_sell_history_4h: f64,
    pub v_sell_history_4h_usd: f64,
    pub v_sell_4h_change_percent: f64,
    pub trade_8h: u64,
    pub trade_history_8h: u64,
    pub trade_8h_change_percent: f64,
    pub sell_8h: u64,
    pub sell_history_8h: u64,
    pub sell_8h_change_percent: f64,
    pub buy_8h: u64,
    pub buy_history_8h: u64,
    pub buy_8h_change_percent: f64,
    pub v_8h: f64,
    pub v_8h_usd: f64,
    pub v_history_8h: f64,
    pub v_history_8h_usd: f64,
    pub v_8h_change_percent: f64,
    pub v_buy_8h: f64,
    pub v_buy_8h_usd: f64,
    pub v_buy_history_8h: f64,
    pub v_buy_history_8h_usd: f64,
    pub v_buy_8h_change_percent: f64,
    pub v_sell_8h: f64,
    pub v_sell_8h_usd: f64,
    pub v_sell_history_8h: f64,
    pub v_sell_history_8h_usd: f64,
    pub v_sell_8h_change_percent: f64,
    pub trade_24h: u64,
    pub trade_history_24h: u64,
    pub trade_24h_change_percent: f64,
    pub sell_24h: u64,
    pub sell_history_24h: u64,
    pub sell_24h_change_percent: f64,
    pub buy_24h: u64,
    pub buy_history_24h: u64,
    pub buy_24h_change_percent: f64,
    pub v_24h: f64,
    pub v_24h_usd: f64,
    pub v_history_24h: f64,
    pub v_history_24h_usd: f64,
    pub v_24h_change_percent: f64,
    pub v_buy_24h: f64,
    pub v_buy_24h_usd: f64,
    pub v_buy_history_24h: f64,
    pub v_buy_history_24h_usd: f64,
    pub v_buy_24h_change_percent: f64,
    pub v_sell_24h: f64,
    pub v_sell_24h_usd: f64,
    pub v_sell_history_24h: f64,
    pub v_sell_history_24h_usd: f64,
    pub v_sell_24h_change_percent: f64,
    pub watch: Option<String>,
    pub number_markets: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub coingecko_id: String,
    pub serum_v3_usdc: String,
    pub serum_v3_usdt: String,
    pub website: String,
    pub telegram: Option<String>,
    pub twitter: String,
    pub description: String,
    pub discord: String,
    pub medium: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub address: String,
    pub price: f64,
    pub liquidity: f64,
    pub supply: f64,
    pub marketcap: f64,
    pub circulating_supply: f64,
    pub circulating_marketcap: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDataResponse {
    pub data: MarketData,
    pub success: bool,
}

// Wallet portfolio
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletPortfolioResponse {
    pub success: bool,
    pub data: WalletPortfolio,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WalletPortfolio {
    pub wallet: String,
    pub total_usd: Option<f64>,
    pub items: Vec<TokenMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TokenMetadata {
    pub address: String,
    pub decimals: u8,
    pub balance: u64,

    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,

    #[serde(rename = "chainId")]
    pub chain_id: String,

    pub name: Option<String>,
    pub symbol: Option<String>,
    pub icon: Option<String>,

    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,

    #[serde(rename = "priceUsd")]
    pub price_usd: Option<f64>,

    pub value_usd: Option<f64>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Token Holder
///
/// ///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TokenHolderQueryParams {
    /// Address of a token
    pub address: String,

    /// offset integer 0 to 10000 Defaults to 0
    pub offset: Option<u32>,

    /// limit integer 0 to 100 Defaults to 100
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenHolderResponse {
    pub data: TokenHolderData,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenHolderData {
    pub items: Vec<TokenHolderInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenHolderInfo {
    pub amount: String,
    pub decimals: u8,
    pub mint: String,
    pub owner: String,
    pub token_account: String,
    pub ui_amount: f64,
}
