use anyhow::Result;

/// Application configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub gemini_embedding_model: String,
    pub tavily_api_key: String,
    pub google_places_api_key: String,
    pub qdrant_url: String,
    pub environment: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            gemini_api_key: std::env::var("GEMINI_API_KEY").unwrap_or_default(),
            gemini_model: std::env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| "gemini-flash-latest".to_string()),
            gemini_embedding_model: std::env::var("GEMINI_EMBEDDING_MODEL")
                .unwrap_or_else(|_| "text-embedding-004".to_string()),
            tavily_api_key: std::env::var("TAVILY_API_KEY").unwrap_or_default(),
            google_places_api_key: std::env::var("GOOGLE_PLACES_API_KEY").unwrap_or_default(),
            qdrant_url: std::env::var("QDRANT_URL")
                .unwrap_or_else(|_| "http://localhost:6333".to_string()),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        })
    }
}
