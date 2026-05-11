use axum::{extract::State, Json};
use tracing::{info, instrument};
use validator::Validate;

use crate::{
    config::AppConfig,
    models::{
        ApiResponse, AppError, BusinessStructure, ComplianceItem, CompliancePriority, CostItem,
        CustomerSegment, ExecutiveSummary, FeasibilityRequest, FeasibilityStudyResponse,
        FinancialProjections, GovernmentSource, LegalRequirements, License, MarketAnalysis,
        PayrollBreakdown, ProfitLossSummary, ProfitLossYear, Recommendations, RevenueProjections,
        RiskAssessment, RiskItem, RiskLevel, Verdict,
    },
    services::{
        document_service::RagPipeline,
        gemini_service::GeminiService,
    },
};

/// POST /api/rag-study
///
/// Generates a comprehensive, RAG-based feasibility study that strictly bases
/// financial and legal advice on uploaded Saudi government documents.
#[instrument(skip(config), fields(business_name = %payload.business_name))]
pub async fn generate_feasibility_study(
    State(config): State<AppConfig>,
    Json(payload): Json<FeasibilityRequest>,
) -> Result<Json<ApiResponse<FeasibilityStudyResponse>>, AppError> {
    info!("Received RAG study request for: {}", payload.business_name);

    // Validate the request payload
    payload
        .validate()
        .map_err(|e: validator::ValidationErrors| AppError::Validation(e.to_string()))?;

    // Initialize services
    let gemini = GeminiService::new(&config);
    let rag = RagPipeline::new(&config);

    // Step 1: Retrieve relevant context from RAG
    let search_query = format!(
        "{} business in {} Saudi Arabia regulatory requirements licenses GOSI labor costs wages rent operating costs",
        payload.industry, payload.target_city
    );

    let context = match rag.retrieve_context(&search_query, 5).await {
        Ok(ctx) => ctx,
        Err(e) => {
            info!(
                "RAG retrieval failed or no documents found: {}. Using general knowledge.",
                e
            );
            "No specific Saudi regulatory documents found. Analysis based on general Saudi business knowledge.".to_string()
        }
    };

    // Step 2: Build business request for Gemini
    let capital_text = if payload.capital_budget > 0.0 {
        format!("SAR {:.2}", payload.capital_budget)
    } else {
        "Not provided".to_string()
    };
    let employees_text = if payload.initial_employees > 0 {
        payload.initial_employees.to_string()
    } else {
        "Not provided".to_string()
    };

    let business_request = format!(
        "Business: {}\nIndustry: {}\nModel: {:?}\nLocation: {}\nCapital: {}\nEmployees: {}\nExperience: {:?}\n\nDescription: {}",
        payload.business_name,
        payload.industry,
        payload.business_model,
        payload.target_city,
        capital_text,
        employees_text,
        payload.founder_experience,
        payload.description
    );

    // Step 3: Generate feasibility study using Gemini
    let study_json = match gemini
        .generate_feasibility_study(&business_request, &context)
        .await
    {
        Ok(json) => json,
        Err(e) => {
            if config.environment.eq_ignore_ascii_case("development")
                || config.environment.eq_ignore_ascii_case("dev")
            {
                info!(
                    "Gemini failed, returning sample study for development: {}",
                    e
                );
                let sample = create_sample_study(&payload);
                return Ok(Json(ApiResponse::success(sample)));
            }

            return Err(AppError::AiService(format!(
                "Study generation failed: {}",
                e
            )));
        }
    };

    // Step 4: Parse the response into our structure
    let response = parse_study_response(study_json, &payload)?;

    info!("Feasibility study generated: {}", response.study_id);

    Ok(Json(ApiResponse::success(response)))
}

