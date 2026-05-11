use reqwest::Client;
use tracing::{error, info, instrument};

use crate::config::AppConfig;

/// Google Gemini 1.5 Pro Service
///
/// Used for: RAG pipeline document analysis
/// Key capability: Massive context window for Arabic PDF documents
pub struct GeminiService {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl GeminiService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.gemini_api_key.clone(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            model: config.gemini_model.clone(),
        }
    }

    /// Analyze documents with long context window
    /// Gemini 1.5 Pro supports up to 1M+ token context window
    #[instrument(skip(self, documents))]
    pub async fn analyze_documents(
        &self,
        query: &str,
        documents: Vec<String>,
    ) -> anyhow::Result<String> {
        info!(
            "Sending {} documents to Gemini 1.5 Pro for analysis",
            documents.len()
        );

        if self.api_key.is_empty() {
            anyhow::bail!("Gemini API key not configured");
        }

        // Combine documents into context
        let context = documents.join("\n\n---\n\n");
        let full_prompt = format!(
            "Context from Saudi government documents:\n{}\n\nUser Query: {}\n\nProvide a detailed answer based ONLY on the context provided. Cite specific document sources where applicable. If the context doesn't contain relevant information, say so.",
            context, query
        );

        let request_body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": full_prompt
                        }
                    ]
                }
            ],
            "generationConfig": {
                "temperature": 0.1,
                "topK": 1,
                "topP": 1,
                "maxOutputTokens": 8192
            }
        });

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response"))?;

        Ok(text.to_string())
    }

    /// Generate response grounded in provided context (RAG-style)
    pub async fn generate_grounded_response(
        &self,
        prompt: &str,
        context: &[String],
    ) -> anyhow::Result<String> {
        info!(
            "Generating grounded response with {} context chunks",
            context.len()
        );

        // Create a system instruction for grounded responses
        let system_instruction = "You are a Saudi Arabian regulatory expert. Answer the user's question using ONLY the provided context from Saudi government documents. If you cannot find the answer in the context, clearly state that. Always cite the specific source document when providing information.";

        let combined_context = context.join("\n\n");

        let full_prompt = format!(
            "System: {}\n\nContext:\n{}\n\nUser Question: {}\n\nAnswer:",
            system_instruction, combined_context, prompt
        );

        let request_body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": full_prompt
                        }
                    ]
                }
            ],
            "generationConfig": {
                "temperature": 0.0, // Very deterministic for factual accuracy
                "maxOutputTokens": 4096
            }
        });

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response"))?;

        Ok(text.to_string())
    }

    /// Extract structured information from Saudi documents
    pub async fn extract_regulatory_info(
        &self,
        document_text: &str,
        business_type: &str,
    ) -> anyhow::Result<serde_json::Value> {
        info!("Extracting regulatory info for {} business", business_type);

        let prompt = format!(
            r#"Extract regulatory requirements for a {} business from this Saudi government document.

Document Content:
{}

Extract and return ONLY JSON with this structure:
{{
  "licenses_required": [{{"name": "...", "authority": "...", "estimated_cost_sar": number, "processing_days": number}}],
  "compliance_requirements": [{{"regulation": "...", "description": "...", "priority": "critical|high|medium|low"}}],
  "business_structures": [{{"type": "...", "pros": ["..."], "cons": ["..."]}}],
  "estimated_setup_costs_sar": number,
  "setup_timeline_weeks": number,
  "key_insights": ["..."]
}}"#,
            business_type, document_text
        );

        let request_body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": prompt}]
                }
            ],
            "generationConfig": {
                "temperature": 0.1,
                "maxOutputTokens": 4096
            }
        });

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        let text = result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No text in response"))?;

        // Try to extract JSON from the response (it might be wrapped in markdown)
        let json_str = if text.contains("```json") {
            text.split("```json")
                .nth(1)
                .unwrap_or(text)
                .split("```")
                .next()
                .unwrap_or(text)
        } else if text.contains("```") {
            text.split("```").nth(1).unwrap_or(text)
        } else {
            text
        };

        let parsed: serde_json::Value = serde_json::from_str(json_str.trim())
            .map_err(|e| anyhow::anyhow!("Failed to parse extracted JSON: {}", e))?;

        Ok(parsed)
    }

    /// Generate a complete feasibility study using Gemini
    /// This replaces Claude for users who only have Gemini API key
    pub async fn generate_feasibility_study(
        &self,
        business_request: &str,
        context: &str,
    ) -> anyhow::Result<serde_json::Value> {
        info!("Generating feasibility study with Gemini 1.5 Pro");

        if self.api_key.is_empty() {
            anyhow::bail!("Gemini API key not configured");
        }

        let prompt = format!(
            r#"You are a Saudi Arabian business feasibility expert. Generate a comprehensive feasibility study based on the following business request and regulatory context.

BUSINESS REQUEST:
{}

SAUDI REGULATORY CONTEXT:
{}

Generate a detailed feasibility study in STRICT JSON format with this exact structure:
{{
  "executive_summary": {{
    "viability_score": 75.0,
    "summary_text": "Detailed summary here...",
    "key_strengths": ["strength1", "strength2", "strength3"],
    "key_challenges": ["challenge1", "challenge2", "challenge3"],
    "time_to_break_even_months": 14
  }},
  "market_analysis": {{
    "target_market_size": "SAR XXXM annually",
    "market_growth_rate": "X% CAGR",
    "customer_segments": [
      {{
        "name": "Segment name",
        "description": "Description",
        "estimated_size": "Size",
        "characteristics": ["trait1", "trait2"]
      }}
    ],
    "competitive_landscape": "Description of competition",
    "market_entry_barriers": ["barrier1", "barrier2"]
  }},
  "financial_projections": {{
    "initial_investment_breakdown": [
      {{"category": "Equipment", "description": "Details", "amount_sar": 100000, "is_one_time": true}}
    ],
    "monthly_operating_costs": [
      {{"category": "Rent", "description": "Details", "amount_sar": 15000, "is_one_time": false}}
    ],
        "payroll_breakdown": {{
            "headcount": 6,
            "avg_monthly_salary_sar": 5500,
            "base_salaries_monthly_sar": 33000,
            "overtime_monthly_sar": 1500,
            "allowances_monthly_sar": 3000,
            "gosi_employer_monthly_sar": 3630,
            "gosi_employee_monthly_sar": 3630,
            "end_of_service_accrual_monthly_sar": 900,
            "total_payroll_monthly_sar": 42030
        }},
    "revenue_projections": {{
      "year_1_monthly_avg": 50000,
      "year_2_monthly_avg": 75000,
      "year_3_monthly_avg": 100000,
      "revenue_streams": ["Primary sales", "Add-on services"]
    }},
        "profit_loss_summary": {{
            "year_1": {{
                "revenue_sar": 600000,
                "cogs_sar": 180000,
                "gross_profit_sar": 420000,
                "operating_expenses_sar": 240000,
                "net_profit_sar": 180000,
                "net_margin_percent": 30.0
            }},
            "year_2": {{
                "revenue_sar": 900000,
                "cogs_sar": 270000,
                "gross_profit_sar": 630000,
                "operating_expenses_sar": 260000,
                "net_profit_sar": 370000,
                "net_margin_percent": 41.1
            }},
            "year_3": {{
                "revenue_sar": 1200000,
                "cogs_sar": 360000,
                "gross_profit_sar": 840000,
                "operating_expenses_sar": 280000,
                "net_profit_sar": 560000,
                "net_margin_percent": 46.7
            }},
            "assumptions": ["COGS based on industry norm", "Rent fixed for year 1"]
        }},
        "financial_assumptions": ["GOSI rates applied per Saudi/non-Saudi split"],
    "profitability_timeline": "Break-even expected month XX",
    "roi_estimate_3yr": 1.35
  }},
  "legal_requirements": {{
    "business_structure_options": [
      {{
        "structure_type": "LLC",
        "description": "Limited Liability Company",
        "pros": ["Liability protection"],
        "cons": ["More paperwork"],
        "suitability_score": 9
      }}
    ],
    "required_licenses": [
      {{
        "name": "Commercial Registration",
        "issuing_authority": "Ministry of Commerce",
        "estimated_cost_sar": 500,
        "processing_time_days": 3,
        "is_mandatory": true
      }}
    ],
    "regulatory_compliance": [
      {{
        "regulation": "Saudization",
        "authority": "Ministry of HR",
        "description": "Saudi employee ratio requirements",
        "priority": "critical"
      }}
    ],
    "estimated_setup_costs_sar": 25000,
    "setup_timeline_weeks": 6
  }},
  "risk_assessment": {{
    "market_risks": [
      {{"risk_name": "Competition", "description": "Market saturation", "likelihood": "medium", "impact": "high", "mitigation": "Differentiation strategy"}}
    ],
    "financial_risks": [
      {{"risk_name": "Cash Flow", "description": "Working capital needs", "likelihood": "medium", "impact": "high", "mitigation": "Maintain reserves"}}
    ],
    "operational_risks": [
      {{"risk_name": "Staffing", "description": "Finding qualified employees", "likelihood": "medium", "impact": "medium", "mitigation": "Training programs"}}
    ],
    "regulatory_risks": [
      {{"risk_name": "Compliance Changes", "description": "New regulations", "likelihood": "low", "impact": "medium", "mitigation": "Stay updated"}}
    ],
    "mitigation_strategies": ["Strategy 1", "Strategy 2"]
  }},
  "recommendations": {{
    "go_no_go_verdict": "yes",
    "critical_success_factors": ["Factor 1", "Factor 2"],
    "next_steps": ["Step 1", "Step 2", "Step 3"],
    "suggested_partnerships": ["Partner 1", "Partner 2"]
  }},
  "sources_cited": [
    {{
      "document_name": "Document Name",
      "authority": "Authority Name",
      "url": "https://example.com",
      "citation_text": "Relevant quote",
      "relevance_score": 0.95
    }}
  ]
}}

IMPORTANT:
1. Use realistic SAR amounts based on Saudi market rates
2. All licenses and regulations must be real Saudi requirements
3. Use "yes", "no", "strong_yes", "strong_no", or "neutral" for verdict
4. Viability score should be 0-100
5. Be specific to the Saudi Arabian market and the business type described
6. Monthly operating costs MUST include line items for salaries, GOSI employer contribution, rent, utilities, insurance, marketing, and industry-specific COGS/inventory
7. Payroll breakdown MUST reflect GOSI rates from provided context; include assumptions for Saudi vs non-Saudi headcount
8. If capital budget or employee count is missing or 0, estimate realistic values and note assumptions in financial_assumptions
9. Return ONLY the JSON, no markdown formatting"#,
            business_request, context
        );

        let request_body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": prompt}]
                }
            ],
            "generationConfig": {
                "temperature": 0.2,
                "maxOutputTokens": 8192
            }
        });

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response"))?;

        // Extract JSON from response
        let json_str = if text.contains("```json") {
            text.split("```json")
                .nth(1)
                .unwrap_or(text)
                .split("```")
                .next()
                .unwrap_or(text)
        } else if text.contains("```") {
            text.split("```").nth(1).unwrap_or(text)
        } else {
            text
        };

        let study_json: serde_json::Value = serde_json::from_str(json_str.trim()).map_err(|e| {
            error!("Failed to parse Gemini response as JSON: {}", e);
            error!("Response was: {}", text);
            anyhow::anyhow!("Failed to parse feasibility study JSON: {}", e)
        })?;

        Ok(study_json)
    }

    /// Generate differentiation strategy based on competitor context
    #[instrument(skip(self, prompt))]
    pub async fn generate_differentiation_strategy(
        &self,
        prompt: &str,
    ) -> anyhow::Result<Vec<String>> {
        info!("Generating differentiation strategy with Gemini");

        if self.api_key.is_empty() {
            anyhow::bail!("Gemini API key not configured");
        }

        let request_body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": prompt}]
                }
            ],
            "generationConfig": {
                "temperature": 0.2,
                "maxOutputTokens": 1024
            }
        });

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response"))?;

        let json_str = if text.contains("```json") {
            text.split("```json")
                .nth(1)
                .unwrap_or(text)
                .split("```")
                .next()
                .unwrap_or(text)
        } else if text.contains("```") {
            text.split("```").nth(1).unwrap_or(text)
        } else {
            text
        };

        let parsed: serde_json::Value = serde_json::from_str(json_str.trim())
            .map_err(|e| anyhow::anyhow!("Failed to parse differentiation JSON: {}", e))?;

        let strategies = parsed["differentiation_strategy"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(strategies)
    }

    /// Orchestrate a persona debate using Gemini
    /// This replaces Claude for users who only have Gemini API key
    pub async fn orchestrate_debate(
        &self,
        business_request: &str,
        personas: &[serde_json::Value],
    ) -> anyhow::Result<serde_json::Value> {
        info!(
            "Orchestrating persona debate with Gemini using {} personas",
            personas.len()
        );

        if self.api_key.is_empty() {
            anyhow::bail!("Gemini API key not configured");
        }

        // Format personas for the prompt
        let personas_desc = personas
            .iter()
            .map(|p| {
                format!(
                    "- {} ({}): {}",
                    p["name"].as_str().unwrap_or("Unknown"),
                    p["role"].as_str().unwrap_or("Role"),
                    p["perspective"].as_str().unwrap_or("Perspective")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"You are simulating a panel discussion of Saudi Arabian business experts evaluating a new business idea. Each expert has a different background and perspective.

BUSINESS PROPOSAL:
{}

PANEL MEMBERS:
{}

Generate a debate transcript where each panel member speaks at least once, sharing their perspective on the business. Then provide a consensus summary.

Return STRICT JSON with this exact structure:
{{
  "debate_transcript": [
    {{
      "turn_number": 1,
      "persona_id": "inv_001",
      "persona_name": "Name",
      "message": "Their detailed opinion...",
      "sentiment": "supportive|skeptical|neutral|concerned|enthusiastic",
      "concerns_raised": ["concern1", "concern2"],
      "opportunities_identified": ["opp1", "opp2"]
    }}
  ],
  "consensus_summary": "Summary of the panel's collective opinion...",
  "key_risks": ["risk1", "risk2", "risk3"],
  "key_opportunities": ["opp1", "opp2", "opp3"],
  "overall_verdict": "yes|no|strong_yes|strong_no|neutral",
  "confidence_score": 0.75
}}

IMPORTANT:
- Each persona should have distinct voice matching their background
- Include realistic Saudi market insights
- Verdict options: "strong_yes", "yes", "neutral", "no", "strong_no"
- Confidence score 0.0-1.0
- Return ONLY the JSON, no markdown"#,
            business_request, personas_desc
        );

        let request_body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": prompt}]
                }
            ],
            "generationConfig": {
                "temperature": 0.7, // Higher for creative debate
                "maxOutputTokens": 4096
            }
        });

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response"))?;

        // Extract JSON from response
        let json_str = if text.contains("```json") {
            text.split("```json")
                .nth(1)
                .unwrap_or(text)
                .split("```")
                .next()
                .unwrap_or(text)
        } else if text.contains("```") {
            text.split("```").nth(1).unwrap_or(text)
        } else {
            text
        };

        let debate_json: serde_json::Value =
            serde_json::from_str(json_str.trim()).map_err(|e| {
                error!("Failed to parse Gemini debate response: {}", e);
                error!("Response was: {}", text);
                anyhow::anyhow!("Failed to parse debate JSON: {}", e)
            })?;

        Ok(debate_json)
    }
}
