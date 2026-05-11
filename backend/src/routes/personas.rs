use axum::{extract::State, Json};
use serde_json;
use tracing::{info, instrument};
use validator::Validate;

use crate::{
    config::AppConfig,
    models::{
        ApiResponse, AppError, DebateTurn, FeasibilityRequest, PersonaAgent, PersonaDebateResponse,
        Sentiment, Verdict,
    },
    services::gemini_service::GeminiService,
};

/// POST /api/personas
///
/// Initiates an AI persona debate where multiple Saudi demographic personas
/// critique and debate the submitted business idea.
#[instrument(skip(config), fields(business_name = %payload.business_name))]
pub async fn create_persona_debate(
    State(config): State<AppConfig>,
    Json(payload): Json<FeasibilityRequest>,
) -> Result<Json<ApiResponse<PersonaDebateResponse>>, AppError> {
    info!(
        "Received persona debate request for: {}",
        payload.business_name
    );

    // Validate the request payload
    payload
        .validate()
        .map_err(|e: validator::ValidationErrors| AppError::Validation(e.to_string()))?;

    // Initialize Gemini service (using Gemini instead of Claude for now)
    let gemini = GeminiService::new(&config);

    // Define personas for the debate as JSON values for Gemini
    let personas = vec![
        serde_json::json!({
            "id": "inv_001",
            "name": "Abdullah Al-Rashid",
            "role": "Angel Investor",
            "perspective": "ROI-focused, risk-aware investor. Questions payback periods, scalability, and exit strategies."
        }),
        serde_json::json!({
            "id": "stu_001",
            "name": "Fatima Al-Zahrani",
            "role": "Young Consumer",
            "perspective": "University student, price-sensitive, values convenience and digital experience."
        }),
        serde_json::json!({
            "id": "biz_001",
            "name": "Khalid Al-Otaibi",
            "role": "Business Owner",
            "perspective": "Experienced SME owner, practical operational insights, concerned about competition and regulations."
        }),
        serde_json::json!({
            "id": "gov_001",
            "name": "Sara Al-Qahtani",
            "role": "Regulatory Expert",
            "perspective": "Former SAGIA advisor, focuses on regulatory compliance, licensing, and legal requirements."
        }),
    ];

    // Build business idea description
    let business_idea = format!(
        "Business Name: {}\nIndustry: {}\nBusiness Model: {:?}\nLocation: {}\nCapital Budget: SAR {:.2}\nDescription: {}",
        payload.business_name,
        payload.industry,
        payload.business_model,
        payload.target_city,
        payload.capital_budget,
        payload.description
    );

    // Orchestrate debate using Gemini (instead of Claude for now)
    // Note: ClaudeService code is kept in codebase for later use
    let debate_result = match gemini.orchestrate_debate(&business_idea, &personas).await {
        Ok(result) => result,
        Err(e) => {
            if config.environment.eq_ignore_ascii_case("development")
                || config.environment.eq_ignore_ascii_case("dev")
            {
                info!(
                    "Gemini failed, returning sample debate for development: {}",
                    e
                );
                let sample = build_sample_debate_json(&payload);
                let response = parse_debate_result(sample, &payload, personas).await?;
                return Ok(Json(ApiResponse::success(response)));
            }

            return Err(AppError::AiService(format!(
                "Debate orchestration failed: {}",
                e
            )));
        }
    };

    // Parse the debate result into our response structure
    let response = parse_debate_result(debate_result, &payload, personas).await?;

    info!(
        "Persona debate completed for session: {}",
        response.session_id
    );

    Ok(Json(ApiResponse::success(response)))
}

fn generate_session_id() -> String {
    format!(
        "sess_{}",
        &uuid::Uuid::new_v4().to_string().replace("-", "")[..16]
    )
}

