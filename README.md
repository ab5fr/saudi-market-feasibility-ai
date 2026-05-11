# Saudi Market AI - Feasibility Study Platform

An innovative SaaS platform tailored to the Saudi Arabian market that helps entrepreneurs test their business ideas and generate real, data-driven feasibility studies.

## Quick Start (TL;DR)

```bash
# 1. Set your API keys (see "Where to Put API Keys" section below)
cp backend/.env.example backend/.env
# Edit backend/.env with your actual API keys

# 2. Start the infrastructure
docker-compose up -d postgres qdrant

# 3. Build and start backend (in a new terminal)
cd backend
cargo run

# 4. Build and start frontend (in a new terminal)
cd frontend
npm install
npm run dev

# 5. Open http://localhost:3000 and test!
```

## Where to Put API Keys (CRITICAL)

All API keys go in `backend/.env` file:

```bash
# Create from template
cd backend
cp .env.example .env

# Edit .env with your keys:
nano .env  # or use VS Code: code .env
```

### Required API Keys:

| Service | Key Name | Where to Get | Cost |
|---------|----------|--------------|------|
| **OpenAI** | `OPENAI_API_KEY` | https://platform.openai.com/api-keys | ~$0.10 per 1M tokens for embeddings |
| **Google Gemini** | `GEMINI_API_KEY` | https://aistudio.google.com/app/apikey | Free tier: 15 requests/min |
| **Anthropic Claude** | `ANTHROPIC_API_KEY` | https://console.anthropic.com/settings/keys | ~$3 per feasibility study |
| **Tavily** | `TAVILY_API_KEY` | https://app.tavily.com/home | Free: 1000 requests/month |
| **Google Places** | `GOOGLE_PLACES_API_KEY` | https://developers.google.com/maps/documentation/places/web-service/get-api-key | $5 per 1000 requests |

### Example .env file:
```env
# AI Model API Keys (REQUIRED)
GEMINI_API_KEY=AIzaSyCxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
ANTHROPIC_API_KEY=sk-ant-api03-xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
OPENAI_API_KEY=sk-proj-xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
TAVILY_API_KEY=tvly-xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
GOOGLE_PLACES_API_KEY=AIzaSyCxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# Database (automatically set by Docker)
DATABASE_URL=postgres://postgres:postgres@localhost:5432/saudi_market_ai

# Vector Database (automatically set by Docker)
QDRANT_URL=http://localhost:6333

# Security
JWT_SECRET=your-secret-key-change-in-production

# Environment
ENVIRONMENT=development
```

## Where to Put Documents (For RAG Citations)

Put your Saudi government documents here:

```
documents/
├── 01-government/
│   ├── monshaat/          # SME authority docs
│   ├── qiwa/              # Labor market docs
│   ├── balady/            # Municipal license docs
│   ├── gosi/              # Social insurance docs
│   ├── ministry_of_commerce/  # Commercial registration docs
│   └── zatca/             # Tax authority docs
├── 02-feasibility-templates/
└── 03-regulations/
```

**See `documents/README.md` for detailed instructions.**

### Quick Document Setup:
```bash
# Create a sample document
cat > documents/01-government/monshaat/sme_funding.txt << 'EOF'
TITLE: SME Funding Programs
AUTHORITY: Monsha'at
DATE: 2024

## Overview
Monsha'at offers various funding programs for SMEs...

## Eligibility
- Commercial Registration required
- Annual revenue under 375M SAR
...
EOF
```

## Architecture Overview

### Monorepo Structure
```
saudi-market-feasibility/
├── backend/              # Rust Axum API (Port 3001)
│   ├── src/
│   │   ├── main.rs       # Server entry point
│   │   ├── routes/       # API endpoints
│   │   ├── services/     # AI service integrations
│   │   ├── models/       # Data models
│   │   └── migrations/   # Database schema
│   └── .env              # API KEYS GO HERE
├── frontend/             # Next.js 14 (Port 3000)
│   ├── src/
│   │   ├── app/          # Next.js App Router
│   │   ├── components/   # React components
│   │   └── types/        # TypeScript types
│   └── .env.local        # Frontend config
├── documents/            # RAG source documents
├── docker-compose.yml    # Infrastructure orchestration
└── README.md             # This file
```

### AI Model Routing

