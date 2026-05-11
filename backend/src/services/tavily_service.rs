use reqwest::Client;
use tracing::{info, instrument};

use crate::config::AppConfig;

/// Tavily Search API Service
///
/// Used for: Deep web searches for competitor data without SEO spam
/// Provides AI-optimized search results
pub struct TavilyService {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TavilyService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.tavily_api_key.clone(),
            base_url: "https://api.tavily.com".to_string(),
        }
    }

    /// Search for competitor information
    #[instrument(skip(self))]
    pub async fn search_competitors(
        &self,
        query: &str,
        location: &str,
    ) -> anyhow::Result<serde_json::Value> {
        info!(
            "Searching Tavily for competitors: {} in {}",
            query, location
        );

        if self.api_key.is_empty() {
            anyhow::bail!("Tavily API key not configured");
        }

        let search_query = format!(
            "{} competitors {} Saudi Arabia reviews pricing",
            query, location
        );

        let request_body = serde_json::json!({
            "api_key": self.api_key,
            "query": search_query,
            "search_depth": "advanced",
            "include_answer": true,
            "include_images": false,
            "include_raw_content": false,
            "max_results": 20
        });

        let response = self
            .client
            .post(format!("{}/search", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Tavily API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        info!(
            "Tavily search returned {} results",
            result["results"].as_array().map(|r| r.len()).unwrap_or(0)
        );

        Ok(result)
    }

    /// Deep search with specific filters
    #[allow(dead_code)]
    pub async fn deep_search(
        &self,
        query: &str,
        include_domains: Option<Vec<String>>,
        exclude_domains: Option<Vec<String>>,
    ) -> anyhow::Result<serde_json::Value> {
        info!("Performing deep Tavily search: {}", query);

        let mut request_body = serde_json::json!({
            "api_key": self.api_key,
            "query": query,
            "search_depth": "advanced",
            "include_answer": true,
            "max_results": 30
        });

        if let Some(domains) = include_domains {
            request_body["include_domains"] = serde_json::json!(domains);
        }

        if let Some(domains) = exclude_domains {
            request_body["exclude_domains"] = serde_json::json!(domains);
        }

        let response = self
            .client
            .post(format!("{}/search", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Tavily API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }

    /// Search for specific competitor reviews and pricing
    #[allow(dead_code)]
    pub async fn search_competitor_reviews(
        &self,
        business_name: &str,
        location: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let query = format!(
            "{} {} reviews ratings customer feedback Saudi Arabia",
            business_name, location
        );
        self.deep_search(&query, None, None).await
    }
}
