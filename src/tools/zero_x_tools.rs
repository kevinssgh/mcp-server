use anyhow::anyhow;
use reqwest::Client;
use rmcp::schemars;
use std::collections::HashMap;

use crate::tools::MultiTool;
use crate::tools::traits::ZeroXTools;

const BASE_URL: &str = "https://api.0x.org";
const DEFAULT_ETH_TOKEN_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

const QUOTE_PARAM_SELL_TOKEN: &str = "sellToken";
const QUOTE_PARAM_BUY_TOKEN: &str = "buyToken";
const QUOTE_PARAM_SELL_AMOUNT: &str = "sellAmount";
const QUOTE_PARAM_CHAIN_ID: &str = "chainId";

const HEADER_API_KEY: &str = "0x-api-key";
const HEADER_VERSION: &str = "0x-version";

const GET_PRICE_PATH: &str = "/swap/permit2/price";

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct QuoteInput {
    #[schemars(
        description = "The token or currency address to be swapped from, can be simply Eth"
    )]
    pub from_token: String,
    #[schemars(description = "The address of the account to get the balance for")]
    pub to_token: String,
    #[schemars(description = "The amount of tokens to sell")]
    pub amount: String,
}

pub struct ZeroXContext {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ZeroXContext {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: String::from(BASE_URL),
        }
    }
}

impl ZeroXTools for MultiTool {
    async fn get_quote(
        &self,
        mut from_token: String,
        to_token: String,
        amount: String,
    ) -> anyhow::Result<String> {
        let mut params = HashMap::new();
        let url = format!("{}{GET_PRICE_PATH}", self.zero_x_context.base_url);

        // If getting quote from Eth to another token
        if from_token.to_lowercase().eq("eth") {
            from_token = String::from(DEFAULT_ETH_TOKEN_ADDRESS)
        }

        params.insert(QUOTE_PARAM_SELL_TOKEN, from_token);
        params.insert(QUOTE_PARAM_BUY_TOKEN, to_token);
        params.insert(QUOTE_PARAM_SELL_AMOUNT, amount);
        params.insert(QUOTE_PARAM_CHAIN_ID, String::from("1"));

        let response = self
            .zero_x_context
            .client
            .get(&url)
            .header(HEADER_API_KEY, &self.zero_x_context.api_key)
            .header(HEADER_VERSION, "v2")
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(anyhow!("0x API error: {}", error));
        }

        let quote = response.text().await?;
        Ok(quote)
    }
}
