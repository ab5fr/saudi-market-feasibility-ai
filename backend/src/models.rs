use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod db_models;

// ============================================================================
// Core Feasibility Request - Shared between Frontend and Backend
// ============================================================================

/// Main payload for initiating a feasibility study
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FeasibilityRequest {
    /// Business idea name/title
    #[validate(length(
        min = 3,
        max = 200,
        message = "Business name must be between 3 and 200 characters"
    ))]
    pub business_name: String,

    /// Detailed description of the business idea
    #[validate(length(
        min = 50,
        max = 5000,
        message = "Description must be between 50 and 5000 characters"
    ))]
    pub description: String,

    /// Target Saudi city (e.g., "Jeddah", "Riyadh", "Dammam")
    #[validate(length(min = 2, max = 100, message = "City name is required"))]
    pub target_city: String,

    /// Specific district/neighborhood within the city (optional)
    pub district: Option<String>,

    /// Capital budget in SAR (Saudi Riyals)
    #[validate(range(
        min = 0.0,
        max = 1000000000.0,
        message = "Budget must be 0 or more (use 0 to let AI estimate)"
    ))]
    pub capital_budget: f64,

    /// Industry/category (e.g., "food", "retail", "tech", "healthcare")
    pub industry: String,

    /// Business model type
    pub business_model: BusinessModel,

    /// Expected number of employees at launch
    #[validate(range(min = 0, max = 10000))]
    pub initial_employees: i32,

    /// User's experience level in this industry
    pub founder_experience: ExperienceLevel,

    /// Contact email for the report
    #[validate(email(message = "Valid email is required"))]
    pub contact_email: String,

    /// Optional: Specific questions the user wants answered
    pub specific_questions: Option<Vec<String>>,

    /// Whether the user wants competitor analysis
    pub include_competitor_analysis: bool,

    /// Whether the user wants persona debates
    pub include_persona_debate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusinessModel {
    BrickAndMortar, // Physical store/location
    Ecommerce,      // Online-only
    Hybrid,         // Both physical and online
    ServiceBased,   // Service-oriented business
    B2b,            // Business-to-business
    Marketplace,    // Platform connecting buyers/sellers
    Subscription,   // Recurring revenue model
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperienceLevel {
    Beginner,     // No prior experience
    Intermediate, // Some experience
    Experienced,  // Significant experience
    Expert,       // Industry veteran
}

// ============================================================================
// Response Types
// ============================================================================

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }
}

// ============================================================================
// Persona Debate Types
// ============================================================================

