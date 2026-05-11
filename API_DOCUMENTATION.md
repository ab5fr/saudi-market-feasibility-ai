# Saudi Market AI - API Documentation

## Base URL
```
http://localhost:3001
```

## Authentication
Currently, the API does not require authentication (for MVP phase). Rate limiting may be implemented in production.

## Content-Type
All requests should include:
```
Content-Type: application/json
```

---

## Endpoints

### 1. Health Check
Check if the API is running.

**Endpoint:** `GET /health`

**Response:**
```json
"✅ Saudi Market AI Backend is healthy"
```

**Example:**
```bash
curl http://localhost:3001/health
```

---

### 2. Generate Feasibility Study
Generates a comprehensive RAG-based feasibility study for a business idea.

**Endpoint:** `POST /api/rag-study`

**Request Body:**
```json
{
  "business_name": "Al-Rashid Coffee",
  "description": "A specialty coffee shop targeting young professionals in Jeddah. We serve premium arabica coffee and fresh pastries.",
  "target_city": "Jeddah",
  "district": "Al-Andalus",  // Optional
  "capital_budget": 500000,
  "industry": "food",
  "business_model": "brick_and_mortar",
  "initial_employees": 8,
  "founder_experience": "intermediate",
  "contact_email": "founder@example.com",
  "specific_questions": ["What are the licensing requirements?"],  // Optional
  "include_competitor_analysis": true,
  "include_persona_debate": true
}
```

**Required Fields:**
- `business_name` (string, 3-200 chars)
- `description` (string, 50-5000 chars)
- `target_city` (string)
- `capital_budget` (number, 1000-1000000000)
- `industry` (string)
- `business_model` (enum: brick_and_mortar, ecommerce, hybrid, service_based, b2b, marketplace, subscription)
- `initial_employees` (integer, 1-10000)
- `founder_experience` (enum: beginner, intermediate, experienced, expert)
- `contact_email` (valid email)
- `include_competitor_analysis` (boolean)
- `include_persona_debate` (boolean)

**Response:**
```json
{
  "success": true,
  "data": {
    "study_id": "study_a1b2c3d4e5f6",
    "business_name": "Al-Rashid Coffee",
    "generated_at": "2024-05-07T10:30:00Z",
    "executive_summary": {
      "viability_score": 78.5,
      "summary_text": "The coffee shop shows strong viability...",
      "key_strengths": ["Prime location", "Growing market"],
      "key_challenges": ["High rent costs", "Competition"],
      "time_to_break_even_months": 14
    },
    "market_analysis": {
      "target_market_size": "SAR 450M annually",
      "market_growth_rate": "12% CAGR",
      "customer_segments": [...],
      "competitive_landscape": "Moderately competitive...",
      "market_entry_barriers": ["High initial investment", ...]
    },
    "financial_projections": {
      "initial_investment_breakdown": [...],
      "monthly_operating_costs": [...],
      "revenue_projections": {...},
      "profitability_timeline": "Break-even expected month 14",
      "roi_estimate_3yr": 1.45
    },
    "legal_requirements": {
      "business_structure_options": [...],
      "required_licenses": [...],
      "regulatory_compliance": [...],
      "estimated_setup_costs_sar": 25000,
      "setup_timeline_weeks": 6
    },
    "risk_assessment": {
      "market_risks": [...],
      "financial_risks": [...],
      "operational_risks": [...],
      "regulatory_risks": [...],
      "mitigation_strategies": [...]
    },
    "recommendations": {
      "go_no_go_verdict": "yes",
      "critical_success_factors": [...],
      "next_steps": [...],
      "suggested_partnerships": [...]
    },
    "sources_cited": [
      {
        "document_name": "SME Funding Guide",
        "authority": "Monsha'at",
        "url": "https://monshaat.gov.sa",
        "citation_text": "SMEs represent 99.5% of business entities",
        "relevance_score": 0.95
      }
    ]
  },
  "error": null,
  "timestamp": "2024-05-07T10:30:00Z"
}
```

