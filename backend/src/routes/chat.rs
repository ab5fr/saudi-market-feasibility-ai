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

/// Returns true for greetings, filler, or confusion that should not trigger RAG retrieval.
fn is_conversational_only(message: &str) -> bool {
    let trimmed = message.trim().to_lowercase();

    if trimmed.is_empty() {
        return false;
    }

    const EXACT: &[&str] = &[
        "yo", "hi", "hey", "sup", "hello", "hiya", "what", "huh", "ok", "okay", "k", "thanks",
        "thank you", "thx", "bye", "goodbye", "cool", "nice", "lol", "hmm", "hm", "??", "???",
    ];

    if EXACT.contains(&trimmed.as_str()) {
        return true;
    }

    if trimmed.len() <= 20 {
        const PREFIXES: &[&str] = &[
            "hi ",
            "hey ",
            "hello ",
            "good morning",
            "good afternoon",
            "good evening",
            "what's up",
            "whats up",
            "how are you",
            "thank you",
        ];
        if PREFIXES.iter().any(|p| trimmed.starts_with(p)) {
            return true;
        }
    }

    false
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

    let (context, sources) = if is_conversational_only(question) {
        info!("Skipping RAG for conversational message");
        (String::new(), Vec::new())
    } else {
        match rag.retrieve_context_with_sources(question, 3).await {
            Ok((ctx, srcs, count)) => {
                info!("Retrieved {} relevant chunks from RAG", count);
                if count == 0 {
                    (String::new(), Vec::new())
                } else {
                    (ctx, srcs)
                }
            }
            Err(e) => {
                info!("RAG retrieval failed: {}. Answering without documents.", e);
                (String::new(), Vec::new())
            }
        }
    };

    let answer = gemini
        .generate_chat_answer(question, &context)
        .await
        .map_err(|e| AppError::AiService(format!("Failed to generate answer: {}", e)))?;

    Ok(Json(ApiResponse::success(ChatResponse { answer, sources })))
}
