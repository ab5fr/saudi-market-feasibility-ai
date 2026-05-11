import os

import pytest
import requests

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:3001")
RUN_TESTS = os.getenv("RUN_API_TESTS") == "1"

pytestmark = pytest.mark.skipif(
    not RUN_TESTS,
    reason="Set RUN_API_TESTS=1 to run integration tests",
)


def _post_json(path: str, payload: dict) -> dict:
    response = requests.post(
        f"{BASE_URL}{path}",
        json=payload,
        timeout=120,
    )
    response.raise_for_status()
    body = response.json()
    assert body.get("success") is True, body.get("error")
    return body


def test_health() -> None:
    response = requests.get(f"{BASE_URL}/health", timeout=30)
    response.raise_for_status()
    assert response.text


def test_rag_study() -> None:
    payload = {
        "business_name": "Test Coffee Shop",
        "description": "A specialty coffee shop targeting young professionals in Jeddah with premium arabica coffee and pastries.",
        "target_city": "Jeddah",
        "capital_budget": 450000,
        "industry": "food",
        "business_model": "brick_and_mortar",
        "initial_employees": 6,
        "founder_experience": "intermediate",
        "contact_email": "test@example.com",
        "include_competitor_analysis": False,
        "include_persona_debate": False,
    }

    body = _post_json("/api/rag-study", payload)
    assert body["data"]["study_id"]


def test_persona_debate() -> None:
    payload = {
        "business_name": "Al-Rashid Tech Solutions",
        "description": "IT consulting and software development for SMEs in Riyadh.",
        "target_city": "Riyadh",
        "capital_budget": 300000,
        "industry": "tech",
        "business_model": "service_based",
        "initial_employees": 5,
        "founder_experience": "experienced",
        "contact_email": "test@example.com",
        "include_competitor_analysis": False,
        "include_persona_debate": True,
    }

    body = _post_json("/api/personas", payload)
    assert body["data"]["session_id"]


def test_competitor_analysis() -> None:
    payload = {
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
        "include_competitor_analysis": True,
        "include_persona_debate": False,
    }

    body = _post_json("/api/competitors", payload)
    assert body["data"]["analysis_id"]