**Example:**
```bash
curl -X POST http://localhost:3001/api/rag-study \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Tech Solutions LLC",
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

---

### 3. Persona Debate
Initiates an AI persona debate where multiple Saudi demographic personas critique the business idea.

**Endpoint:** `POST /api/personas`

**Request Body:**
```json
{
  "business_name": "Al-Rashid Coffee",
  "description": "A specialty coffee shop in Jeddah...",
  "target_city": "Jeddah",
  "capital_budget": 500000,
  "industry": "food",
  "business_model": "brick_and_mortar",
  "initial_employees": 8,
  "founder_experience": "intermediate",
  "contact_email": "test@example.com",
  "include_competitor_analysis": false,
  "include_persona_debate": true
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "session_id": "sess_a1b2c3d4e5f6",
    "business_name": "Al-Rashid Coffee",
    "personas": [
      {
        "id": "inv_001",
        "name": "Abdullah Al-Rashid",
        "demographic_profile": "Experienced angel investor, Riyadh-based...",
        "role_description": "ROI-focused investor perspective"
      },
      {
        "id": "stu_001",
        "name": "Fatima Al-Zahrani",
        "demographic_profile": "University student, Jeddah, age 22...",
        "role_description": "Young consumer perspective"
      },
      {
        "id": "biz_001",
        "name": "Khalid Al-Otaibi",
        "demographic_profile": "Small business owner, 15 years experience...",
        "role_description": "Established business owner perspective"
      },
      {
        "id": "gov_001",
        "name": "Sara Al-Qahtani",
        "demographic_profile": "Former SAGIA advisor...",
        "role_description": "Regulatory perspective"
      }
    ],
    "debate_transcript": [
      {
        "turn_number": 1,
        "persona_id": "inv_001",
        "persona_name": "Abdullah Al-Rashid",
        "message": "I've reviewed this business. The capital structure seems reasonable...",
        "sentiment": "skeptical",
        "concerns_raised": ["Payback period", "CAC unclear"]
      }
    ],
    "consensus_summary": "The virtual Saudi audience sees potential but raises concerns...",
    "key_risks": ["Market saturation", "High initial costs"],
    "key_opportunities": ["Growing demand", "Vision 2030 alignment"],
    "overall_verdict": "yes"
  }
}
```

**Verdict Options:**
- `strong_yes` - Highly recommended
- `yes` - Recommended
- `neutral` - Proceed with caution
- `no` - Not recommended
- `strong_no` - Strongly not recommended

**Sentiment Options:**
- `supportive` - Positive toward idea
- `skeptical` - Questioning, needs convincing
- `neutral` - Balanced view
- `concerned` - Worried about specific issues
- `enthusiastic` - Very positive

**Example:**
```bash
curl -X POST http://localhost:3001/api/personas \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Test Business",
    "description": "Description here...",
    "target_city": "Riyadh",
    "capital_budget": 400000,
    "industry": "retail",
    "business_model": "brick_and_mortar",
    "initial_employees": 6,
    "founder_experience": "intermediate",
    "contact_email": "test@example.com",
    "include_competitor_analysis": false,
    "include_persona_debate": true
  }'