| Task | Model | Provider | Purpose |
|------|-------|----------|---------|
| **RAG Document Analysis** | Gemini 1.5 Pro | Google AI Studio | 1M+ token context for Arabic PDFs |
| **Feasibility Study JSON** | Claude 3.5 Sonnet | Anthropic | Structured output, financial calculations |
| **Vector Embeddings** | text-embedding-3-large | OpenAI | Document embeddings (3072-dim) |
| **Web Search** | Tavily API | Tavily | Clean competitor research |
| **Local Competitors** | Places API | Google Maps | Real business ratings & reviews |

## Complete Setup Instructions

### Prerequisites

Install these tools:

```bash
# 1. Rust (with cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Node.js 18+ and npm
# Download from https://nodejs.org/

# 3. Docker & Docker Compose
# Download from https://www.docker.com/products/docker-desktop
```

### Step 1: Clone and Setup

```bash
# Navigate to project
cd saudi-market-feasibility

# Setup backend environment
cd backend
cp .env.example .env
# Edit .env with your API keys!

# Setup frontend
cd ../frontend
cp .env.local.example .env.local
```

### Step 2: Start Infrastructure (Docker)

```bash
# From project root
docker-compose up -d postgres qdrant

# Verify services are running
docker-compose ps

# Check logs if needed
docker-compose logs -f postgres
docker-compose logs -f qdrant
```

This starts:
- PostgreSQL on port 5432
- Qdrant (vector DB) on port 6333

### Step 3: Run Backend

```bash
cd backend

# Install dependencies (first time)
cargo build

# Run server
cargo run

# Server will start at http://localhost:3001
```

**Expected output:**
```
🚀 Server running at http://0.0.0.0:3001
📊 Health check: GET http://0.0.0.0:3001/health
🎭 Personas API: POST http://0.0.0.0:3001/api/personas
📋 RAG Study API: POST http://0.0.0.0:3001/api/rag-study
🔍 Competitors API: POST http://0.0.0.0:3001/api/competitors
```

### Step 4: Run Frontend

```bash
cd frontend

# Install dependencies (first time, takes ~2 minutes)
npm install

# Run dev server
npm run dev

# Frontend will start at http://localhost:3000
```

### Step 5: Test the Application

1. **Open browser:** http://localhost:3000
2. **Fill out the multi-step form:**
   - Step 1: Business name, description, industry
   - Step 2: City (e.g., Riyadh, Jeddah)
   - Step 3: Budget, employees, experience
   - Step 4: Email and analysis options
3. **Submit** and wait for AI analysis

## API Endpoints

### Test with curl

```bash
# Health check
curl http://localhost:3001/health

# Test persona debate
curl -X POST http://localhost:3001/api/personas \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Al-Rashid Coffee",
    "description": "Premium coffee shop serving specialty arabica coffee in Jeddah. Targeting young professionals and students.",
    "target_city": "Jeddah",
    "capital_budget": 500000,
    "industry": "food",
    "business_model": "brick_and_mortar",
    "initial_employees": 8,
    "founder_experience": "intermediate",
    "contact_email": "test@example.com",
    "include_competitor_analysis": true,
    "include_persona_debate": true
  }'

# Test feasibility study
curl -X POST http://localhost:3001/api/rag-study \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Tech Solutions",
    "description": "IT consulting and software development for Saudi SMEs",
    "target_city": "Riyadh",
    "capital_budget": 300000,
    "industry": "tech",
    "business_model": "service_based",
    "initial_employees": 5,
    "founder_experience": "experienced",
    "contact_email": "test@example.com",
    "include_competitor_analysis": true,
    "include_persona_debate": false
  }'
```

## Document Processing (RAG Setup)

### Adding Documents for AI Citations

1. **Create text files** in the `documents/` folder:
```bash
cat > documents/01-government/monshaat/sme_guide.txt << 'EOF'
TITLE: SME Startup Guide
AUTHORITY: Monsha'at
DATE: 2024

## Commercial Registration
Required documents:
- ID copy
- Lease contract
- Activity code

Processing time: 3 days
Fee: SAR 500
EOF
```

2. **Documents are automatically processed** when the backend starts

3. **The AI will cite these documents** in feasibility studies

### Processing Status

Check logs to see document processing:
```bash
docker-compose logs -f backend | grep -i "document\|embedding"
```

## Troubleshooting

### Common Issues

