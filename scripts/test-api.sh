#!/bin/bash

# Saudi Market AI - API Test Script
# Run this script to test if the backend APIs are working

BASE_URL="http://localhost:3001"

echo "====================================="
echo "Saudi Market AI - API Test Script"
echo "====================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;37m'
NC='\033[0m' # No Color

# Test 1: Health Check
echo -e "${YELLOW}Test 1: Health Check${NC}"
echo "---------------------"
if response=$(curl -s "$BASE_URL/health"); then
    echo -e "${GREEN}✅ Health check passed: $response${NC}"
else
    echo -e "${RED}❌ Health check failed${NC}"
    echo -e "${RED}Make sure backend is running on port 3001${NC}"
fi
echo ""

# Test 2: Feasibility Study
echo -e "${YELLOW}Test 2: Feasibility Study (RAG)${NC}"
echo "---------------------"
study_payload='{
    "business_name": "Test Coffee Shop",
    "description": "A specialty coffee shop targeting young professionals in Jeddah. We will serve premium arabica coffee, fresh pastries, and provide a comfortable workspace environment.",
    "target_city": "Jeddah",
    "capital_budget": 450000,
    "industry": "food",
    "business_model": "brick_and_mortar",
    "initial_employees": 6,
    "founder_experience": "intermediate",
    "contact_email": "test@example.com",
    "include_competitor_analysis": false,
    "include_persona_debate": false
}'

echo -e "${GRAY}Sending request to /api/rag-study...${NC}"
if response=$(curl -s -X POST "$BASE_URL/api/rag-study" \
    -H "Content-Type: application/json" \
    -d "$study_payload"); then
    
    # Check if response contains success
    if echo "$response" | grep -q '"success":true'; then
        echo -e "${GREEN}✅ Feasibility study generated successfully!${NC}"
        echo -e "${GRAY}   Study ID: $(echo "$response" | grep -o '"study_id":"[^"]*' | cut -d'"' -f4)${NC}"
        echo -e "${GRAY}   Business: $(echo "$response" | grep -o '"business_name":"[^"]*' | cut -d'"' -f4)${NC}"
    else
        echo -e "${RED}❌ Request failed${NC}"
        echo -e "${RED}   Response: $(echo "$response" | head -c 200)${NC}"
    fi
else
    echo -e "${RED}❌ Request failed${NC}"
    echo -e "${RED}Make sure backend is running and API keys are configured${NC}"
fi
echo ""

# Test 3: Persona Debate
echo -e "${YELLOW}Test 3: Persona Debate${NC}"
echo "---------------------"
debate_payload='{
    "business_name": "Al-Rashid Tech Solutions",
    "description": "We provide IT consulting and software development services for SMEs in Riyadh.",
    "target_city": "Riyadh",
    "capital_budget": 300000,
    "industry": "tech",
    "business_model": "service_based",
    "initial_employees": 5,
    "founder_experience": "experienced",
    "contact_email": "test@example.com",
    "include_competitor_analysis": false,
    "include_persona_debate": true
}'

echo -e "${GRAY}Sending request to /api/personas...${NC}"
if response=$(curl -s -X POST "$BASE_URL/api/personas" \
    -H "Content-Type: application/json" \
    -d "$debate_payload"); then
    
    if echo "$response" | grep -q '"success":true'; then
        echo -e "${GREEN}✅ Persona debate completed successfully!${NC}"
        echo -e "${GRAY}   Session ID: $(echo "$response" | grep -o '"session_id":"[^"]*' | cut -d'"' -f4)${NC}"
        echo -e "${GRAY}   Verdict: $(echo "$response" | grep -o '"overall_verdict":"[^"]*' | cut -d'"' -f4)${NC}"
    else
        echo -e "${RED}❌ Request failed${NC}"
    fi
else
    echo -e "${RED}❌ Request failed${NC}"
fi
echo ""

# Test 4: Competitor Analysis
echo -e "${YELLOW}Test 4: Competitor Analysis${NC}"
echo "---------------------"
competitor_payload='{
    "business_name": "Test Retail Store",
    "description": "A fashion retail store selling modest clothing for women in Riyadh.",
    "target_city": "Riyadh",
    "district": "Al-Olaya",
    "capital_budget": 600000,
    "industry": "retail",
    "business_model": "brick_and_mortar",
    "initial_employees": 8,
    "founder_experience": "intermediate",
    "contact_email": "test@example.com",
    "include_competitor_analysis": true,
    "include_persona_debate": false
}'

echo -e "${GRAY}Sending request to /api/competitors...${NC}"
if response=$(curl -s -X POST "$BASE_URL/api/competitors" \
    -H "Content-Type: application/json" \
    -d "$competitor_payload"); then
    
    if echo "$response" | grep -q '"success":true'; then
        echo -e "${GREEN}✅ Competitor analysis completed successfully!${NC}"
        echo -e "${GRAY}   Analysis ID: $(echo "$response" | grep -o '"analysis_id":"[^"]*' | cut -d'"' -f4)${NC}"
        echo -e "${GRAY}   Competitors Found: $(echo "$response" | grep -o '"total_competitors_found":[0-9]*' | grep -o '[0-9]*')${NC}"
    else
        echo -e "${RED}❌ Request failed${NC}"
    fi
else
    echo -e "${RED}❌ Request failed${NC}"
fi
echo ""

echo "====================================="
echo -e "${CYAN}Test Complete!${NC}"
echo "====================================="
echo ""
echo -e "${GREEN}If all tests passed, your API is working correctly!${NC}"
echo -e "${WHITE}Open http://localhost:3000 to use the frontend.${NC}"
echo ""
echo -e "${YELLOW}Troubleshooting:${NC}"
echo "- If tests fail, ensure backend is running: cd backend && cargo run"
echo "- Check API keys are set in backend/.env"
echo "- Verify Docker services: docker-compose ps"
