use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

use crate::config::AppConfig;

/// Create database connection pool
pub async fn create_pool(config: &AppConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&config.database_url)
        .await?;
    
    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    // For production, use sqlx migrate
    // For now, migrations are run automatically by PostgreSQL init script in Docker
    
    // Verify connection
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;
    
    tracing::info!("Database connected successfully: {:?}", row);
    Ok(())
}
