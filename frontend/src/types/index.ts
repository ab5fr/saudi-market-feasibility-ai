// Re-export types from shared types
// In production, this would import from the shared-types package

export type BusinessModel =
  | "brick_and_mortar"
  | "ecommerce"
  | "hybrid"
  | "service_based"
  | "b2b"
  | "marketplace"
  | "subscription";

export type ExperienceLevel =
  | "beginner"
  | "intermediate"
  | "experienced"
  | "expert";

export type Sentiment =
  | "supportive"
  | "skeptical"
  | "neutral"
  | "concerned"
  | "enthusiastic";

export type Verdict = "strong_yes" | "yes" | "neutral" | "no" | "strong_no";

export type CompliancePriority = "critical" | "high" | "medium" | "low";

export type RiskLevel = "low" | "medium" | "high" | "critical";

export type ThreatLevel = "low" | "medium" | "high" | "direct_competitor";

export interface FeasibilityRequest {
  business_name: string;
  description: string;
  target_city: string;
  district?: string;
  capital_budget: number;
  industry: string;
  business_model: BusinessModel;
  initial_employees: number;
  founder_experience: ExperienceLevel;
  contact_email: string;
  specific_questions?: string[];
  include_competitor_analysis: boolean;
  include_persona_debate: boolean;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
  timestamp: string;
}

export interface PersonaDebateResponse {
  session_id: string;
  business_name: string;
  personas: PersonaAgent[];
  debate_transcript: DebateTurn[];
  consensus_summary: string;
  key_risks: string[];
  key_opportunities: string[];
  overall_verdict: Verdict;
}

export interface PersonaAgent {
  id: string;
  name: string;
  demographic_profile: string;
  role_description: string;
  avatar_url?: string;
}

export interface DebateTurn {
  turn_number: number;
  persona_id: string;
  persona_name: string;
  message: string;
  sentiment: Sentiment;
  concerns_raised: string[];
}

export interface FeasibilityStudyResponse {
  study_id: string;
  business_name: string;
  generated_at: string;
  executive_summary: ExecutiveSummary;
  market_analysis: MarketAnalysis;
  financial_projections: FinancialProjections;
  legal_requirements: LegalRequirements;
  risk_assessment: RiskAssessment;
  recommendations: Recommendations;
  sources_cited: GovernmentSource[];
}

export interface ExecutiveSummary {
  viability_score: number;
  summary_text: string;
  key_strengths: string[];
  key_challenges: string[];
  time_to_break_even_months: number;
}

export interface MarketAnalysis {
  target_market_size: string;
  market_growth_rate: string;
  customer_segments: CustomerSegment[];
  competitive_landscape: string;
  market_entry_barriers: string[];
}

export interface CustomerSegment {
  name: string;
  description: string;
  estimated_size: string;
  characteristics: string[];
}

export interface FinancialProjections {
  initial_investment_breakdown: CostItem[];
  monthly_operating_costs: CostItem[];
  payroll_breakdown: PayrollBreakdown;
  revenue_projections: RevenueProjections;
  profit_loss_summary: ProfitLossSummary;
  financial_assumptions: string[];
  profitability_timeline: string;
  roi_estimate_3yr: number;
}

export interface CostItem {
  category: string;
  description: string;
  amount_sar: number;
  is_one_time: boolean;
}

export interface RevenueProjections {
  year_1_monthly_avg: number;
  year_2_monthly_avg: number;
  year_3_monthly_avg: number;
  revenue_streams: string[];
}

export interface PayrollBreakdown {
  headcount: number;
  avg_monthly_salary_sar: number;
  base_salaries_monthly_sar: number;
  overtime_monthly_sar: number;
  allowances_monthly_sar: number;
  gosi_employer_monthly_sar: number;
  gosi_employee_monthly_sar: number;
  end_of_service_accrual_monthly_sar: number;
  total_payroll_monthly_sar: number;
}

export interface ProfitLossSummary {
  year_1: ProfitLossYear;
  year_2: ProfitLossYear;
  year_3: ProfitLossYear;
  assumptions: string[];
}

export interface ProfitLossYear {
  revenue_sar: number;
  cogs_sar: number;
  gross_profit_sar: number;
  operating_expenses_sar: number;
  net_profit_sar: number;
  net_margin_percent: number;
}

export interface LegalRequirements {
  business_structure_options: BusinessStructure[];
  required_licenses: License[];
  regulatory_compliance: ComplianceItem[];
  estimated_setup_costs_sar: number;
  setup_timeline_weeks: number;
}

export interface BusinessStructure {
  structure_type: string;
  description: string;
  pros: string[];
  cons: string[];
  suitability_score: number;
}

export interface License {
  name: string;
  issuing_authority: string;
  estimated_cost_sar: number;
  processing_time_days: number;
  is_mandatory: boolean;
}

export interface ComplianceItem {
  regulation: string;
  authority: string;
  description: string;
  priority: CompliancePriority;
}

export interface RiskAssessment {
  market_risks: RiskItem[];
  financial_risks: RiskItem[];
  operational_risks: RiskItem[];
  regulatory_risks: RiskItem[];
  mitigation_strategies: string[];
}

export interface RiskItem {
  risk_name: string;
  description: string;
  likelihood: RiskLevel;
  impact: RiskLevel;
  mitigation: string;
}

export interface Recommendations {
  go_no_go_verdict: Verdict;
  critical_success_factors: string[];
  next_steps: string[];
  suggested_partnerships: string[];
}

export interface GovernmentSource {
  document_name: string;
  authority: string;
  url?: string;
  citation_text: string;
  relevance_score: number;
}

export interface CompetitorAnalysisResponse {
  analysis_id: string;
  business_name: string;
  search_location: string;
  search_query_used: string;
  competitors: Competitor[];
  market_saturation_score: number;
  market_gap_analysis: string;
  differentiation_strategy: string[];
  pricing_benchmarks: PricingBenchmarks;
  online_presence_summary: OnlinePresenceSummary;
}

export interface Competitor {
  name: string;
  location: string;
  distance_km?: number;
  business_type: string;
  rating?: number;
  review_count?: number;
  price_level?: number;
  website?: string;
  phone?: string;
  strengths: string[];
  weaknesses: string[];
  threat_level: ThreatLevel;
}

export interface PricingBenchmarks {
  average_price_range: string;
  lowest_observed: string;
  highest_observed: string;
  pricing_strategy_recommendation: string;
}

export interface OnlinePresenceSummary {
  total_competitors_found: number;
  avg_google_rating?: number;
  competitors_with_websites: number;
  social_media_presence: string;
  online_reputation_summary: string;
}

export interface FormStep1Data {
  business_name: string;
  description: string;
  industry: string;
  business_model: BusinessModel;
}

export interface FormStep2Data {
  target_city: string;
  district?: string;
}

export interface FormStep3Data {
  capital_budget: number;
  initial_employees: number;
  founder_experience: ExperienceLevel;
}

export interface FormStep4Data {
  contact_email: string;
  include_competitor_analysis: boolean;
  include_persona_debate: boolean;
  specific_questions?: string[];
}

export type MultiStepFormData = FormStep1Data &
  FormStep2Data &
  FormStep3Data &
  FormStep4Data;
