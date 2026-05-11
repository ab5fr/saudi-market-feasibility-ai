// Services module for external API integrations
// 
// Each service encapsulates interaction with external AI providers:
// - gemini_service: Google Gemini 1.5 Pro for RAG document analysis
// - claude_service: Anthropic Claude 3.5 Sonnet for coding/logic tasks
// - openai_service: OpenAI text-embedding-3-large for vector embeddings
// - tavily_service: Tavily Search API for web search
// - places_service: Google Places API for local competitor data
// - qdrant_service: Qdrant vector database operations
// - document_service: Document ingestion and RAG pipeline

pub mod gemini_service;
pub mod claude_service;
pub mod openai_service;
pub mod tavily_service;
pub mod places_service;
pub mod qdrant_service;
pub mod document_service;
