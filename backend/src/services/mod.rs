// Services module for external API integrations
//
// Each service encapsulates interaction with external providers:
// - gemini_service: Google Gemini for generation and embeddings
// - tavily_service: Tavily Search API for web search
// - places_service: Google Places API for local competitor data
// - qdrant_service: Qdrant vector database operations
// - document_service: Document ingestion and RAG pipeline

pub mod document_service;
pub mod gemini_service;
pub mod places_service;
pub mod qdrant_service;
pub mod tavily_service;
