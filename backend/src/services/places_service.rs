use reqwest::Client;
use tracing::{info, instrument};

use crate::config::AppConfig;

/// Google Places API Service
/// 
/// Used for: Fetching real, local competitors with ratings, reviews, location data
pub struct PlacesService {
    client: Client,
    api_key: String,
    base_url: String,
}

impl PlacesService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.google_places_api_key.clone(),
            base_url: "https://maps.googleapis.com/maps/api".to_string(),
        }
    }

    /// Find local competitors near a location
    #[instrument(skip(self))]
    pub async fn find_local_competitors(
        &self,
        query: &str,
        location: &str,
        radius_meters: i32,
    ) -> anyhow::Result<serde_json::Value> {
        info!(
            "Searching Google Places for: '{}' near '{}' ({}m radius)",
            query, location, radius_meters
        );
        
        if self.api_key.is_empty() {
            anyhow::bail!("Google Places API key not configured");
        }

        // First, geocode the location to get coordinates
        let (lat, lng) = self.geocode_location(location).await?;
        
        // Build the Places API URL
        let url = format!(
            "{}/place/nearbysearch/json?location={},{}&radius={}&keyword={}&key={}",
            self.base_url,
            lat,
            lng,
            radius_meters,
            urlencoding::encode(query),
            self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Google Places API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        
        if result["status"].as_str() != Some("OK") && result["status"].as_str() != Some("ZERO_RESULTS") {
            anyhow::bail!("Google Places API error: {}", result["status"]);
        }

        let results_count = result["results"].as_array().map(|r| r.len()).unwrap_or(0);
        info!("Google Places search returned {} results", results_count);
        
        Ok(result)
    }

    /// Geocode a location string to coordinates
    pub async fn geocode_location(
        &self,
        address: &str,
    ) -> anyhow::Result<(f64, f64)> {
        let url = format!(
            "{}/geocode/json?address={}&key={}",
            self.base_url,
            urlencoding::encode(address),
            self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Geocoding API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        
        if result["status"].as_str() != Some("OK") {
            anyhow::bail!("Geocoding failed: {}", result["status"]);
        }

        let location = result["results"][0]["geometry"]["location"]
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("No location found"))?;
        
        let lat = location["lat"].as_f64().ok_or_else(|| anyhow::anyhow!("Invalid latitude"))?;
        let lng = location["lng"].as_f64().ok_or_else(|| anyhow::anyhow!("Invalid longitude"))?;

        Ok((lat, lng))
    }
}
