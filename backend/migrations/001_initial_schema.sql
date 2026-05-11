-- Saudi Market AI - Initial Database Schema
-- This will be auto-executed by PostgreSQL on first container start

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Feasibility projects table
CREATE TABLE IF NOT EXISTS feasibility_projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    business_name VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    target_city VARCHAR(100) NOT NULL,
    district VARCHAR(100),
    capital_budget DECIMAL(15, 2) NOT NULL,
    industry VARCHAR(50) NOT NULL,
    business_model VARCHAR(50) NOT NULL,
    initial_employees INTEGER NOT NULL,
    founder_experience VARCHAR(20) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Feasibility studies results
CREATE TABLE IF NOT EXISTS feasibility_studies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES feasibility_projects(id) ON DELETE CASCADE,
    viability_score DECIMAL(4, 2),
    executive_summary JSONB,
    market_analysis JSONB,
    financial_projections JSONB,
    legal_requirements JSONB,
    risk_assessment JSONB,
    recommendations JSONB,
    sources_cited JSONB,
    generated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Persona debates
CREATE TABLE IF NOT EXISTS persona_debates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES feasibility_projects(id) ON DELETE CASCADE,
    session_id VARCHAR(50) UNIQUE NOT NULL,
    personas JSONB NOT NULL,
    debate_transcript JSONB NOT NULL,
    consensus_summary TEXT,
    key_risks JSONB,
    key_opportunities JSONB,
    overall_verdict VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Competitor analyses
CREATE TABLE IF NOT EXISTS competitor_analyses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES feasibility_projects(id) ON DELETE CASCADE,
    analysis_id VARCHAR(50) UNIQUE NOT NULL,
    search_location VARCHAR(255),
    search_query_used VARCHAR(255),
    competitors JSONB,
    market_saturation_score DECIMAL(4, 2),
    market_gap_analysis TEXT,
    pricing_benchmarks JSONB,
    online_presence_summary JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Document store for RAG (metadata only, actual files on disk)
CREATE TABLE IF NOT EXISTS documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename VARCHAR(255) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size BIGINT,
    mime_type VARCHAR(100),
    document_type VARCHAR(50), -- 'government', 'feasibility_template', 'regulation', etc.
    authority VARCHAR(100), -- 'Monshaat', 'Qiwa', 'Balady', 'GOSI', etc.
    description TEXT,
    is_processed BOOLEAN DEFAULT FALSE,
    chunk_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Document chunks for RAG (links to Qdrant vectors)
CREATE TABLE IF NOT EXISTS document_chunks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID REFERENCES documents(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    chunk_text TEXT NOT NULL,
    qdrant_point_id VARCHAR(50), -- Reference to Qdrant vector
    token_count INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(document_id, chunk_index)
);

-- API request logs for monitoring
CREATE TABLE IF NOT EXISTS api_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    endpoint VARCHAR(255) NOT NULL,
    method VARCHAR(10) NOT NULL,
    request_body JSONB,
    response_status INTEGER,
    response_body JSONB,
    duration_ms INTEGER,
    user_id UUID REFERENCES users(id),
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_projects_user_id ON feasibility_projects(user_id);
CREATE INDEX idx_projects_status ON feasibility_projects(status);
CREATE INDEX idx_studies_project_id ON feasibility_studies(project_id);
CREATE INDEX idx_debates_project_id ON persona_debates(project_id);
CREATE INDEX idx_analyses_project_id ON competitor_analyses(project_id);
CREATE INDEX idx_documents_type ON documents(document_type);
CREATE INDEX idx_documents_authority ON documents(authority);
CREATE INDEX idx_api_logs_created_at ON api_logs(created_at);

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON feasibility_projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
