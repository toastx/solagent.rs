use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
    IWallet, SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_birdeye::{get_market_data, get_token_overview, MarketDataResponse, TokenOverviewResponse};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct MarketDataArgs {
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct MarketDataOutput {
    pub data: MarketDataResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("MarketData error")]
pub struct MarketDataError;

pub struct MarketData<W: IWallet> {
    agent: Arc<SolanaAgentKit<W>>,
}

impl<W: IWallet> MarketData<W> {
    pub fn new(agent: Arc<SolanaAgentKit<W>>) -> Self {
        MarketData { agent }
    }
}

impl<W: IWallet + std::marker::Send + std::marker::Sync> Tool for MarketData<W> {
    const NAME: &'static str = "get_market_data";

    type Error = MarketDataError;
    type Args = MarketDataArgs;
    type Output = MarketDataOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_market_data".to_string(),
            description: "Get market data of single token by birdeye api".to_string(),
            parameters: parameters!(
                address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_market_data(&self.agent, &args.address).await.expect("get_market_data");

        Ok(MarketDataOutput { data })
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenOverviewArgs {
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenOverviewOutput {
    pub data: TokenOverviewResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("MarketData error")]
pub struct TokenOverviewError;

pub struct TokenOverview<W: IWallet> {
    agent: Arc<SolanaAgentKit<W>>,
}

impl<W: IWallet> TokenOverview<W> {
    pub fn new(agent: Arc<SolanaAgentKit<W>>) -> Self {
        TokenOverview { agent }
    }
}

impl<W: IWallet + std::marker::Send + std::marker::Sync> Tool for TokenOverview<W> {
    const NAME: &'static str = "get_token_overview";

    type Error = TokenOverviewError;
    type Args = TokenOverviewArgs;
    type Output = TokenOverviewOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_overview".to_string(),
            description: "Get overview of a token by birdeye api".to_string(),
            parameters: parameters!(
                address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_overview(&self.agent, &args.address).await.expect("get_token_overview");

        Ok(TokenOverviewOutput { data })
    }
}
