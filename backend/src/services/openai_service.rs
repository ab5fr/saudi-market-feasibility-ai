use reqwest::Client;
use tracing::{info, instrument, error};

use crate::config::AppConfig;

/// OpenAI Service
/// 
/// Used for: Creating vector embeddings of Arabic reference documents
/// Model: text-embedding-3-large
pub struct OpenAIService {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.openai_api_key.clone(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// Create embeddings for RAG document storage
    /// Uses text-embedding-3-large for 3072-dimensional embeddings
    #[instrument(skip(self, texts))]
    pub async fn create_embeddings(
        &self,
        texts: Vec<String>,
    ) -> anyhow::Result<Vec<Vec<f32>>> {
        info!("Creating embeddings for {} chunks using text-embedding-3-large", texts.len());
        
        if self.api_key.is_empty() {
            anyhow::bail!("OpenAI API key not configured");
        }

        let request_body = serde_json::json!({
            "model": "text-embedding-3-large",
            "input": texts,
            "encoding_format": "float"
        });

        let response = self.client
            .post(format!("{}/embeddings", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("OpenAI API error: {}", error_text);
        }

        let response_json: serde_json::Value = response.json().await?;
        let embeddings: Vec<Vec<f32>> = response_json["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
            .iter()
            .map(|item| {
                item["embedding"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Missing embedding"))?
                    .iter()
                    .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid embedding value")).map(|f| f as f32))
                    .collect::<Result<Vec<f32>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        info!("Successfully created {} embeddings (dim: 3072)", embeddings.len());
        Ok(embeddings)
    }

    /// Create embedding for a single query
    pub async fn create_query_embedding(
        &self,
        query: &str,
    ) -> anyhow::Result<Vec<f32>> {
        info!("Creating query embedding");
        
        let embeddings = self.create_embeddings(vec![query.to_string()]).await?;
        embeddings.into_iter().next().ok_or_else(|| anyhow::anyhow!("No embedding generated"))
    }

    /// Split text into chunks for embedding
    /// Uses a simple character-based approach; for production, consider:
    /// - Sentence/token boundaries
    /// - Overlapping chunks
    /// - Semantic chunking
    pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        
        for line in text.lines() {
            if current_chunk.len() + line.len() > max_chars && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
            }
            current_chunk.push_str(line);
            current_chunk.push('\n');
        }
        
        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }
        
        chunks
    }
}
