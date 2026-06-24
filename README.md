# Saudi Market AI - Feasibility Study Platform

Saudi Market AI is a SaaS platform for generating data-informed feasibility studies for business ideas in Saudi Arabia. It includes a Rust API, a Next.js frontend, RAG over local documents, and Gemini-based generation.

## Features

- Feasibility study generation with structured JSON output
- Persona debate simulation
- Competitor analysis using Tavily plus Google Places
- RAG citations from local government and regulatory documents
- Qdrant vector search for document retrieval

## Repository Layout

```
saudi-market-feasibility/
├── backend/              # Rust Axum API (port 3001)
├── frontend/             # Next.js app (port 3000)
├── documents/            # RAG source documents
├── shared-types/         # Shared TypeScript types
├── docker-compose.yml    # Qdrant
└── README.md
```

## Quick Start (TL;DR)

```bash
# 1) Configure API keys
cp backend/.env.example backend/.env

# 2) Start infrastructure
docker-compose up -d qdrant

# 3) Run backend
cd backend
cargo run

# 4) Run frontend
cd ../frontend
npm install
npm run dev

# 5) Open the app
http://localhost:3000
```

## Prerequisites

- Rust (cargo)
- Node.js 18+ and npm
- Docker Desktop
- Python 3.10+ (for tests)

## Configuration

All backend keys live in `backend/.env`.

Required keys:

- `GEMINI_API_KEY`
- `TAVILY_API_KEY`
- `GOOGLE_PLACES_API_KEY`

Optional overrides:

- `GEMINI_MODEL` (default `gemini-flash-latest`)
- `GEMINI_EMBEDDING_MODEL` (default `text-embedding-004`)

Example `backend/.env`:

```env
GEMINI_API_KEY=your_gemini_key
GEMINI_MODEL=gemini-flash-latest
GEMINI_EMBEDDING_MODEL=text-embedding-004
TAVILY_API_KEY=your_tavily_key
GOOGLE_PLACES_API_KEY=your_places_key

QDRANT_URL=http://localhost:6333
ENVIRONMENT=development
```

Frontend config (optional):

```env
NEXT_PUBLIC_API_URL=http://localhost:3001/api
```

## Running Locally

Start infrastructure:

```bash
docker-compose up -d qdrant
```

Run backend:

```bash
cd backend
cargo run
```

Run frontend:

```bash
cd frontend
npm install
npm run dev
```

## API Endpoints

Base URL: `http://localhost:3001`

- `GET /health`
- `POST /api/rag-study`
- `POST /api/personas`
- `POST /api/competitors`
- `POST /api/chat` — RAG-based document Q&A

Example chat request:

```bash
curl -X POST http://localhost:3001/api/chat \
  -H "Content-Type: application/json" \
  -d '{"question": "What licenses are required to open a coffee shop in Jeddah?"}'
```

Example feasibility study request:

```bash
curl -X POST http://localhost:3001/api/rag-study \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Al-Rashid Coffee",
    "description": "Premium coffee shop targeting young professionals in Jeddah.",
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
```

## Documents and RAG

The RAG system searches local documents in `documents/` and cites them in results.

Directory structure:

```
documents/
├── 01-government/
│   ├── monshaat/
│   ├── qiwa/
│   ├── balady/
│   ├── gosi/
│   ├── ministry_of_commerce/
│   └── zatca/
├── 02-feasibility-templates/
├── 03-regulations/
└── 04-research/
```

Supported formats:

- `.txt` (recommended)
- `.md`
- `.pdf` (text-based PDFs supported; scanned PDFs may require OCR)

Suggested metadata header for text files:

```
TITLE: Commercial Registration Requirements
AUTHORITY: Ministry of Commerce
URL: https://mc.gov.sa
DATE: 2024-01-15

[content]
```

If you switch embedding models, delete the existing Qdrant collection to recreate it with the new vector size.

## PDF Notes

PDF extraction is enabled. If a PDF is scanned or image-based, text extraction may return empty; convert to `.txt` or run OCR before adding it to `documents/`.

## Tests (Python)

Integration tests are in `backend/tests` and require the backend to be running.

```bash
python -m pip install -r backend/tests/requirements.txt
RUN_API_TESTS=1 pytest backend/tests
```

You can override the API base URL:

```bash
API_BASE_URL=http://localhost:3001 RUN_API_TESTS=1 pytest backend/tests
```

## Troubleshooting

- API key errors: confirm `backend/.env` exists and all keys are set.
- Port conflicts: check ports 3000, 3001, 6333.
- Qdrant errors after model change: delete the collection and restart.

## Production Notes

- Set `ENVIRONMENT=production`.
- Point `NEXT_PUBLIC_API_URL` to your hosted backend.

## Costs

Costs depend on Gemini usage plus Tavily and Google Places. Track API usage in each provider dashboard and budget for both generation and embeddings.
