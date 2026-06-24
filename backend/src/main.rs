use axum::{
    Router,
    routing::{get, post},
};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    time::SystemTime,
};
use tokio::net::TcpListener;
use tokio::time::{Duration, interval};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod config;
mod models;
mod routes;
mod services;

use config::AppConfig;
use routes::{chat, competitors, personas, rag_study};
use services::document_service::DocumentService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing/logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .compact()
        .init();

    info!("Starting Saudi Market AI Backend...");

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("Configuration loaded successfully");

    // Ingest documents on startup
    tokio::spawn(ingest_documents(config.clone()));

    // Watch documents folder for changes
    tokio::spawn(watch_documents(config.clone()));

    // Build the router with all routes
    let app = create_router(config);

    // Define the address
    let addr: SocketAddr = "0.0.0.0:3001".parse()?;
    info!("Server will bind to {}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await?;
    info!("🚀 Server running at http://{}", addr);
    info!("📊 Health check: GET http://{}/health", addr);
    info!("💬 Chat API: POST http://{}/api/chat", addr);
    info!("🎭 Personas API: POST http://{}/api/personas", addr);
    info!("📋 RAG Study API: POST http://{}/api/rag-study", addr);
    info!("🔍 Competitors API: POST http://{}/api/competitors", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(config: AppConfig) -> Router {
    // CORS configuration for Next.js frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Core API routes
        .route("/api/chat", post(chat::answer_question))
        .route("/api/personas", post(personas::create_persona_debate))
        .route(
            "/api/rag-study",
            post(rag_study::generate_feasibility_study),
        )
        .route("/api/competitors", post(competitors::analyze_competitors))
        // Layers
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(cors)
        // Application state
        .with_state(config)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "✅ Saudi Market AI Backend is healthy"
}

/// Ingest demo documents into Qdrant on startup
async fn ingest_documents(config: AppConfig) {
    // Wait a moment for Qdrant to be ready
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let document_service = DocumentService::new(&config);

    match document_service.process_all_documents().await {
        Ok(documents) => {
            info!("✅ Ingested {} documents into Qdrant", documents.len());
        }
        Err(e) => {
            info!("⚠️ Failed to ingest documents: {}", e);
        }
    }
}

async fn watch_documents(config: AppConfig) {
    let document_service = DocumentService::new(&config);
    let interval_secs: u64 = std::env::var("DOCUMENTS_WATCH_INTERVAL_SECS")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(5);

    let mut known: HashMap<String, SystemTime> = HashMap::new();

    if let Ok(files) = document_service.list_documents().await {
        for file in files {
            if let Ok(metadata) = tokio::fs::metadata(&file).await
                && let Ok(modified) = metadata.modified()
            {
                known.insert(file, modified);
            }
        }
    }

    info!(
        "📂 Watching documents for changes every {} seconds",
        interval_secs
    );

    let mut ticker = interval(Duration::from_secs(interval_secs));

    loop {
        ticker.tick().await;

        let files = match document_service.list_documents().await {
            Ok(list) => list,
            Err(e) => {
                info!("⚠️ Failed to list documents: {}", e);
                continue;
            }
        };

        let mut seen: HashSet<String> = HashSet::new();

        for file in files {
            seen.insert(file.clone());

            let modified = match tokio::fs::metadata(&file)
                .await
                .and_then(|metadata| metadata.modified())
            {
                Ok(time) => time,
                Err(e) => {
                    info!("⚠️ Failed to stat document {}: {}", file, e);
                    continue;
                }
            };

            let needs_process = match known.get(&file) {
                None => true,
                Some(prev) => *prev < modified,
            };

            if needs_process {
                match document_service.process_path(&file).await {
                    Ok(_) => {
                        known.insert(file.clone(), modified);
                        info!("✅ Ingested updated document: {}", file);
                    }
                    Err(e) => {
                        info!("⚠️ Failed to ingest {}: {}", file, e);
                    }
                }
            }
        }

        known.retain(|file, _| seen.contains(file));
    }
}
