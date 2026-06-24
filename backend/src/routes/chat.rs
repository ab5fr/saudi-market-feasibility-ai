use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    config::AppConfig,
    models::{ApiResponse, AppError},
    services::{document_service::RagPipeline, gemini_service::GeminiService},
};

/// Chat request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub question: String,
}

/// Chat response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub answer: String,
    pub sources: Vec<String>,
}

/// POST /api/chat
///
/// Answers user questions using RAG pipeline to retrieve relevant documents
/// and Gemini to generate grounded, contextual responses.
#[instrument(skip(config), fields(question = %payload.question))]
pub async fn answer_question(
    State(config): State<AppConfig>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ApiResponse<ChatResponse>>, AppError> {
    let question = payload.question.trim();
    if question.is_empty() {
        return Err(AppError::Validation(
            "Question must not be empty".to_string(),
        ));
    }

    info!("Received chat question: {}", question);

    let rag = RagPipeline::new(&config);
    let gemini = GeminiService::new(&config);

    let (context, sources) = match rag.retrieve_context_with_sources(question, 3).await {
        Ok((ctx, srcs, count)) => {
            info!("Retrieved {} chunks from RAG", count);
            (ctx, srcs)
        }
        Err(e) => {
            info!("RAG retrieval failed: {}. Using general knowledge.", e);
            (
                "No specific documents found in the knowledge base.".to_string(),
                vec!["General knowledge".to_string()],
            )
        }
    };

    let prompt = format!(
        "Based on the following context, answer this question:\n\n\
         Context:\n{}\n\n\
         Question: {}\n\n\
         If the context doesn't contain relevant information, say so and provide a general answer.",
        context, question
    );

    let answer = gemini
        .generate_answer(&prompt)
        .await
        .map_err(|e| AppError::AiService(format!("Failed to generate answer: {}", e)))?;

    Ok(Json(ApiResponse::success(ChatResponse { answer, sources })))
}