**1. "API key not configured" error**
```bash
# Solution: Check your .env file
cat backend/.env
# Make sure keys are set and not empty
```

**2. Database connection failed**
```bash
# Solution: Start Docker services
docker-compose up -d postgres

# Check if PostgreSQL is running
docker-compose ps
```

**3. Port already in use**
```bash
# Solution: Kill processes on ports 3000, 3001, 5432, 6333
# Or change ports in configuration
```

**4. npm install fails**
```bash
# Solution: Clear npm cache and try again
cd frontend
rm -rf node_modules package-lock.json
npm cache clean --force
npm install
```

**5. Rust build fails**
```bash
# Solution: Update Rust and clean build
cd backend
rustup update
cargo clean
cargo build
```

### Getting API Keys

**OpenAI (Required for embeddings):**
1. Go to https://platform.openai.com/api-keys
2. Create new secret key
3. Copy to `backend/.env`: `OPENAI_API_KEY=sk-...`

**Google Gemini (Required for RAG):**
1. Go to https://aistudio.google.com/app/apikey
2. Click "Create API Key"
3. Copy to `backend/.env`: `GEMINI_API_KEY=AIza...`

**Anthropic Claude (Required for feasibility studies):**
1. Go to https://console.anthropic.com/settings/keys
2. Create new key
3. Copy to `backend/.env`: `ANTHROPIC_API_KEY=sk-ant...`

**Tavily (Required for web search):**
1. Go to https://app.tavily.com/home
2. Sign up and get API key
3. Copy to `backend/.env`: `TAVILY_API_KEY=tvly-...`

**Google Places (Required for local competitors):**
1. Go to https://developers.google.com/maps/documentation/places/web-service/get-api-key
2. Create project in Google Cloud Console
3. Enable Places API
4. Create credentials → API Key
5. Copy to `backend/.env`: `GOOGLE_PLACES_API_KEY=AIza...`

## Development Mode (Without Docker)

If you prefer running without Docker:

```bash
# 1. Install PostgreSQL locally (version 15+)
# Download from https://www.postgresql.org/download/

# 2. Install Qdrant locally
# See https://qdrant.tech/documentation/guides/installation/

# 3. Update backend/.env
DATABASE_URL=postgres://postgres:password@localhost:5432/saudi_market_ai
QDRANT_URL=http://localhost:6333

# 4. Run services manually
cargo run  # In backend directory
npm run dev  # In frontend directory
```

## Production Deployment

### Using Docker Compose (Recommended)

```bash
# Build production images
docker-compose -f docker-compose.yml -f docker-compose.prod.yml build

# Start all services
docker-compose up -d
```

### Environment Variables for Production

```env
# backend/.env
ENVIRONMENT=production
DATABASE_URL=postgres://postgres:securepassword@postgres:5432/saudi_market_ai
QDRANT_URL=http://qdrant:6333
JWT_SECRET=your-256-bit-secret-key-here

# frontend/.env.local
NEXT_PUBLIC_API_URL=https://api.yourdomain.com
```

## Cost Estimates

Running this application incurs API costs:

| Service | Cost per Feasibility Study | Free Tier |
|---------|---------------------------|-----------|
| OpenAI Embeddings | ~$0.02 | N/A |
| Claude 3.5 Sonnet | ~$0.50-2.00 | N/A |
| Gemini 1.5 Pro | ~$0.05 | 15 req/min |
| Tavily Search | ~$0.01 | 1000 req/mo |
| Google Places | ~$0.05 | $200 credit |

**Estimated cost per full analysis:** ~$0.50 - $2.50

## Project Status

✅ **Implemented:**
- Docker Compose infrastructure (PostgreSQL + Qdrant)
- Database schema with migrations
- All AI service integrations (Gemini, Claude, OpenAI, Tavily, Places)
- Document ingestion and RAG pipeline
- Multi-step form frontend (Next.js + Shadcn/UI)
- API routes with real AI integration

🚧 **TODO (Next Steps):**
- PDF extraction for document processing
- User authentication system
- Results dashboard with study history
- Payment/subscription system
- Email notifications

## Support

For issues or questions:
1. Check the Troubleshooting section above
2. Review logs: `docker-compose logs`
3. Check API status at http://localhost:3001/health
4. Verify API keys are set correctly

## License

MIT License - See LICENSE file

---

**Built for the Saudi Arabian market with ❤️**
