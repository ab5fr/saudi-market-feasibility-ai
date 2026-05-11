use reqwest::Client;
use tracing::{info, instrument};

use crate::config::AppConfig;

/// Qdrant Vector Database Service
///
/// Used for: Storing and retrieving document embeddings for RAG
pub struct QdrantService {
    client: Client,
    base_url: String,
    collection_name: String,
}

impl QdrantService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            base_url: config.qdrant_url.clone(),
            collection_name: "saudi_government_docs".to_string(),
        }
    }

    /// Store document embeddings
    #[allow(dead_code)]
    #[instrument(skip(self, embeddings))]
    pub async fn store_embeddings(
        &self,
        documents: Vec<String>,
        embeddings: Vec<Vec<f32>>,
        metadata: Vec<serde_json::Value>,
    ) -> anyhow::Result<Vec<String>> {
        info!("Storing {} embeddings in Qdrant", embeddings.len());

        // Build points for upsert
        let points: Vec<serde_json::Value> = embeddings
            .iter()
            .enumerate()
            .map(|(idx, embedding)| {
                let point_id = uuid::Uuid::new_v4().to_string();
                serde_json::json!({
                    "id": point_id,
                    "vector": embedding,
                    "payload": {
                        "text": documents.get(idx).cloned().unwrap_or_default(),
                        "metadata": metadata.get(idx).cloned().unwrap_or(serde_json::json!({})),
                        "chunk_index": idx
                    }
                })
            })
            .collect();

        let request_body = serde_json::json!({
            "points": points
        });

        let url = format!(
            "{}/collections/{}/points?wait=true",
            self.base_url, self.collection_name
        );

        let response = self
            .client
            .put(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Qdrant upsert error: {}", error_text);
        }

        let point_ids: Vec<String> = points
            .iter()
            .map(|p| p["id"].as_str().unwrap_or("").to_string())
            .collect();

        info!("Successfully stored {} embeddings", point_ids.len());
        Ok(point_ids)
    }

    /// Search for similar documents
    pub async fn search_similar(
        &self,
        query_embedding: Vec<f32>,
        top_k: i64,
    ) -> anyhow::Result<Vec<SearchResult>> {
        info!("Searching Qdrant for {} similar documents", top_k);

        let request_body = serde_json::json!({
            "vector": query_embedding,
            "limit": top_k,
            "with_payload": true
        });

        let url = format!(
            "{}/collections/{}/points/search",
            self.base_url, self.collection_name
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Qdrant search error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;

        let search_results: Vec<SearchResult> = result["result"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|item| SearchResult {
                id: item["id"].as_str().unwrap_or("").to_string(),
                score: item["score"].as_f64().unwrap_or(0.0) as f32,
                payload: item["payload"].clone(),
                text: item["payload"]["text"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(search_results)
    }

    /// Create collection if not exists
    #[allow(dead_code)]
    pub async fn ensure_collection(&self) -> anyhow::Result<()> {
        info!(
            "Ensuring Qdrant collection '{}' exists",
            self.collection_name
        );

        // Check if collection exists
        let url = format!("{}/collections/{}", self.base_url, self.collection_name);

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            info!("Collection '{}' already exists", self.collection_name);
            return Ok(());
        }

        // Create collection
        let create_url = format!("{}/collections/{}", self.base_url, self.collection_name);

        let request_body = serde_json::json!({
            "vectors": {
                "size": 768, // text-embedding-004 dimension
                "distance": "Cosine"
            }
        });

        let create_response = self
            .client
            .put(&create_url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !create_response.status().is_success() {
            let error_text = create_response.text().await?;
            anyhow::bail!("Failed to create Qdrant collection: {}", error_text);
        }

        info!("Created Qdrant collection '{}'", self.collection_name);
        Ok(())
    }

    /// Delete collection
    #[allow(dead_code)]
    pub async fn delete_collection(&self) -> anyhow::Result<()> {
        let url = format!("{}/collections/{}", self.base_url, self.collection_name);

        let response = self.client.delete(&url).send().await?;

        if response.status().is_success() {
            info!("Deleted Qdrant collection '{}'", self.collection_name);
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SearchResult {
    #[allow(dead_code)]
    pub id: String,
    pub score: f32,
    #[allow(dead_code)]
    pub payload: serde_json::Value,
    pub text: String,
}