/// Response from the persona debate endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaDebateResponse {
    pub session_id: String,
    pub business_name: String,
    pub personas: Vec<PersonaAgent>,
    pub debate_transcript: Vec<DebateTurn>,
    pub consensus_summary: String,
    pub key_risks: Vec<String>,
    pub key_opportunities: Vec<String>,
    pub overall_verdict: Verdict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaAgent {
    pub id: String,
    pub name: String,
    pub demographic_profile: String,
    pub role_description: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateTurn {
    pub turn_number: i32,
    pub persona_id: String,
    pub persona_name: String,
    pub message: String,
    pub sentiment: Sentiment,
    pub concerns_raised: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sentiment {
    Supportive,
    Skeptical,
    Neutral,
    Concerned,
    Enthusiastic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    StrongYes,
    Yes,
    Neutral,
    No,
    StrongNo,
}

// ============================================================================
// RAG Study Types
// ============================================================================

/// Response from the RAG-based feasibility study endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibilityStudyResponse {
    pub study_id: String,
    pub business_name: String,
    pub generated_at: DateTime<Utc>,
    pub executive_summary: ExecutiveSummary,
    pub market_analysis: MarketAnalysis,
    pub financial_projections: FinancialProjections,
    pub legal_requirements: LegalRequirements,
    pub risk_assessment: RiskAssessment,
    pub recommendations: Recommendations,
    pub sources_cited: Vec<GovernmentSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    pub viability_score: f32, // 0-100
    pub summary_text: String,
    pub key_strengths: Vec<String>,
    pub key_challenges: Vec<String>,
    pub time_to_break_even_months: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketAnalysis {
    pub target_market_size: String,
    pub market_growth_rate: String,
    pub customer_segments: Vec<CustomerSegment>,
    pub competitive_landscape: String,
    pub market_entry_barriers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    pub name: String,
    pub description: String,
    pub estimated_size: String,
    pub characteristics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialProjections {
    pub initial_investment_breakdown: Vec<CostItem>,
    pub monthly_operating_costs: Vec<CostItem>,
    pub payroll_breakdown: PayrollBreakdown,
    pub revenue_projections: RevenueProjections,
    pub profit_loss_summary: ProfitLossSummary,
    pub financial_assumptions: Vec<String>,
    pub profitability_timeline: String,
    pub roi_estimate_3yr: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub category: String,
    pub description: String,
    pub amount_sar: f64,
    pub is_one_time: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueProjections {
    pub year_1_monthly_avg: f64,
    pub year_2_monthly_avg: f64,
    pub year_3_monthly_avg: f64,
    pub revenue_streams: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollBreakdown {
    pub headcount: i32,
    pub avg_monthly_salary_sar: f64,
    pub base_salaries_monthly_sar: f64,
    pub overtime_monthly_sar: f64,
    pub allowances_monthly_sar: f64,
    pub gosi_employer_monthly_sar: f64,
    pub gosi_employee_monthly_sar: f64,
    pub end_of_service_accrual_monthly_sar: f64,
    pub total_payroll_monthly_sar: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitLossSummary {
    pub year_1: ProfitLossYear,
    pub year_2: ProfitLossYear,
    pub year_3: ProfitLossYear,
    pub assumptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitLossYear {
    pub revenue_sar: f64,
    pub cogs_sar: f64,
    pub gross_profit_sar: f64,
    pub operating_expenses_sar: f64,
    pub net_profit_sar: f64,
    pub net_margin_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalRequirements {
    pub business_structure_options: Vec<BusinessStructure>,
    pub required_licenses: Vec<License>,
    pub regulatory_compliance: Vec<ComplianceItem>,
    pub estimated_setup_costs_sar: f64,
    pub setup_timeline_weeks: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessStructure {
    pub structure_type: String,
    pub description: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub suitability_score: i32, // 1-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub issuing_authority: String,
    pub estimated_cost_sar: f64,
    pub processing_time_days: i32,
    pub is_mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceItem {
    pub regulation: String,
    pub authority: String,
    pub description: String,
    pub priority: CompliancePriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompliancePriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub market_risks: Vec<RiskItem>,
    pub financial_risks: Vec<RiskItem>,
    pub operational_risks: Vec<RiskItem>,
    pub regulatory_risks: Vec<RiskItem>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskItem {
    pub risk_name: String,
    pub description: String,
    pub likelihood: RiskLevel,
    pub impact: RiskLevel,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendations {
    pub go_no_go_verdict: Verdict,
    pub critical_success_factors: Vec<String>,
    pub next_steps: Vec<String>,
    pub suggested_partnerships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentSource {
    pub document_name: String,
    pub authority: String, // e.g., "Monsha'at", "Qiwa", "Balady"
    pub url: Option<String>,
    pub citation_text: String,
    pub relevance_score: f32,
}

// ============================================================================
// Competitor Analysis Types
// ============================================================================

/// Response from the competitor analysis endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorAnalysisResponse {
    pub analysis_id: String,
    pub business_name: String,
    pub search_location: String,
    pub search_query_used: String,
    pub competitors: Vec<Competitor>,
    pub market_saturation_score: f32, // 0-100, higher = more saturated
    pub market_gap_analysis: String,
    pub differentiation_strategy: Vec<String>,
    pub pricing_benchmarks: PricingBenchmarks,
    pub online_presence_summary: OnlinePresenceSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub name: String,
    pub location: String,
    pub distance_km: Option<f64>,
    pub business_type: String,
    pub rating: Option<f32>,
    pub review_count: Option<i32>,
    pub price_level: Option<i32>, // 1-4, like Google Maps
    pub website: Option<String>,
    pub phone: Option<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    DirectCompetitor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingBenchmarks {
    pub average_price_range: String,
    pub lowest_observed: String,
    pub highest_observed: String,
    pub pricing_strategy_recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlinePresenceSummary {
    pub total_competitors_found: i32,
    pub avg_google_rating: Option<f32>,
    pub competitors_with_websites: i32,
    pub social_media_presence: String,
    pub online_reputation_summary: String,
}

// ============================================================================
// Error Types
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("AI service error: {0}")]
    AiService(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("RAG retrieval error: {0}")]
    RagRetrieval(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::Json;
        use axum::http::StatusCode;

        let (status, error_message) = match &self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::ExternalApi(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::AiService(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::RagRetrieval(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = serde_json::json!({
            "success": false,
            "error": error_message,
            "timestamp": Utc::now().to_rfc3339(),
        });

        (status, Json(body)).into_response()
    }
}
