use reqwest::Client;
use tracing::{error, info, instrument};

use crate::config::AppConfig;

/// Anthropic Claude 3.5 Sonnet Service
///
/// Used for: Complex math calculations, structured JSON generation, coding tasks
/// Key capability: Superior reasoning and code generation
pub struct ClaudeService {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ClaudeService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            api_key: config.anthropic_api_key.clone(),
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    /// Generate structured feasibility study JSON
    #[instrument(skip(self, context))]
    pub async fn generate_feasibility_study(
        &self,
        business_request: &str,
        context: &str,
    ) -> anyhow::Result<serde_json::Value> {
        info!("Requesting feasibility study from Claude 3.5 Sonnet");

        if self.api_key.is_empty() {
            anyhow::bail!("Anthropic API key not configured");
        }

        let system_prompt = r#"You are a Saudi Arabian business feasibility expert. Your task is to analyze business ideas and create structured feasibility studies.

You MUST return ONLY valid JSON matching the required schema. Do not include any explanatory text outside the JSON.

The JSON must include:
- viability_score (0-100)
- executive_summary with strengths, challenges, time_to_break_even_months
- market_analysis with customer segments
- financial_projections with investment breakdown, payroll_breakdown, profit_loss_summary, and revenue projections
- legal_requirements specific to Saudi Arabia (Monshaat, Qiwa, Balady, GOSI)
- risk_assessment
- recommendations with go_no_go_verdict
- sources_cited (cite specific Saudi government sources when referenced)

All financial amounts should be in Saudi Riyals (SAR).
All regulatory references must be specific to Saudi Arabia.
Monthly operating costs must include line items for salaries, GOSI employer contribution, rent, utilities, insurance, marketing, and industry-specific COGS/inventory.
Payroll breakdown must reflect GOSI rates from the provided context and include assumptions for Saudi vs non-Saudi headcount.
If capital budget or employee count is missing or 0, estimate realistic values and note assumptions in financial_assumptions.
Use the provided context about Saudi regulations to inform your analysis."#;

        let user_prompt = format!(
            "Business Request: {}\n\nSaudi Regulatory Context:\n{}\n\nGenerate a complete feasibility study as structured JSON.",
            business_request,
            context
        );

        let request_body = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 8000,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Claude API error: {}", error_text);
        }

        let result: serde_json::Value = response.json().await?;
        let content = result["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No content in Claude response"))?;

        // Parse the JSON from Claude's response
        let study_json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| anyhow::anyhow!("Failed to parse Claude JSON response: {}", e))?;

        Ok(study_json)
    }

    /// Orchestrate persona debate
    pub async fn orchestrate_debate(
        &self,
        personas: &[PersonaDefinition],
        business_idea: &str,
        rounds: i32,
    ) -> anyhow::Result<serde_json::Value> {
        info!(
            "Orchestrating {}-round debate with {} personas",
            rounds,
            personas.len()
        );

        let system_prompt = format!(
            r#"You are moderating a debate between {} Saudi Arabian personas discussing a business idea.

Personas:
{}

You must simulate a realistic debate where each persona speaks in turn, expressing views consistent with their background.

Return ONLY JSON with this structure:
{{
  "debate_transcript": [
    {{
      "turn_number": 1,
      "persona_id": "...",
      "persona_name": "...",
      "message": "...",
      "sentiment": "supportive|skeptical|neutral|concerned|enthusiastic",
      "concerns_raised": ["..."]
    }}
  ],
  "consensus_summary": "...",
  "key_risks": ["..."],
  "key_opportunities": ["..."],
  "overall_verdict": "strong_yes|yes|neutral|no|strong_no"
}}"#,
            personas.len(),
            personas
                .iter()
                .map(|p| format!("- {} ({}): {}", p.id, p.name, p.role_description))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let user_prompt = format!(
            "Business Idea to Debate: {}\n\nSimulate a {}-round debate. Make it realistic and specific to the Saudi market context.",
            business_idea, rounds
        );

        let request_body = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 6000,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        let content = result["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No content in Claude response"))?;

        let debate_json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| anyhow::anyhow!("Failed to parse debate JSON: {}", e))?;

        Ok(debate_json)
    }

    /// Calculate financial projections with precision
    pub async fn calculate_projections(
        &self,
        parameters: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let system_prompt = r#"You are a financial analyst specializing in Saudi Arabian business projections.

Calculate detailed financial projections based on the provided parameters.
Return ONLY JSON with:
- initial_investment_breakdown (category, description, amount_sar, is_one_time)
- monthly_operating_costs
- revenue_projections (year 1, 2, 3 monthly averages)
- profitability_timeline
- roi_estimate_3yr (as decimal, e.g., 1.45 for 145%)

All amounts in SAR. Be realistic about Saudi market conditions."#;

        let request_body = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 4000,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": parameters
                }
            ]
        });

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        let content = result["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No content in response"))?;

        let projections: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| anyhow::anyhow!("Failed to parse projections: {}", e))?;

        Ok(projections)
    }
}

/// Persona definition for debate orchestration
pub struct PersonaDefinition {
    pub id: String,
    pub name: String,
    pub demographic_profile: String,
    pub role_description: String,
}
