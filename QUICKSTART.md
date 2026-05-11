# Quick Start Guide

Get the Saudi Market AI platform running in 5 minutes!

## Prerequisites Checklist

Before you start, make sure you have:

- [ ] **Docker Desktop** installed and running
  - Download: https://www.docker.com/products/docker-desktop

- [ ] **Rust** installed
  - Windows: https://rustup.rs/
  - Or use: `winget install Rustlang.Rustup`

- [ ] **Node.js 18+** installed
  - Download: https://nodejs.org/
  - Or use: `winget install OpenJS.NodeJS.LTS`

- [ ] **5 API Keys** obtained (see list below)

## API Keys You Need

| Service | Key Name | Where to Get | Cost |
|---------|----------|--------------|------|
| **OpenAI** | `OPENAI_API_KEY` | https://platform.openai.com/api-keys | ~$0.10 per 1M tokens |
| **Google Gemini** | `GEMINI_API_KEY` | https://aistudio.google.com/app/apikey | Free tier: 15 req/min |
| **Anthropic Claude** | `ANTHROPIC_API_KEY` | https://console.anthropic.com/settings/keys | ~$3 per study |
| **Tavily** | `TAVILY_API_KEY` | https://app.tavily.com/home | Free: 1000 req/month |
| **Google Places** | `GOOGLE_PLACES_API_KEY` | https://developers.google.com/maps/documentation/places/web-service/get-api-key | $5 per 1000 requests |

## Step-by-Step Setup

### Step 1: Clone/Navigate to Project (1 minute)

```powershell
# Navigate to the project folder
cd C:\Users\ab5\CascadeProjects\saudi-market-feasibility
```

### Step 2: Configure API Keys (2 minutes)

```powershell
# Go to backend folder
cd backend

# Copy the example environment file
copy .env.example .env

# Open .env in your editor (VS Code)
code .env

# Or use Notepad
notepad .env
```

**Edit the `.env` file with your actual API keys:**

```env
# Replace these with your actual keys
GEMINI_API_KEY=AIzaSyCyour_actual_key_here
ANTHROPIC_API_KEY=sk-ant-api03-your_actual_key_here
OPENAI_API_KEY=sk-proj-your_actual_key_here
TAVILY_API_KEY=tvly-your_actual_key_here
GOOGLE_PLACES_API_KEY=AIzaSyCyour_actual_key_here

# These are already set correctly for local development
DATABASE_URL=postgres://postgres:postgres@localhost:5432/saudi_market_ai
QDRANT_URL=http://localhost:6333
RUST_LOG=info
```

**Save and close the file.**

### Step 3: Start Infrastructure (1 minute)

```powershell
# Go back to project root
cd ..

# Start PostgreSQL and Qdrant
docker-compose up -d postgres qdrant

# Wait 30 seconds for services to start
Start-Sleep -Seconds 30

# Check if services are running
docker-compose ps
```

You should see:
- `saudi_market_postgres` running
- `saudi_market_qdrant` running

### Step 4: Start Backend (2 minutes)

Open a **new terminal window**:

```powershell
# Navigate to backend
cd C:\Users\ab5\CascadeProjects\saudi-market-feasibility\backend

# Build and run (first time takes ~2-3 minutes)
cargo run
```

Wait for the message:
```
🚀 Server running at http://0.0.0.0:3001
```

**Keep this terminal running!**

### Step 5: Start Frontend (2 minutes)

Open a **third terminal window**:

```powershell
# Navigate to frontend
cd C:\Users\ab5\CascadeProjects\saudi-market-feasibility\frontend

# Install dependencies (first time only, takes ~2 minutes)
npm install

# Run development server
npm run dev
```

Wait for:
```
ready - started server on 0.0.0.0:3000
```

**Keep this terminal running!**

### Step 6: Test! (1 minute)

Open your browser:
```
http://localhost:3000
```

You should see the Saudi Market AI landing page with a form.

## Quick Test

### Option A: Use the Web Interface
1. Fill out the 4-step form
2. Submit
3. Wait 30-60 seconds for AI analysis

### Option B: Use Test Script

