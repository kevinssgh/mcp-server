use anyhow::anyhow;
use reqwest::Client;
use rmcp::schemars;
use serde::Deserialize;
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

/// Brave Search API response structures
#[derive(Debug, Deserialize)]
pub struct BraveSearchResponse {
    pub web: Option<WebSearchResults>,
}

#[derive(Debug, Deserialize)]
pub struct WebSearchResults {
    pub results: Vec<WebResult>,
}

#[derive(Debug, Deserialize)]
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub description: String,
}

/// Implementation of Brave Context.
///
/// Added some helper functions to isolate contract addresses for this application, a more
/// generic parser can be implemented using the Brave API specifications but this is enough for
/// the type of commands we are sending.
impl BraveContext {
    /// Extract contract addresses from search results
    fn extract_contract_addresses(&self, text: &str) -> Vec<String> {
        let ethereum_addr_regex = regex::Regex::new(r"0x[a-fA-F0-9]{40}").unwrap();
        ethereum_addr_regex
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect()
    }

    /// Format search results specifically for contract address queries
    fn format_contract_search_results(
        &self,
        response: &BraveSearchResponse,
        query: &str,
    ) -> String {
        let mut formatted = String::new();
        formatted.push_str(&format!("🔍 Search results for: {}\n\n", query));

        let mut found_addresses = Vec::new();

        if let Some(web_results) = &response.web {
            for (i, result) in web_results.results.iter().take(10).enumerate() {
                formatted.push_str(&format!("{}. {}\n", i + 1, result.title));
                formatted.push_str(&format!("   URL: {}\n", result.url));
                formatted.push_str(&format!("   Description: {}\n", result.description));

                // Extract contract addresses from title and description
                let addresses_in_title = self.extract_contract_addresses(&result.title);
                let addresses_in_desc = self.extract_contract_addresses(&result.description);

                for addr in addresses_in_title.iter().chain(addresses_in_desc.iter()) {
                    if !found_addresses.contains(addr) {
                        found_addresses.push(addr.clone());
                        formatted.push_str(&format!("   📄 Contract Address: {}\n", addr));
                    }
                }

                formatted.push_str("\n");
            }
        }

        // Consolidate results into String
        if !found_addresses.is_empty() {
            formatted.push_str("Contract Addresses Found:\n");
            for (i, addr) in found_addresses.iter().enumerate() {
                formatted.push_str(&format!("   {}. {}\n", i + 1, addr));
            }
            formatted.push_str("\n");
        }

        formatted
    }

    /// Format general search results - only included WEB response objects
    fn format_general_search_results(&self, response: &BraveSearchResponse, query: &str) -> String {
        let mut formatted = String::new();
        formatted.push_str(&format!("🔍 Search results for: {}\n\n", query));

        if let Some(web_results) = &response.web {
            formatted.push_str("📄 Web Results:\n");
            for (i, result) in web_results.results.iter().take(8).enumerate() {
                formatted.push_str(&format!("{}. {}\n", i + 1, result.title));
                formatted.push_str(&format!("   {}\n", result.url));
                formatted.push_str(&format!("   {}\n\n", result.description));
            }
        }

        formatted
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

        // let search_response: BraveSearchResponse = response.json().await?;
        //
        // // Check if this looks like a contract address query
        // let is_contract_query = query.to_lowercase().contains("contract")
        //     || query.to_lowercase().contains("address")
        //     || query.to_lowercase().contains("token")
        //     || query.to_lowercase().contains("erc20")
        //     || query.to_lowercase().contains("erc721")
        //     || query.contains("0x");
        //
        // let formatted_results = if is_contract_query {
        //     self.brave_ctx
        //         .format_contract_search_results(&search_response, &query)
        // } else {
        //     self.brave_ctx
        //         .format_general_search_results(&search_response, &query)
        // };

        let formatted_results = response.text().await?;

        Ok(formatted_results)
    }
}
