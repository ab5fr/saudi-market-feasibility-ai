use std::path::{Path, PathBuf};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    models::db_models::Document,
    services::{gemini_service::GeminiService, qdrant_service::QdrantService},
};

/// Document ingestion and processing service
/// Handles PDF/text files, chunks them, creates embeddings, and stores in Qdrant
pub struct DocumentService {
    gemini: GeminiService,
    qdrant: QdrantService,
    documents_path: String,
}

impl DocumentService {
    pub fn new(config: &AppConfig) -> Self {
        let documents_path = resolve_documents_path();
        Self {
            gemini: GeminiService::new(config),
            qdrant: QdrantService::new(config),
            documents_path,
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

        // Create document record with a stable ID derived from the file path
        let document_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, file_path.as_bytes());
        let filename = Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let document = Document {
            id: document_id,
            filename: filename.clone(),
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
        let embeddings = self.gemini.create_embeddings(chunks.clone()).await?;

        // Prepare metadata for each chunk
        let metadata: Vec<serde_json::Value> = chunks
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                serde_json::json!({
                    "document_id": document.id.to_string(),
                    "filename": filename,
                    "document_type": document_type,
                    "authority": authority,
                    "chunk_index": idx,
                    "total_chunks": chunks.len()
                })
            })
            .collect();

        // Remove stale chunks before re-ingesting the same document
        self.qdrant
            .delete_by_document_id(&document.id.to_string())
            .await?;

        // Store in Qdrant
        let point_ids = self
            .qdrant
            .store_embeddings(chunks, embeddings, metadata)
            .await?;
        info!("Stored {} embeddings in Qdrant", point_ids.len());