```powershell
# In a new terminal
cd C:\Users\ab5\CascadeProjects\saudi-market-feasibility
.\scripts\test-api.ps1
```

### Option C: Manual curl Test

```powershell
# Test health
curl http://localhost:3001/health

# Test feasibility study
curl -X POST http://localhost:3001/api/rag-study `
  -H "Content-Type: application/json" `
  -d '{"business_name":"Test Coffee","description":"Coffee shop in Riyadh","target_city":"Riyadh","capital_budget":400000,"industry":"food","business_model":"brick_and_mortar","initial_employees":6,"founder_experience":"intermediate","contact_email":"test@example.com","include_competitor_analysis":true,"include_persona_debate":true}'
```

## Troubleshooting

### Issue: "API key not configured" error

**Solution:** Check your `.env` file
```powershell
cat backend/.env
# Make sure keys are not empty
```

### Issue: "Connection refused" to database

**Solution:** Start Docker services
```powershell
docker-compose up -d postgres qdrant
docker-compose ps  # Check status
```

### Issue: Port already in use

**Solution:** Kill processes using those ports
```powershell
# Find what's using port 3000
netstat -ano | findstr :3000

# Kill the process (replace PID)
taskkill /PID <PID> /F
```

### Issue: cargo build fails

**Solution:** Update Rust and clean build
```powershell
cd backend
rustup update
cargo clean
cargo build
```

### Issue: npm install fails

**Solution:** Clear cache and retry
```powershell
cd frontend
Remove-Item -Recurse -Force node_modules
Remove-Item package-lock.json
npm cache clean --force
npm install
```

## What's Running Where?

| Service | URL | Port | Terminal |
|---------|-----|------|----------|
| Frontend | http://localhost:3000 | 3000 | Terminal 3 |
| Backend API | http://localhost:3001 | 3001 | Terminal 2 |
| PostgreSQL | localhost | 5432 | Docker |
| Qdrant | http://localhost:6333 | 6333 | Docker |

## File Locations

| What | Where |
|------|-------|
| **API Keys** | `backend/.env` |
| **Government Docs** | `documents/01-government/` |
| **Backend Code** | `backend/src/` |
| **Frontend Code** | `frontend/src/` |
| **Database Migrations** | `backend/migrations/` |

## Next Steps After Setup

1. **Add More Documents:**
   - Put Saudi government PDFs in `documents/01-government/[authority]/`
   - The AI will use these for citations

2. **Customize Frontend:**
   - Edit `frontend/src/app/page.tsx` for landing page
   - Edit `frontend/src/components/multi-step-form.tsx` for form

3. **Test All Features:**
   ```powershell
   .\scripts\test-api.ps1
   ```

4. **Read Documentation:**
   - Full guide: `README.md`
   - API docs: `API_DOCUMENTATION.md`
   - Setup details: `SETUP_COMPLETE.md`

## Cost Estimates

Running the system costs approximately:
- **Per feasibility study:** $0.50 - $2.50
- **Per persona debate:** $0.30 - $1.00
- **Per competitor analysis:** $0.10 - $0.50

**Example:** 10 studies/month = ~$15-40

## Shutting Down

To stop everything:

```powershell
# 1. Stop frontend (Terminal 3)
# Press Ctrl+C in the terminal

# 2. Stop backend (Terminal 2)
# Press Ctrl+C in the terminal

# 3. Stop infrastructure
docker-compose down

# Or stop everything including data:
docker-compose down -v
```

## Getting Help

If you're stuck:

1. Check the logs:
   ```powershell
   docker-compose logs backend
   docker-compose logs postgres
   ```

2. Verify services:
   ```powershell
   curl http://localhost:3001/health
   curl http://localhost:3000
   docker-compose ps
   ```

3. Review documentation in `README.md`

4. Check all API keys are set correctly in `backend/.env`

## Success! 🎉

If you can see the page at http://localhost:3000, you're all set!

The platform is ready to:
- Generate AI-powered feasibility studies
- Run virtual persona debates
- Analyze real competitors
- Cite Saudi government documents

**Start testing with a business idea!**
