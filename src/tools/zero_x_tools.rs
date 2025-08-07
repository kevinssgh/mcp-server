use anyhow::anyhow;
use reqwest::Client;
use rmcp::schemars;
use std::collections::HashMap;

use crate::tools::MultiTool;
use crate::tools::traits::ZeroXTools;

const BASE_URL: &str = "https://api.0x.org";

const QUOTE_PARAM_SELL_TOKEN: &str = "sellToken";
const QUOTE_PARAM_BUY_TOKEN: &str = "buyToken";
const QUOTE_PARAM_SELL_AMOUNT: &str = "sellAmount";
const QUOTE_PARAM_CHAIN_ID: &str = "chainId";

const HEADER_API_KEY: &str = "0x-api-key";
const HEADER_VERSION: &str = "0x-version";

const GET_PRICE_PATH: &str = "/swap/permit2/price";

/// Quote input struct
///
///     Fields:
///         from_token (String): The contract address of the token type being swapped from
///         to_token (String): The contract address of the token type being swapped to
///         amount (String): The amount to process the quote for
///
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

/// ZeroXContext
///
///     Description:
///         Used to keep the state of the connection to the 0x protocol.
///
///     Fields:
///         client (reqwest::Client): An HTTP client used to connect and communicate with the
///                                     0x protocol.
///         api_key (String): Authentication key used for the 0x api.
///         base_url (String): Base url of the 0x REST api
///
pub struct ZeroXContext {
    client: Client,
    api_key: String,
    base_url: String,
}

/// Context constructor
impl ZeroXContext {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: String::from(BASE_URL),
        }
    }
}

/// ZeroXTools
///
///     Description:
///         A toolset to communicate with 0x protocol api. Only  implements one path to request
///         quotes for token swaps.
///
impl ZeroXTools for MultiTool {
    async fn get_quote(&self, mut input: QuoteInput) -> anyhow::Result<String> {
        let mut params = HashMap::new();
        let url = format!("{}{GET_PRICE_PATH}", self.zero_x_context.base_url);

        // If getting quote with ETH as token type, need to convert to default address
        if input.from_token.to_lowercase().eq("eth")  {
            input.from_token = String::from(super::DEFAULT_ETH_TOKEN_ADDRESS)
        }
        if input.to_token.to_lowercase().eq("eth")  {
            input.to_token = String::from(super::DEFAULT_ETH_TOKEN_ADDRESS)
        }

        params.insert(QUOTE_PARAM_SELL_TOKEN, input.from_token);
        params.insert(QUOTE_PARAM_BUY_TOKEN, input.to_token);
        params.insert(QUOTE_PARAM_SELL_AMOUNT, input.amount);
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
