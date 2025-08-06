use anyhow::anyhow;
use reqwest::Client;
use rmcp::schemars;
use std::collections::HashMap;

use crate::tools::MultiTool;
use crate::tools::traits::BraveTools;

const BASE_URL: &str = "https://api.search.brave.com/res/v1";
const QUERY_PARAM: &str = "q";
const QUERY_PARAM_COUNT: &str = "count";

const HEADER_ACCEPT: &str = "Accept";
const HEADER_ACCEPT_ENCODING: &str = "Accept-Encoding";
const HEADER_SUBSCRIPTION_TOKEN: &str = "X-Subscription-Token";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct WebSearchInput {
    #[schemars(description = "Query string to use for web search")]
    pub query: String,
}

pub struct BraveContext {
    client: Client,
    api_key: String,
    base_url: String,
}

impl BraveContext {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: String::from(BASE_URL),
        }
    }
}

impl BraveTools for MultiTool {
    async fn search(&self, query: String) -> anyhow::Result<String> {
        let url = format!("{}/web/search", self.brave_ctx.base_url);

        // Set Query parameters, can add more if needed
        let mut params = HashMap::new();
        params.insert(QUERY_PARAM, query.clone());
        params.insert(QUERY_PARAM_COUNT, "10".to_string());

        let response = self
            .brave_ctx
            .client
            .get(&url)
            .header(HEADER_ACCEPT, "application/json")
            .header(HEADER_ACCEPT_ENCODING, "gzip")
            .header(HEADER_SUBSCRIPTION_TOKEN, &self.brave_ctx.api_key)
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Brave Search API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }

        let formatted_results = response.text().await?;

        Ok(formatted_results)
    }
}
