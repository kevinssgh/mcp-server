//! Brave Search integration module.
//!
//! Provides a wrapper for querying the Brave Search API via the `BraveTools` trait,
//! including request building, authentication, and basic error handling.
//!
//! Note: This does not parse or deserialize the result for processing as the Agent will be
//!         expected to interpret this.
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

/// Input payload for a Brave web search request.
///
/// Designed for use with API schemas and deserialization.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct WebSearchInput {
    #[schemars(description = "Query string to use for web search")]
    pub query: String,
}

/// Holds configuration and HTTP client for Brave Search requests.
pub struct BraveContext {
    client: Client,
    api_key: String,
    base_url: String,
}

impl BraveContext {
    /// Creates a new Brave Search API context.
    ///
    /// # Arguments
    /// * `api_key` - Brave Search API subscription token.
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: String::from(BASE_URL),
        }
    }
}

impl BraveTools for MultiTool {
    /// Performs a Brave web search using the given query string.
    ///
    /// Sends a GET request to the Brave Search API and returns the raw JSON response.
    ///
    /// # Arguments
    /// * `query` - Search term to query Brave Search.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if the API responds with a non-success status.
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