```

---

### 4. Competitor Analysis
Analyzes real competitors in the specified Saudi city/district using Google Places and web search.

**Endpoint:** `POST /api/competitors`

**Request Body:**
```json
{
  "business_name": "Al-Rashid Fashion",
  "description": "Modest clothing retail for women",
  "target_city": "Riyadh",
  "district": "Al-Olaya",  // Optional
  "capital_budget": 600000,
  "industry": "retail",
  "business_model": "brick_and_mortar",
  "initial_employees": 8,
  "founder_experience": "intermediate",
  "contact_email": "test@example.com",
  "include_competitor_analysis": true,
  "include_persona_debate": false
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "analysis_id": "comp_a1b2c3d4e5f6",
    "business_name": "Al-Rashid Fashion",
    "search_location": "Al-Olaya, Riyadh",
    "search_query_used": "retail businesses near Al-Olaya, Riyadh",
    "competitors": [
      {
        "name": "Competitor Name",
        "location": "Al-Olaya District, Riyadh",
        "distance_km": 2.3,
        "business_type": "Direct Competitor",
        "rating": 4.2,
        "review_count": 156,
        "price_level": 3,
        "website": "https://example.com",
        "phone": "+966 11 123 4567",
        "strengths": ["Established brand", "Prime location"],
        "weaknesses": ["Limited digital presence"],
        "threat_level": "direct_competitor"
      }
    ],
    "market_saturation_score": 65.5,
    "market_gap_analysis": "Analysis reveals a gap in the mid-tier segment...",
    "pricing_benchmarks": {
      "average_price_range": "SAR 150 - 450",
      "lowest_observed": "SAR 120",
      "highest_observed": "SAR 650",
      "pricing_strategy_recommendation": "Position at SAR 200-350 range..."
    },
    "online_presence_summary": {
      "total_competitors_found": 12,
      "avg_google_rating": 4.1,
      "competitors_with_websites": 8,
      "social_media_presence": "Moderate activity on Instagram",
      "online_reputation_summary": "Generally positive reviews..."
    }
  }
}
```

**Threat Level Options:**
- `low` - Minimal competitive threat
- `medium` - Moderate competition
- `high` - Strong competitor
- `direct_competitor` - Direct competition for same customers

**Example:**
```bash
curl -X POST http://localhost:3001/api/competitors \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Test Store",
    "description": "Fashion retail store",
    "target_city": "Jeddah",
    "capital_budget": 500000,
    "industry": "retail",
    "business_model": "brick_and_mortar",
    "initial_employees": 6,
    "founder_experience": "intermediate",
    "contact_email": "test@example.com",
    "include_competitor_analysis": true,
    "include_persona_debate": false
  }'
```

---

## Error Responses

All errors follow this format:

```json
{
  "success": false,
  "data": null,
  "error": "Error message describing what went wrong",
  "timestamp": "2024-05-07T10:30:00Z"
}
```

**Common Error Codes:**
- `400 Bad Request` - Invalid input data (validation failed)
- `401 Unauthorized` - Authentication required (when implemented)
- `500 Internal Server Error` - Server-side error
- `502 Bad Gateway` - External API error (AI services)
- `503 Service Unavailable` - Service temporarily unavailable

---

## Business Model Options

| Value | Description |
|-------|-------------|
| `brick_and_mortar` | Physical store/location |
| `ecommerce` | Online-only business |
| `hybrid` | Both physical and online |
| `service_based` | Service-oriented business |
| `b2b` | Business-to-business |
| `marketplace` | Platform connecting buyers/sellers |
| `subscription` | Recurring revenue model |

---

## Experience Level Options

| Value | Description |
|-------|-------------|
| `beginner` | No prior experience |
| `intermediate` | Some experience |
| `experienced` | Significant experience |
| `expert` | Industry veteran |

---

## Valid Industries

The following industries are commonly used:
- `food` - Food & Beverage
- `retail` - Retail stores
- `tech` - Technology/IT
- `healthcare` - Medical/Health
- `education` - Education/Training
- `construction` - Construction
- `manufacturing` - Manufacturing
- `logistics` - Logistics/Transport
- `tourism` - Tourism/Hospitality
- `other` - Other industries

---

## Rate Limiting

Currently, no rate limiting is enforced. In production:
- 100 requests per minute per IP
- 1000 requests per day per API key (when auth implemented)

---

## Testing

Use the provided test scripts:

**PowerShell:**
```powershell
.\scripts\test-api.ps1
```

**Bash:**
```bash
chmod +x scripts/test-api.sh
./scripts/test-api.sh
```

---

## Support

For API issues:
1. Check backend is running: `curl http://localhost:3001/health`
2. Verify API keys are configured in `backend/.env`
3. Check backend logs for detailed error messages
4. Review request payload matches expected schema
