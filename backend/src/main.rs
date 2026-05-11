use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod db;
mod models;
mod routes;
mod services;
mod middleware;
mod utils;

use models::db_models;

use config::AppConfig;
use routes::{personas, rag_study, competitors};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing/logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .compact()
        .init();

    info!("Starting Saudi Market AI Backend...");

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("Configuration loaded successfully");

    // Build the router with all routes
    let app = create_router(config);

    // Define the address
    let addr: SocketAddr = "0.0.0.0:3001".parse()?;
    info!("Server will bind to {}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await?;
    info!("🚀 Server running at http://{}", addr);
    info!("📊 Health check: GET http://{}/health", addr);
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
        .route("/api/personas", post(personas::create_persona_debate))
        .route("/api/rag-study", post(rag_study::generate_feasibility_study))
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
