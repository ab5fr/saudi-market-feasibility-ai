use std::path::Path;
use tracing::{info, error};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    models::db_models::{Document, DocumentChunk},
    services::{openai_service::OpenAIService, qdrant_service::QdrantService},
};

/// Document ingestion and processing service
/// Handles PDF/text files, chunks them, creates embeddings, and stores in Qdrant
pub struct DocumentService {
    openai: OpenAIService,
    qdrant: QdrantService,
    documents_path: String,
}

impl DocumentService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            openai: OpenAIService::new(config),
            qdrant: QdrantService::new(config),
            documents_path: "/app/documents".to_string(),
        }
    }

    /// Process a document file and add it to the RAG system
    pub async fn process_document(
        &self,
        file_path: &str,
        document_type: &str,
        authority: Option<&str>,
        description: Option<&str>,
    ) -> anyhow::Result<Document> {
        info!("Processing document: {}", file_path);

        // Read file content
        let content = self.read_file(file_path).await?;
        
        // Extract text based on file type
        let text = self.extract_text(&content, file_path).await?;

        // Create document record
        let document = Document {
            id: Uuid::new_v4(),
            filename: Path::new(file_path).file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            original_name: file_path.to_string(),
            file_path: file_path.to_string(),
            file_size: Some(content.len() as i64),
            mime_type: self.detect_mime_type(file_path),
            document_type: document_type.to_string(),
            authority: authority.map(|a| a.to_string()),
            description: description.map(|d| d.to_string()),
            is_processed: false,
            chunk_count: 0,
            created_at: chrono::Utc::now(),
        };

        // Chunk the text
        let chunks = self.chunk_text(&text, 2000, 200); // 2000 chars, 200 overlap
        info!("Created {} chunks from document", chunks.len());

        // Create embeddings for chunks
        let embeddings = self.openai.create_embeddings(chunks.clone()).await?;
        
        // Prepare metadata for each chunk
        let metadata: Vec<serde_json::Value> = chunks.iter().enumerate().map(|(idx, _)| {
            serde_json::json!({
                "document_id": document.id.to_string(),
                "document_type": document_type,
                "authority": authority,
                "chunk_index": idx,
                "total_chunks": chunks.len()
            })
        }).collect();

        // Store in Qdrant
        let point_ids = self.qdrant.store_embeddings(chunks, embeddings, metadata).await?;
        info!("Stored {} embeddings in Qdrant", point_ids.len());

        Ok(document)
    }

    /// Search for relevant document chunks
    pub async fn search_relevant_chunks(
        &self,
        query: &str,
        top_k: i64,
    ) -> anyhow::Result<Vec<(String, f32)>> {
        // Create query embedding
        let query_embedding = self.openai.create_query_embedding(query).await?;
        
        // Search in Qdrant
        let results = self.qdrant.search_similar(query_embedding, top_k).await?;
        
        // Return text and score
        Ok(results.into_iter()
            .map(|r| (r.text, r.score))
            .collect())
    }

    /// Read file content
    async fn read_file(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        tokio::fs::read(path).await
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", path, e))
    }

    /// Extract text from various file formats
    /// For simplicity, this handles text files. For PDFs, you'd use a PDF extraction library
    async fn extract_text(&self, content: &[u8], file_path: &str) -> anyhow::Result<String> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "txt" | "md" | "csv" => {
                String::from_utf8(content.to_vec())
                    .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
            }
            "pdf" => {
                // PDF support temporarily disabled to avoid build issues
                // To enable: add pdf-extract to Cargo.toml and uncomment extract_pdf_text method
                info!("PDF file detected but extraction temporarily disabled: {}", file_path);
                anyhow::bail!("PDF support temporarily disabled. Please convert to .txt format or see HOW_TO_ADD_PDF.md for instructions.")
            }
            _ => {
                // Try to interpret as text
                String::from_utf8(content.to_vec())
                    .map_err(|_| anyhow::anyhow!("Unsupported file format: {}", extension))
            }
        }
    }

    /// Chunk text into overlapping segments
    fn chunk_text(&self, text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut start = 0;
        
        while start < text.len() {
            let end = (start + chunk_size).min(text.len());
            let chunk = &text[start..end];
            
            // Try to end at a sentence or paragraph boundary
            let chunk_text = if end < text.len() {
                if let Some(pos) = chunk.rfind(". ") {
                    &text[start..start + pos + 1]
                } else if let Some(pos) = chunk.rfind("\n\n") {
                    &text[start..start + pos]
                } else {
                    chunk
                }
            } else {
                chunk
            };
            
            chunks.push(chunk_text.trim().to_string());
            
            // Move forward with overlap
            let next_start = start + chunk_text.len();
            if next_start > start {
                start = next_start - overlap.min(chunk_text.len());
            } else {
                break;
            }
            
            if next_start >= text.len() {
                break;
            }
        }
        
        chunks
    }

    // NOTE: PDF extraction functions temporarily disabled
    // To enable PDF support, add pdf-extract = "0.7" to Cargo.toml
    // and uncomment these functions
    
    /*
    /// Extract text from PDF file content
    fn extract_pdf_text(&self, content: &[u8]) -> anyhow::Result<String> {
        use std::io::Cursor;
        
        // Create a cursor from the PDF bytes
        let cursor = Cursor::new(content);
        
        // Extract text using pdf-extract
        let text = pdf_extract::extract_text_from_mem(content)
            .map_err(|e| anyhow::anyhow!("PDF extraction error: {}", e))?;
        
        // Clean up the extracted text
        let cleaned_text = self.clean_extracted_text(&text);
        
        if cleaned_text.is_empty() {
            anyhow::bail!("PDF extraction returned empty text. The PDF may be image-based or scanned.")
        }
        
        Ok(cleaned_text)
    }
    
    /// Clean up extracted text (remove extra whitespace, fix encoding issues)
    fn clean_extracted_text(&self, text: &str) -> String {
        text
            // Replace multiple spaces with single space
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            // Fix common PDF extraction artifacts
            .replace("\n\n\n", "\n\n")
            .replace("  ", " ")
            // Remove null bytes
            .replace('\0', "")
            .trim()
            .to_string()
    }
    */

    /// Detect MIME type from file extension
    fn detect_mime_type(&self, file_path: &str) -> Option<String> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())?
            .to_lowercase();
        
        match extension.as_str() {
            "txt" => Some("text/plain".to_string()),
            "md" => Some("text/markdown".to_string()),
            "pdf" => Some("application/pdf".to_string()),
            "csv" => Some("text/csv".to_string()),
            "json" => Some("application/json".to_string()),
            _ => Some("application/octet-stream".to_string()),
        }
    }

    /// List all documents in the documents directory
    pub async fn list_documents(&self) -> anyhow::Result<Vec<String>> {
        let mut files = Vec::new();
        
        match tokio::fs::read_dir(&self.documents_path).await {
            Ok(mut entries) => {
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.is_file() {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
            Err(e) => {
                error!("Failed to read documents directory: {}", e);
            }
        }
        
        Ok(files)
    }

    /// Process all documents in the documents directory
    pub async fn process_all_documents(&self) -> anyhow::Result<Vec<Document>> {
        let files = self.list_documents().await?;
        let mut documents = Vec::new();
        
        for file in files {
            // Determine document type from path
            let (doc_type, authority) = self.classify_document(&file);
            
            match self.process_document(&file, doc_type, authority, None).await {
                Ok(doc) => documents.push(doc),
                Err(e) => error!("Failed to process {}: {}", file, e),
            }
        }
        
        Ok(documents)
    }

    /// Classify document based on file path/name
    fn classify_document(&self, path: &str) -> (&str, Option<&str>) {
        let lower_path = path.to_lowercase();
        
        if lower_path.contains("monshaat") {
            ("government", Some("Monshaat"))
        } else if lower_path.contains("qiwa") {
            ("government", Some("Qiwa"))
        } else if lower_path.contains("balady") || lower_path.contains("municipal") {
            ("government", Some("Balady"))
        } else if lower_path.contains("gosi") || lower_path.contains("social_insurance") {
            ("government", Some("GOSI"))
        } else if lower_path.contains("moc") || lower_path.contains("commerce") {
            ("government", Some("Ministry of Commerce"))
        } else if lower_path.contains("sagia") || lower_path.contains("misa") {
            ("government", Some("MISA"))
        } else if lower_path.contains("zakat") || lower_path.contains("tax") {
            ("government", Some("ZATCA"))
        } else if lower_path.contains("feasibility") || lower_path.contains("template") {
            ("feasibility_template", None)
        } else {
            ("regulation", None)
        }
    }
}

/// RAG Pipeline for retrieving context
pub struct RagPipeline {
    document_service: DocumentService,
    gemini: crate::services::gemini_service::GeminiService,
}

impl RagPipeline {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            document_service: DocumentService::new(config),
            gemini: crate::services::gemini_service::GeminiService::new(config),
        }
    }

    /// Retrieve relevant context for a business query
    pub async fn retrieve_context(
        &self,
        query: &str,
        top_k: i64,
    ) -> anyhow::Result<String> {
        // Search for relevant chunks
        let chunks = self.document_service.search_relevant_chunks(query, top_k).await?;
        
        // Combine chunks into context
        let context = chunks.into_iter()
            .map(|(text, score)| format!("[Relevance: {:.2}]\n{}", score, text))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");
        
        Ok(context)
    }

    /// Generate a grounded response using RAG
    pub async fn generate_grounded_response(
        &self,
        query: &str,
    ) -> anyhow::Result<String> {
        // Retrieve context
        let context = self.retrieve_context(query, 5).await?;
        
        // Generate response using Gemini
        let response = self.gemini.generate_grounded_response(query, &[context]).await?;
        
        Ok(response)
    }
}