/// Parse Gemini's debate result into our response structure
async fn parse_debate_result(
    debate_json: serde_json::Value,
    payload: &FeasibilityRequest,
    personas: Vec<serde_json::Value>,
) -> Result<PersonaDebateResponse, AppError> {
    let session_id = generate_session_id();

    // Convert personas to PersonaAgent format
    let persona_agents: Vec<PersonaAgent> = personas
        .into_iter()
        .map(|p| PersonaAgent {
            id: p["id"].as_str().unwrap_or("").to_string(),
            name: p["name"].as_str().unwrap_or("").to_string(),
            demographic_profile: p["perspective"].as_str().unwrap_or("").to_string(),
            role_description: p["role"].as_str().unwrap_or("").to_string(),
            avatar_url: None,
        })
        .collect();

    // Parse debate transcript
    let transcript: Vec<DebateTurn> = debate_json["debate_transcript"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|turn| DebateTurn {
            turn_number: turn["turn_number"].as_i64().unwrap_or(0) as i32,
            persona_id: turn["persona_id"].as_str().unwrap_or("").to_string(),
            persona_name: turn["persona_name"].as_str().unwrap_or("").to_string(),
            message: turn["message"].as_str().unwrap_or("").to_string(),
            sentiment: parse_sentiment(turn["sentiment"].as_str().unwrap_or("neutral")),
            concerns_raised: turn["concerns_raised"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
        .collect();

    // Parse consensus
    let consensus_summary = debate_json["consensus_summary"]
        .as_str()
        .unwrap_or("No consensus reached")
        .to_string();

    // Parse key risks
    let key_risks: Vec<String> = debate_json["key_risks"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    // Parse key opportunities
    let key_opportunities: Vec<String> = debate_json["key_opportunities"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    // Parse verdict
    let overall_verdict =
        parse_verdict(debate_json["overall_verdict"].as_str().unwrap_or("neutral"));

    Ok(PersonaDebateResponse {
        session_id,
        business_name: payload.business_name.clone(),
        personas: persona_agents,
        debate_transcript: transcript,
        consensus_summary,
        key_risks,
        key_opportunities,
        overall_verdict,
    })
}

fn parse_sentiment(s: &str) -> Sentiment {
    match s.to_lowercase().as_str() {
        "supportive" => Sentiment::Supportive,
        "skeptical" => Sentiment::Skeptical,
        "enthusiastic" => Sentiment::Enthusiastic,
        "concerned" => Sentiment::Concerned,
        _ => Sentiment::Neutral,
    }
}

fn parse_verdict(s: &str) -> Verdict {
    match s.to_lowercase().as_str() {
        "strong_yes" => Verdict::StrongYes,
        "yes" => Verdict::Yes,
        "no" => Verdict::No,
        "strong_no" => Verdict::StrongNo,
        _ => Verdict::Neutral,
    }
}

fn build_sample_debate_json(payload: &FeasibilityRequest) -> serde_json::Value {
    serde_json::json!({
        "debate_transcript": [
            {
                "turn_number": 1,
                "persona_id": "inv_001",
                "persona_name": "Abdullah Al-Rashid",
                "message": format!(
                    "The {} concept has potential, but I need clarity on margins and a realistic payback timeline.",
                    payload.business_name
                ),
                "sentiment": "skeptical",
                "concerns_raised": ["Unit economics", "Payback period"],
                "opportunities_identified": ["Growing demand", "Underserved niche"]
            },
            {
                "turn_number": 2,
                "persona_id": "stu_001",
                "persona_name": "Fatima Al-Zahrani",
                "message": "If the experience is convenient and priced fairly, I would try it with friends.",
                "sentiment": "supportive",
                "concerns_raised": ["Price sensitivity"],
                "opportunities_identified": ["Social media buzz", "Student promotions"]
            },
            {
                "turn_number": 3,
                "persona_id": "biz_001",
                "persona_name": "Khalid Al-Otaibi",
                "message": "Operations and staffing will be the hardest part. Plan for reliable suppliers and training.",
                "sentiment": "concerned",
                "concerns_raised": ["Staffing", "Supplier reliability"],
                "opportunities_identified": ["Operational discipline", "Process automation"]
            },
            {
                "turn_number": 4,
                "persona_id": "gov_001",
                "persona_name": "Sara Al-Qahtani",
                "message": "Make sure you budget time for licensing and compliance requirements early in the setup.",
                "sentiment": "neutral",
                "concerns_raised": ["Licensing timeline"],
                "opportunities_identified": ["Regulatory clarity", "Support programs"]
            }
        ],
        "consensus_summary": "The idea is promising with clear demand, but requires strong financial planning and operational readiness.",
        "key_risks": ["Cash flow strain", "Competitive pressure", "Hiring challenges"],
        "key_opportunities": ["Growing local demand", "Differentiated experience", "Partnerships"],
        "overall_verdict": "neutral"
    })
}