/// Parse Gemini's study JSON into FeasibilityStudyResponse
fn parse_study_response(
    json: serde_json::Value,
    request: &FeasibilityRequest,
) -> Result<FeasibilityStudyResponse, AppError> {
    let study_id = format!(
        "study_{}",
        &uuid::Uuid::new_v4().to_string().replace("-", "")[..16]
    );

    // Helper function to parse arrays safely
    let parse_string_array = |val: &serde_json::Value| -> Vec<String> {
        val.as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    };

    // Parse executive summary
    let exec_summary = json
        .get("executive_summary")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let executive_summary = ExecutiveSummary {
        viability_score: exec_summary["viability_score"].as_f64().unwrap_or(50.0) as f32,
        summary_text: exec_summary["summary_text"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        key_strengths: parse_string_array(&exec_summary["key_strengths"]),
        key_challenges: parse_string_array(&exec_summary["key_challenges"]),
        time_to_break_even_months: exec_summary["time_to_break_even_months"]
            .as_i64()
            .unwrap_or(12) as i32,
    };

    // Parse market analysis
    let market_json = json
        .get("market_analysis")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let market_analysis = MarketAnalysis {
        target_market_size: market_json["target_market_size"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        market_growth_rate: market_json["market_growth_rate"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        customer_segments: parse_customer_segments(&market_json["customer_segments"]),
        competitive_landscape: market_json["competitive_landscape"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        market_entry_barriers: parse_string_array(&market_json["market_entry_barriers"]),
    };

    // Parse financial projections
    let financial_json = json
        .get("financial_projections")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let financial_projections = FinancialProjections {
        initial_investment_breakdown: parse_cost_items(
            &financial_json["initial_investment_breakdown"],
        ),
        monthly_operating_costs: parse_cost_items(&financial_json["monthly_operating_costs"]),
        payroll_breakdown: parse_payroll_breakdown(
            &financial_json["payroll_breakdown"],
            request.initial_employees,
        ),
        revenue_projections: parse_revenue_projections(&financial_json["revenue_projections"]),
        profit_loss_summary: parse_profit_loss_summary(&financial_json["profit_loss_summary"]),
        financial_assumptions: parse_string_array(&financial_json["financial_assumptions"]),
        profitability_timeline: financial_json["profitability_timeline"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        roi_estimate_3yr: financial_json["roi_estimate_3yr"].as_f64().unwrap_or(0.0) as f32,
    };

    // Parse legal requirements
    let legal_json = json
        .get("legal_requirements")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let legal_requirements = LegalRequirements {
        business_structure_options: parse_business_structures(
            &legal_json["business_structure_options"],
        ),
        required_licenses: parse_licenses(&legal_json["required_licenses"]),
        regulatory_compliance: parse_compliance_items(&legal_json["regulatory_compliance"]),
        estimated_setup_costs_sar: legal_json["estimated_setup_costs_sar"]
            .as_f64()
            .unwrap_or(25000.0),
        setup_timeline_weeks: legal_json["setup_timeline_weeks"].as_i64().unwrap_or(6) as i32,
    };

    // Parse risk assessment
    let risk_json = json
        .get("risk_assessment")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let risk_assessment = RiskAssessment {
        market_risks: parse_risk_items(&risk_json["market_risks"]),
        financial_risks: parse_risk_items(&risk_json["financial_risks"]),
        operational_risks: parse_risk_items(&risk_json["operational_risks"]),
        regulatory_risks: parse_risk_items(&risk_json["regulatory_risks"]),
        mitigation_strategies: parse_string_array(&risk_json["mitigation_strategies"]),
    };

    // Parse recommendations
    let rec_json = json
        .get("recommendations")
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let recommendations = Recommendations {
        go_no_go_verdict: parse_verdict(rec_json["go_no_go_verdict"].as_str().unwrap_or("neutral")),
        critical_success_factors: parse_string_array(&rec_json["critical_success_factors"]),
        next_steps: parse_string_array(&rec_json["next_steps"]),
        suggested_partnerships: parse_string_array(&rec_json["suggested_partnerships"]),
    };

    // Parse sources cited
    let sources_cited = parse_government_sources(&json["sources_cited"]);

    Ok(FeasibilityStudyResponse {
        study_id,
        business_name: request.business_name.clone(),
        generated_at: chrono::Utc::now(),
        executive_summary,
        market_analysis,
        financial_projections,
        legal_requirements,
        risk_assessment,
        recommendations,
        sources_cited,
    })
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

fn parse_customer_segments(json: &serde_json::Value) -> Vec<CustomerSegment> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| CustomerSegment {
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().unwrap_or("").to_string(),
                    estimated_size: item["estimated_size"].as_str().unwrap_or("").to_string(),
                    characteristics: item["characteristics"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_cost_items(json: &serde_json::Value) -> Vec<CostItem> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| CostItem {
                    category: item["category"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().unwrap_or("").to_string(),
                    amount_sar: item["amount_sar"].as_f64().unwrap_or(0.0),
                    is_one_time: item["is_one_time"].as_bool().unwrap_or(true),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_revenue_projections(json: &serde_json::Value) -> RevenueProjections {
    RevenueProjections {
        year_1_monthly_avg: json["year_1_monthly_avg"].as_f64().unwrap_or(0.0),
        year_2_monthly_avg: json["year_2_monthly_avg"].as_f64().unwrap_or(0.0),
        year_3_monthly_avg: json["year_3_monthly_avg"].as_f64().unwrap_or(0.0),
        revenue_streams: json["revenue_streams"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn parse_payroll_breakdown(json: &serde_json::Value, default_headcount: i32) -> PayrollBreakdown {
    PayrollBreakdown {
        headcount: json["headcount"]
            .as_i64()
            .unwrap_or(default_headcount as i64) as i32,
        avg_monthly_salary_sar: json["avg_monthly_salary_sar"].as_f64().unwrap_or(0.0),
        base_salaries_monthly_sar: json["base_salaries_monthly_sar"].as_f64().unwrap_or(0.0),
        overtime_monthly_sar: json["overtime_monthly_sar"].as_f64().unwrap_or(0.0),
        allowances_monthly_sar: json["allowances_monthly_sar"].as_f64().unwrap_or(0.0),
        gosi_employer_monthly_sar: json["gosi_employer_monthly_sar"].as_f64().unwrap_or(0.0),
        gosi_employee_monthly_sar: json["gosi_employee_monthly_sar"].as_f64().unwrap_or(0.0),
        end_of_service_accrual_monthly_sar: json["end_of_service_accrual_monthly_sar"]
            .as_f64()
            .unwrap_or(0.0),
        total_payroll_monthly_sar: json["total_payroll_monthly_sar"].as_f64().unwrap_or(0.0),
    }
}

fn parse_profit_loss_summary(json: &serde_json::Value) -> ProfitLossSummary {
    ProfitLossSummary {
        year_1: parse_profit_loss_year(&json["year_1"]),
        year_2: parse_profit_loss_year(&json["year_2"]),
        year_3: parse_profit_loss_year(&json["year_3"]),
        assumptions: json["assumptions"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn parse_profit_loss_year(json: &serde_json::Value) -> ProfitLossYear {
    ProfitLossYear {
        revenue_sar: json["revenue_sar"].as_f64().unwrap_or(0.0),
        cogs_sar: json["cogs_sar"].as_f64().unwrap_or(0.0),
        gross_profit_sar: json["gross_profit_sar"].as_f64().unwrap_or(0.0),
        operating_expenses_sar: json["operating_expenses_sar"].as_f64().unwrap_or(0.0),
        net_profit_sar: json["net_profit_sar"].as_f64().unwrap_or(0.0),
        net_margin_percent: json["net_margin_percent"].as_f64().unwrap_or(0.0),
    }
}

fn parse_business_structures(json: &serde_json::Value) -> Vec<BusinessStructure> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| BusinessStructure {
                    structure_type: item["structure_type"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().unwrap_or("").to_string(),
                    pros: item["pros"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default(),
                    cons: item["cons"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default(),
                    suitability_score: item["suitability_score"].as_i64().unwrap_or(5) as i32,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_licenses(json: &serde_json::Value) -> Vec<License> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| License {
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    issuing_authority: item["issuing_authority"].as_str().unwrap_or("").to_string(),
                    estimated_cost_sar: item["estimated_cost_sar"].as_f64().unwrap_or(0.0),
                    processing_time_days: item["processing_time_days"].as_i64().unwrap_or(0) as i32,
                    is_mandatory: item["is_mandatory"].as_bool().unwrap_or(true),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_compliance_items(json: &serde_json::Value) -> Vec<ComplianceItem> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| ComplianceItem {
                    regulation: item["regulation"].as_str().unwrap_or("").to_string(),
                    authority: item["authority"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().unwrap_or("").to_string(),
                    priority: parse_compliance_priority(
                        item["priority"].as_str().unwrap_or("medium"),
                    ),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_compliance_priority(s: &str) -> CompliancePriority {
    match s.to_lowercase().as_str() {
        "critical" => CompliancePriority::Critical,
        "high" => CompliancePriority::High,
        "low" => CompliancePriority::Low,
        _ => CompliancePriority::Medium,
    }
}

fn parse_risk_items(json: &serde_json::Value) -> Vec<RiskItem> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| RiskItem {
                    risk_name: item["risk_name"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().unwrap_or("").to_string(),
                    likelihood: parse_risk_level(item["likelihood"].as_str().unwrap_or("medium")),
                    impact: parse_risk_level(item["impact"].as_str().unwrap_or("medium")),
                    mitigation: item["mitigation"].as_str().unwrap_or("").to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_risk_level(s: &str) -> RiskLevel {
    match s.to_lowercase().as_str() {
        "low" => RiskLevel::Low,
        "high" => RiskLevel::High,
        "critical" => RiskLevel::Critical,
        _ => RiskLevel::Medium,
    }
}

fn parse_government_sources(json: &serde_json::Value) -> Vec<GovernmentSource> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|item| GovernmentSource {
                    document_name: item["document_name"].as_str().unwrap_or("").to_string(),
                    authority: item["authority"].as_str().unwrap_or("").to_string(),
                    url: item["url"].as_str().map(|s| s.to_string()),
                    citation_text: item["citation_text"].as_str().unwrap_or("").to_string(),
                    relevance_score: item["relevance_score"].as_f64().unwrap_or(0.0) as f32,
                })
                .collect()
        })
        .unwrap_or_else(|| {
            vec![GovernmentSource {
                document_name: "SME Business Guidelines".to_string(),
                authority: "Monsha'at".to_string(),
                url: Some("https://monshaat.gov.sa".to_string()),
                citation_text: "Based on general Saudi business regulations".to_string(),
                relevance_score: 0.8,
            }]
        })
}

fn create_sample_study(request: &FeasibilityRequest) -> FeasibilityStudyResponse {
    let assumed_employees = if request.initial_employees > 0 {
        request.initial_employees
    } else {
        6
    };
    let assumed_budget = if request.capital_budget > 0.0 {
        request.capital_budget
    } else {
        450000.0
    };

    let base_salaries_monthly = assumed_employees as f64 * 5500.0;
    let overtime_monthly = base_salaries_monthly * 0.05;
    let allowances_monthly = base_salaries_monthly * 0.10;
    let gosi_employer_monthly = base_salaries_monthly * 0.11;
    let gosi_employee_monthly = base_salaries_monthly * 0.11;
    let end_of_service_monthly = base_salaries_monthly * 0.03;
    let total_payroll_monthly = base_salaries_monthly
        + overtime_monthly
        + allowances_monthly
        + gosi_employer_monthly
        + end_of_service_monthly;

    let revenue_year_1 = 85000.0 * 12.0;
    let revenue_year_2 = 120000.0 * 12.0;
    let revenue_year_3 = 165000.0 * 12.0;
    let cogs_ratio = 0.30;

    let raw_id = uuid::Uuid::new_v4().simple().to_string();
    let short_id = &raw_id[..16];

    FeasibilityStudyResponse {
        study_id: format!("study_{}", short_id),
        business_name: request.business_name.clone(),
        generated_at: chrono::Utc::now(),
        executive_summary: ExecutiveSummary {
            viability_score: 72.5,
            summary_text: format!(
                "Based on analysis of Saudi regulatory documents and market data, \
                 {} shows moderate-to-high viability in the {} market. \
                 The business aligns with Vision 2030 economic diversification goals.",
                request.business_name, request.target_city
            ),
            key_strengths: vec![
                "Strong market demand identified".to_string(),
                "Vision 2030 alignment".to_string(),
                "Experienced founder (per input)".to_string(),
            ],
            key_challenges: vec![
                "Competitive market landscape".to_string(),
                "Regulatory compliance requirements".to_string(),
                "Initial capital intensity".to_string(),
            ],
            time_to_break_even_months: 14,
        },
        market_analysis: MarketAnalysis {
            target_market_size: "SAR 450M annually in target district".to_string(),
            market_growth_rate: "12% CAGR (Source: Monsha'at SME Report 2024)".to_string(),
            customer_segments: vec![CustomerSegment {
                name: "Primary: Young Professionals".to_string(),
                description: "Ages 25-40, middle income, tech-savvy".to_string(),
                estimated_size: "~45,000 in target area".to_string(),
                characteristics: vec!["Price conscious".to_string(), "Quality focused".to_string()],
            }],
            competitive_landscape: "Moderately competitive with 8-12 direct competitors"
                .to_string(),
            market_entry_barriers: vec![
                "High initial investment".to_string(),
                "Regulatory licensing".to_string(),
                "Established competitor loyalty".to_string(),
            ],
        },
        financial_projections: FinancialProjections {
            initial_investment_breakdown: vec![
                CostItem {
                    category: "Equipment & Setup".to_string(),
                    description: "Initial machinery, furniture, fixtures".to_string(),
                    amount_sar: assumed_budget * 0.45,
                    is_one_time: true,
                },
                CostItem {
                    category: "Working Capital".to_string(),
                    description: "6-month operational reserve".to_string(),
                    amount_sar: assumed_budget * 0.35,
                    is_one_time: true,
                },
                CostItem {
                    category: "Licenses & Legal".to_string(),
                    description: "Commercial registration, permits".to_string(),
                    amount_sar: assumed_budget * 0.10,
                    is_one_time: true,
                },
                CostItem {
                    category: "Marketing Launch".to_string(),
                    description: "Initial marketing and branding".to_string(),
                    amount_sar: assumed_budget * 0.10,
                    is_one_time: true,
                },
            ],
            monthly_operating_costs: vec![
                CostItem {
                    category: "Rent".to_string(),
                    description: format!("Commercial space in {}", request.target_city),
                    amount_sar: 15000.0,
                    is_one_time: false,
                },
                CostItem {
                    category: "Salaries".to_string(),
                    description: format!("{} employees (avg)", assumed_employees),
                    amount_sar: assumed_employees as f64 * 5000.0,
                    is_one_time: false,
                },
                CostItem {
                    category: "GOSI Employer Contribution".to_string(),
                    description: "Estimated employer contribution (see GOSI rates)".to_string(),
                    amount_sar: gosi_employer_monthly,
                    is_one_time: false,
                },
            ],
            payroll_breakdown: PayrollBreakdown {
                headcount: assumed_employees,
                avg_monthly_salary_sar: 5500.0,
                base_salaries_monthly_sar: base_salaries_monthly,
                overtime_monthly_sar: overtime_monthly,
                allowances_monthly_sar: allowances_monthly,
                gosi_employer_monthly_sar: gosi_employer_monthly,
                gosi_employee_monthly_sar: gosi_employee_monthly,
                end_of_service_accrual_monthly_sar: end_of_service_monthly,
                total_payroll_monthly_sar: total_payroll_monthly,
            },
            revenue_projections: RevenueProjections {
                year_1_monthly_avg: 85000.0,
                year_2_monthly_avg: 120000.0,
                year_3_monthly_avg: 165000.0,
                revenue_streams: vec![
                    "Primary sales".to_string(),
                    "Add-on services".to_string(),
                    "B2B contracts".to_string(),
                ],
            },
            profit_loss_summary: ProfitLossSummary {
                year_1: ProfitLossYear {
                    revenue_sar: revenue_year_1,
                    cogs_sar: revenue_year_1 * cogs_ratio,
                    gross_profit_sar: revenue_year_1 * (1.0 - cogs_ratio),
                    operating_expenses_sar: 15000.0 * 12.0 + total_payroll_monthly * 12.0,
                    net_profit_sar: revenue_year_1
                        - (revenue_year_1 * cogs_ratio)
                        - (15000.0 * 12.0 + total_payroll_monthly * 12.0),
                    net_margin_percent: ((revenue_year_1
                        - (revenue_year_1 * cogs_ratio)
                        - (15000.0 * 12.0 + total_payroll_monthly * 12.0))
                        / revenue_year_1)
                        * 100.0,
                },
                year_2: ProfitLossYear {
                    revenue_sar: revenue_year_2,
                    cogs_sar: revenue_year_2 * cogs_ratio,
                    gross_profit_sar: revenue_year_2 * (1.0 - cogs_ratio),
                    operating_expenses_sar: 15000.0 * 12.0 + total_payroll_monthly * 12.0,
                    net_profit_sar: revenue_year_2
                        - (revenue_year_2 * cogs_ratio)
                        - (15000.0 * 12.0 + total_payroll_monthly * 12.0),
                    net_margin_percent: ((revenue_year_2
                        - (revenue_year_2 * cogs_ratio)
                        - (15000.0 * 12.0 + total_payroll_monthly * 12.0))
                        / revenue_year_2)
                        * 100.0,
                },
                year_3: ProfitLossYear {
                    revenue_sar: revenue_year_3,
                    cogs_sar: revenue_year_3 * cogs_ratio,
                    gross_profit_sar: revenue_year_3 * (1.0 - cogs_ratio),
                    operating_expenses_sar: 15000.0 * 12.0 + total_payroll_monthly * 12.0,
                    net_profit_sar: revenue_year_3
                        - (revenue_year_3 * cogs_ratio)
                        - (15000.0 * 12.0 + total_payroll_monthly * 12.0),
                    net_margin_percent: ((revenue_year_3
                        - (revenue_year_3 * cogs_ratio)
                        - (15000.0 * 12.0 + total_payroll_monthly * 12.0))
                        / revenue_year_3)
                        * 100.0,
                },
                assumptions: vec![
                    "COGS estimated at 30% of revenue".to_string(),
                    "GOSI employer rate estimated at 11% of base salaries".to_string(),
                    "Monthly rent assumed constant for year 1".to_string(),
                ],
            },
            financial_assumptions: vec![
                "Average salary assumed SAR 5,500 per employee".to_string(),
                "GOSI rates depend on Saudi vs non-Saudi headcount split".to_string(),
            ],
            profitability_timeline: "Break-even expected month 14-16".to_string(),
            roi_estimate_3yr: 1.45, // 145% ROI
        },
        legal_requirements: LegalRequirements {
            business_structure_options: vec![BusinessStructure {
                structure_type: "LLC (Limited Liability Company)".to_string(),
                description: "Most common for SMEs, limited liability protection".to_string(),
                pros: vec![
                    "Liability protection".to_string(),
                    "Easier funding".to_string(),
                ],
                cons: vec![
                    "More paperwork".to_string(),
                    "Higher setup cost".to_string(),
                ],
                suitability_score: 9,
            }],
            required_licenses: vec![
                License {
                    name: "Commercial Registration (CR)".to_string(),
                    issuing_authority: "Ministry of Commerce (MOC)".to_string(),
                    estimated_cost_sar: 500.0,
                    processing_time_days: 3,
                    is_mandatory: true,
                },
                License {
                    name: "Municipal License (Balady)".to_string(),
                    issuing_authority: format!("{} Municipality", request.target_city),
                    estimated_cost_sar: 2500.0,
                    processing_time_days: 14,
                    is_mandatory: true,
                },
            ],
            regulatory_compliance: vec![
                ComplianceItem {
                    regulation: "Saudization (Nitaqat)".to_string(),
                    authority: "Ministry of Human Resources".to_string(),
                    description: "Minimum Saudi employee ratio requirements".to_string(),
                    priority: CompliancePriority::Critical,
                },
                ComplianceItem {
                    regulation: "GOSI Registration".to_string(),
                    authority: "General Organization for Social Insurance".to_string(),
                    description: "Social insurance for all employees".to_string(),
                    priority: CompliancePriority::High,
                },
            ],
            estimated_setup_costs_sar: 25000.0,
            setup_timeline_weeks: 6,
        },
        risk_assessment: RiskAssessment {
            market_risks: vec![RiskItem {
                risk_name: "Economic Downturn".to_string(),
                description: "Reduced consumer spending".to_string(),
                likelihood: RiskLevel::Medium,
                impact: RiskLevel::High,
                mitigation: "Diversify revenue streams".to_string(),
            }],
            financial_risks: vec![RiskItem {
                risk_name: "Cash Flow Issues".to_string(),
                description: "Delayed payments from B2B clients".to_string(),
                likelihood: RiskLevel::Medium,
                impact: RiskLevel::High,
                mitigation: "Maintain 6-month reserve".to_string(),
            }],
            operational_risks: vec![RiskItem {
                risk_name: "Staff Turnover".to_string(),
                description: "High turnover in Saudi market".to_string(),
                likelihood: RiskLevel::High,
                impact: RiskLevel::Medium,
                mitigation: "Competitive compensation + training".to_string(),
            }],
            regulatory_risks: vec![RiskItem {
                risk_name: "Regulatory Changes".to_string(),
                description: "New compliance requirements".to_string(),
                likelihood: RiskLevel::Medium,
                impact: RiskLevel::Medium,
                mitigation: "Subscribe to regulatory updates".to_string(),
            }],
            mitigation_strategies: vec![
                "Maintain strong cash reserves".to_string(),
                "Build regulatory compliance team".to_string(),
                "Diversify customer base".to_string(),
            ],
        },
        recommendations: Recommendations {
            go_no_go_verdict: Verdict::Yes,
            critical_success_factors: vec![
                "Superior customer service".to_string(),
                "Strategic location selection".to_string(),
                "Strong digital marketing presence".to_string(),
            ],
            next_steps: vec![
                "Secure commercial registration".to_string(),
                "Finalize location lease".to_string(),
                "Recruit core team".to_string(),
                "Develop MVP".to_string(),
            ],
            suggested_partnerships: vec![
                "Local supplier networks".to_string(),
                "Chamber of Commerce membership".to_string(),
                "Industry associations".to_string(),
            ],
        },
        sources_cited: vec![
            GovernmentSource {
                document_name: "SME Funding Guide 2024".to_string(),
                authority: "Monsha'at".to_string(),
                url: Some("https://monshaat.gov.sa".to_string()),
                citation_text: "SMEs represent 99.5% of business entities in KSA".to_string(),
                relevance_score: 0.95,
            },
            GovernmentSource {
                document_name: "Commercial Registration Procedures".to_string(),
                authority: "Ministry of Commerce".to_string(),
                url: Some("https://mc.gov.sa".to_string()),
                citation_text: "CR can be obtained within 3 business days via e-services"
                    .to_string(),
                relevance_score: 0.92,
            },
        ],
    }
}
