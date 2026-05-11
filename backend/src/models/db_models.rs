use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for documents (RAG source documents)
#[derive(Debug, Clone, Serialize, Deserialize)]
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
