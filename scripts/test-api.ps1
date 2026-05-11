# Saudi Market AI - API Test Script
# Run this script to test if the backend APIs are working

$BASE_URL = "http://localhost:3001"

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Saudi Market AI - API Test Script" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Health Check
Write-Host "Test 1: Health Check" -ForegroundColor Yellow
Write-Host "---------------------"
try {
    $response = Invoke-RestMethod -Uri "$BASE_URL/health" -Method GET
    Write-Host "✅ Health check passed: $response" -ForegroundColor Green
} catch {
    Write-Host "❌ Health check failed: $_" -ForegroundColor Red
    Write-Host "Make sure backend is running on port 3001" -ForegroundColor Red
}
Write-Host ""

# Test 2: Feasibility Study
Write-Host "Test 2: Feasibility Study (RAG)" -ForegroundColor Yellow
Write-Host "---------------------"
$studyPayload = @{
    business_name = "Test Coffee Shop"
    description = "A specialty coffee shop targeting young professionals in Jeddah. We will serve premium arabica coffee, fresh pastries, and provide a comfortable workspace environment. The shop will focus on high-quality beans sourced from local Saudi roasters."
    target_city = "Jeddah"
    capital_budget = 450000
    industry = "food"
    business_model = "brick_and_mortar"
    initial_employees = 6
    founder_experience = "intermediate"
    contact_email = "test@example.com"
    include_competitor_analysis = $false
    include_persona_debate = $false
} | ConvertTo-Json

try {
    Write-Host "Sending request to /api/rag-study..." -ForegroundColor Gray
    $response = Invoke-RestMethod -Uri "$BASE_URL/api/rag-study" -Method POST -Body $studyPayload -ContentType "application/json"
    
    if ($response.success) {
        Write-Host "✅ Feasibility study generated successfully!" -ForegroundColor Green
        Write-Host "   Study ID: $($response.data.study_id)" -ForegroundColor Gray
        Write-Host "   Business: $($response.data.business_name)" -ForegroundColor Gray
        Write-Host "   Viability Score: $($response.data.executive_summary.viability_score)" -ForegroundColor Gray
        Write-Host "   Time to Break-even: $($response.data.executive_summary.time_to_break_even_months) months" -ForegroundColor Gray
    } else {
        Write-Host "❌ Request failed: $($response.error)" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Request failed: $_" -ForegroundColor Red
    Write-Host "Make sure backend is running and API keys are configured" -ForegroundColor Red
}
Write-Host ""

# Test 3: Persona Debate
Write-Host "Test 3: Persona Debate" -ForegroundColor Yellow
Write-Host "---------------------"
$debatePayload = @{
    business_name = "Al-Rashid Tech Solutions"
    description = "We provide IT consulting and software development services for small and medium enterprises in Riyadh. Our services include custom software development, cloud migration, and digital transformation consulting."
    target_city = "Riyadh"
    capital_budget = 300000
    industry = "tech"
    business_model = "service_based"
    initial_employees = 5
    founder_experience = "experienced"
    contact_email = "test@example.com"
    include_competitor_analysis = $false
    include_persona_debate = $true
} | ConvertTo-Json

try {
    Write-Host "Sending request to /api/personas..." -ForegroundColor Gray
    $response = Invoke-RestMethod -Uri "$BASE_URL/api/personas" -Method POST -Body $debatePayload -ContentType "application/json"
    
    if ($response.success) {
        Write-Host "✅ Persona debate completed successfully!" -ForegroundColor Green
        Write-Host "   Session ID: $($response.data.session_id)" -ForegroundColor Gray
        Write-Host "   Verdict: $($response.data.overall_verdict)" -ForegroundColor Gray
        Write-Host "   Number of turns: $($response.data.debate_transcript.Count)" -ForegroundColor Gray
        
        Write-Host "" -ForegroundColor Gray
        Write-Host "Key Risks Identified:" -ForegroundColor Yellow
        $response.data.key_risks | ForEach-Object { Write-Host "   - $_" -ForegroundColor Gray }
        
        Write-Host "" -ForegroundColor Gray
        Write-Host "Key Opportunities Identified:" -ForegroundColor Yellow
        $response.data.key_opportunities | ForEach-Object { Write-Host "   - $_" -ForegroundColor Gray }
    } else {
        Write-Host "❌ Request failed: $($response.error)" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Request failed: $_" -ForegroundColor Red
}
Write-Host ""

# Test 4: Competitor Analysis
Write-Host "Test 4: Competitor Analysis" -ForegroundColor Yellow
Write-Host "---------------------"
$competitorPayload = @{
    business_name = "Test Retail Store"
    description = "A fashion retail store selling modest clothing and accessories for women in Riyadh."
    target_city = "Riyadh"
    district = "Al-Olaya"
    capital_budget = 600000
    industry = "retail"
    business_model = "brick_and_mortar"
    initial_employees = 8
    founder_experience = "intermediate"
    contact_email = "test@example.com"
    include_competitor_analysis = $true
    include_persona_debate = $false
} | ConvertTo-Json

try {
    Write-Host "Sending request to /api/competitors..." -ForegroundColor Gray
    $response = Invoke-RestMethod -Uri "$BASE_URL/api/competitors" -Method POST -Body $competitorPayload -ContentType "application/json"
    
    if ($response.success) {
        Write-Host "✅ Competitor analysis completed successfully!" -ForegroundColor Green
        Write-Host "   Analysis ID: $($response.data.analysis_id)" -ForegroundColor Gray
        Write-Host "   Location: $($response.data.search_location)" -ForegroundColor Gray
        Write-Host "   Competitors Found: $($response.data.competitors.Count)" -ForegroundColor Gray
        Write-Host "   Market Saturation: $([math]::Round($response.data.market_saturation_score, 1))%" -ForegroundColor Gray
        
        if ($response.data.competitors.Count -gt 0) {
            Write-Host "" -ForegroundColor Gray
            Write-Host "Top Competitors:" -ForegroundColor Yellow
            $response.data.competitors | Select-Object -First 3 | ForEach-Object {
                Write-Host "   - $($_.name) (Rating: $($_.rating))" -ForegroundColor Gray
            }
        }
    } else {
        Write-Host "❌ Request failed: $($response.error)" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Request failed: $_" -ForegroundColor Red
}
Write-Host ""

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Test Complete!" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "If all tests passed, your API is working correctly!" -ForegroundColor Green
Write-Host "Open http://localhost:3000 to use the frontend." -ForegroundColor White
Write-Host ""
Write-Host "Troubleshooting:" -ForegroundColor Yellow
Write-Host "- If tests fail, ensure backend is running: cd backend && cargo run" -ForegroundColor Gray
Write-Host "- Check API keys are set in backend/.env" -ForegroundColor Gray
Write-Host "- Verify Docker services: docker-compose ps" -ForegroundColor Gray
