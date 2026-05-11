use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for users
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for feasibility projects
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeasibilityProject {
    pub id: Uuid,
    pub user_id: Uuid,
    pub business_name: String,
    pub description: String,
    pub target_city: String,
    pub district: Option<String>,
    pub capital_budget: f64,
    pub industry: String,
    pub business_model: String,
    pub initial_employees: i32,
    pub founder_experience: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for feasibility studies
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeasibilityStudy {
    pub id: Uuid,
    pub project_id: Uuid,
    pub viability_score: Option<f64>,
    pub executive_summary: Option<serde_json::Value>,
    pub market_analysis: Option<serde_json::Value>,
    pub financial_projections: Option<serde_json::Value>,
    pub legal_requirements: Option<serde_json::Value>,
    pub risk_assessment: Option<serde_json::Value>,
    pub recommendations: Option<serde_json::Value>,
    pub sources_cited: Option<serde_json::Value>,
    pub generated_at: DateTime<Utc>,
}

/// Database model for persona debates
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PersonaDebate {
    pub id: Uuid,
    pub project_id: Uuid,
    pub session_id: String,
    pub personas: serde_json::Value,
    pub debate_transcript: serde_json::Value,
    pub consensus_summary: Option<String>,
    pub key_risks: Option<serde_json::Value>,
    pub key_opportunities: Option<serde_json::Value>,
    pub overall_verdict: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Database model for competitor analyses
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompetitorAnalysis {
    pub id: Uuid,
    pub project_id: Uuid,
    pub analysis_id: String,
    pub search_location: Option<String>,
    pub search_query_used: Option<String>,
    pub competitors: Option<serde_json::Value>,
    pub market_saturation_score: Option<f64>,
    pub market_gap_analysis: Option<String>,
    pub pricing_benchmarks: Option<serde_json::Value>,
    pub online_presence_summary: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Database model for documents (RAG source documents)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub filename: String,
    pub original_name: String,
    pub file_path: String,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub document_type: String, // 'government', 'feasibility_template', 'regulation'
    pub authority: Option<String>, // 'Monshaat', 'Qiwa', 'Balady', 'GOSI'
    pub description: Option<String>,
    pub is_processed: bool,
    pub chunk_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Database model for document chunks (for RAG)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub id: Uuid,
    pub document_id: Uuid,
    pub chunk_index: i32,
    pub chunk_text: String,
    pub qdrant_point_id: Option<String>,
    pub token_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}
