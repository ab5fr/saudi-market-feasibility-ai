use anyhow::Result;

/// Application configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub anthropic_api_key: String,
    pub openai_api_key: String,
    pub tavily_api_key: String,
    pub google_places_api_key: String,
    pub qdrant_url: String,
    pub jwt_secret: String,
    pub environment: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/saudi_market_ai".to_string()),
            gemini_api_key: std::env::var("GEMINI_API_KEY").unwrap_or_default(),
            gemini_model: std::env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| "gemini-flash-latest".to_string()),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").unwrap_or_default(),
            openai_api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            tavily_api_key: std::env::var("TAVILY_API_KEY").unwrap_or_default(),
            google_places_api_key: std::env::var("GOOGLE_PLACES_API_KEY").unwrap_or_default(),
            qdrant_url: std::env::var("QDRANT_URL")
                .unwrap_or_else(|_| "http://localhost:6333".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-key-change-in-production".to_string()),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        })
    }
}