        Ok(document)
    }

    /// Search for relevant document chunks with source labels
    pub async fn search_relevant_chunks(
        &self,
        query: &str,
        top_k: i64,
    ) -> anyhow::Result<Vec<ChunkMatch>> {
        let query_embedding = self.gemini.create_query_embedding(query).await?;
        let results = self.qdrant.search_similar(query_embedding, top_k).await?;

        Ok(results
            .into_iter()
            .map(|r| ChunkMatch {
                text: r.text,
                score: r.score,
                source_label: source_label_from_payload(&r.payload),
            })
            .collect())
    }

    /// Read file content
    async fn read_file(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        tokio::fs::read(path)
            .await
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
            "txt" | "md" | "csv" => String::from_utf8(content.to_vec())
                .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e)),
            "pdf" => self.extract_pdf_text(content, file_path),
            _ => {
                // Try to interpret as text
                String::from_utf8(content.to_vec())
                    .map_err(|_| anyhow::anyhow!("Unsupported file format: {}", extension))
            }
        }
    }

    /// Chunk text into overlapping segments using char boundaries (UTF-8 safe)
    fn chunk_text(&self, text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        if len == 0 {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < len {
            let end = (start + chunk_size).min(len);
            let chunk: String = chars[start..end].iter().collect();

            let chunk_text = if end < len {
                if let Some(pos) = chunk.rfind(". ") {
                    chars[start..start + pos + 1].iter().collect()
                } else if let Some(pos) = chunk.rfind("\n\n") {
                    chars[start..start + pos].iter().collect()
                } else {
                    chunk
                }
            } else {
                chunk
            };

            let trimmed = chunk_text.trim();
            if !trimmed.is_empty() {
                chunks.push(trimmed.to_string());
            }

            let chunk_len = chunk_text.chars().count();
            if chunk_len == 0 {
                break;
            }

            let next_start = start + chunk_len;
            if next_start >= len {
                break;
            }

            start = next_start.saturating_sub(overlap.min(chunk_len));
        }

        chunks
    }

    /// Extract text from PDF file content
    fn extract_pdf_text(&self, content: &[u8], file_path: &str) -> anyhow::Result<String> {
        let text = pdf_extract::extract_text_from_mem(content)
            .map_err(|e| anyhow::anyhow!("PDF extraction error ({}): {}", file_path, e))?;

        let cleaned_text = self.clean_extracted_text(&text);
        if cleaned_text.is_empty() {
            anyhow::bail!(
                "PDF extraction returned empty text. The PDF may be image-based or scanned: {}",
                file_path
            )
        }

        Ok(cleaned_text)
    }

    /// Clean up extracted text (remove extra whitespace, fix encoding issues)
    fn clean_extracted_text(&self, text: &str) -> String {
        let mut cleaned = String::with_capacity(text.len());
        let mut in_whitespace = false;

        for ch in text.chars() {
            if ch == '\0' {
                continue;
            }
            if ch.is_whitespace() {
                if !in_whitespace {
                    cleaned.push(' ');
                    in_whitespace = true;
                }
            } else {
                cleaned.push(ch);
                in_whitespace = false;
            }
        }

        cleaned.trim().to_string()
    }

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

        let mut stack = vec![PathBuf::from(&self.documents_path)];

        while let Some(dir) = stack.pop() {
            match tokio::fs::read_dir(&dir).await {
                Ok(mut entries) => {
                    while let Some(entry) = entries.next_entry().await? {
                        let path = entry.path();
                        if path.is_dir() {
                            stack.push(path);
                        } else if path.is_file() {
                            files.push(path.to_string_lossy().to_string());
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to read documents directory {}: {}",
                        dir.display(),
                        e
                    );
                }
            }
        }

        Ok(files)
    }

    /// Process all documents in the documents directory
    pub async fn process_all_documents(&self) -> anyhow::Result<Vec<Document>> {
        let files = self.list_documents().await?;
        let mut documents = Vec::new();

        for file in files {
            match self.process_path(&file).await {
                Ok(doc) => documents.push(doc),
                Err(e) => error!("Failed to process {}: {}", file, e),
            }
        }

        Ok(documents)
    }

    /// Process a document path using auto-classification.
    pub async fn process_path(&self, file_path: &str) -> anyhow::Result<Document> {
        // Determine document type from path
        let (doc_type, authority) = self.classify_document(file_path);

        self.process_document(file_path, doc_type, authority, None)
            .await
    }

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

fn resolve_documents_path() -> String {
    if let Ok(path) = std::env::var("DOCUMENTS_PATH") {
        return path;
    }

    let candidates = [
        "../documents",
        "./documents",
        "../../documents",
        "/app/documents",
    ];

    for candidate in candidates {
        if Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }

    "/app/documents".to_string()
}

/// A retrieved document chunk with relevance score and source label
pub struct ChunkMatch {
    pub text: String,
    pub score: f32,
    pub source_label: String,
}

fn source_label_from_payload(payload: &serde_json::Value) -> String {
    let metadata = payload.get("metadata").unwrap_or(payload);

    if let Some(authority) = metadata.get("authority").and_then(|v| v.as_str())
        && !authority.is_empty()
    {
        return authority.to_string();
    }

    if let Some(filename) = metadata.get("filename").and_then(|v| v.as_str())
        && !filename.is_empty()
    {
        return filename.to_string();
    }

    metadata
        .get("document_type")
        .and_then(|v| v.as_str())
        .unwrap_or("Document")
        .to_string()
}

fn dedupe_sources(sources: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for source in sources {
        if !unique.iter().any(|existing| existing == &source) {
            unique.push(source);
        }
    }
    unique
}

/// RAG Pipeline for retrieving context
pub struct RagPipeline {
    document_service: DocumentService,
}

/// Minimum cosine similarity for a chunk to be treated as relevant context
const MIN_RELEVANCE_SCORE: f32 = 0.62;

impl RagPipeline {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            document_service: DocumentService::new(config),
        }
    }

    /// Retrieve relevant context for a business query
    pub async fn retrieve_context(&self, query: &str, top_k: i64) -> anyhow::Result<String> {
        let (context, _, _) = self.retrieve_context_with_sources(query, top_k).await?;
        Ok(context)
    }

    /// Retrieve context text, deduplicated source labels, and chunk count
    pub async fn retrieve_context_with_sources(
        &self,
        query: &str,
        top_k: i64,
    ) -> anyhow::Result<(String, Vec<String>, usize)> {
        let chunks = self
            .document_service
            .search_relevant_chunks(query, top_k)
            .await?;

        let relevant: Vec<_> = chunks
            .into_iter()
            .filter(|c| c.score >= MIN_RELEVANCE_SCORE)
            .collect();

        let count = relevant.len();
        let sources = dedupe_sources(relevant.iter().map(|c| c.source_label.clone()).collect());

        let context = if relevant.is_empty() {
            String::new()
        } else {
            relevant
                .into_iter()
                .map(|chunk| format!("Source: {}\n{}", chunk.source_label, chunk.text))
                .collect::<Vec<_>>()
                .join("\n\n---\n\n")
        };

        Ok((context, sources, count))
    }
}
