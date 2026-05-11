"use client";

import React, { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Checkbox } from "@/components/ui/checkbox";
import { Textarea } from "@/components/ui/textarea";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { submitFullAnalysis } from "@/lib/api";
import {
  FeasibilityRequest,
  ApiResponse,
  FeasibilityStudyResponse,
  PersonaDebateResponse,
  CompetitorAnalysisResponse,
  BusinessModel,
  ExperienceLevel,
  MultiStepFormData,
} from "@/types";
import { Send, Loader2, Sparkles, MapPin, Building2 } from "lucide-react";

type AnalysisErrors = {
  study?: string;
  personas?: string;
  competitors?: string;
};

type AnalysisResult = {
  study?: ApiResponse<FeasibilityStudyResponse>;
  personas?: ApiResponse<PersonaDebateResponse>;
  competitors?: ApiResponse<CompetitorAnalysisResponse>;
  errors?: AnalysisErrors;
};

export function MultiStepForm() {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(
    null,
  );
  const [formData, setFormData] = useState<Partial<MultiStepFormData>>({
    business_model: "brick_and_mortar",
    founder_experience: "intermediate",
    include_competitor_analysis: true,
    include_persona_debate: true,
  });

  const quickTestIdeas: Array<{
    id: string;
    label: string;
    tone: "good" | "risky";
    description: string;
    data: MultiStepFormData;
  }> = [
    {
      id: "good-coffee",
      label: "Specialty Coffee Kiosk (Good)",
      tone: "good",
      description:
        "High-footfall kiosk serving specialty coffee and quick breakfast items near KAFD offices.",
      data: {
        business_name: "KAFD Specialty Coffee",
        description:
          "A compact specialty coffee kiosk focused on speed, quality, and repeat office traffic near KAFD. Menu includes espresso, pour-over, and grab-and-go breakfast. Target customers are young professionals and commuters. Emphasis on mobile ordering and loyalty rewards to drive daily repeat visits.",
        industry: "food",
        business_model: "brick_and_mortar",
        target_city: "Riyadh",
        district: "KAFD",
        capital_budget: 420000,
        initial_employees: 6,
        founder_experience: "experienced",
        contact_email: "test@example.com",
        include_competitor_analysis: true,
        include_persona_debate: true,
        specific_questions: [
          "What monthly rent range is realistic for a kiosk?",
          "How sensitive is demand to pricing vs speed?",
        ],
      },
    },
    {
      id: "good-logistics",
      label: "SME Last-Mile Delivery (Good)",
      tone: "good",
      description:
        "B2B delivery service for SMEs with subscription pricing and same-day guarantees.",
      data: {
        business_name: "Rapid SME Delivery",
        description:
          "A B2B last-mile delivery service focused on small and mid-sized retailers in Jeddah. The business offers subscription tiers with same-day delivery, tracking, and COD handling. Value proposition is reliability and transparent pricing for SMEs that cannot build their own fleets.",
        industry: "logistics",
        business_model: "b2b",
        target_city: "Jeddah",
        district: "Al-Andalus",
        capital_budget: 650000,
        initial_employees: 12,
        founder_experience: "intermediate",
        contact_email: "test@example.com",
        include_competitor_analysis: true,
        include_persona_debate: true,
        specific_questions: [
          "What are realistic fuel and maintenance costs per driver?",
          "What SLAs are expected by SME retailers?",
        ],
      },
    },
    {
      id: "risky-luxury-icecream",
      label: "Luxury Ice Cream in Industrial Zone (High Risk)",
      tone: "risky",
      description:
        "Premium dessert shop planned in a low-footfall industrial area with high rent.",
      data: {
        business_name: "Arctic Luxe Gelato",
        description:
          "A high-end gelato and dessert shop targeting premium customers, planned for an industrial zone with limited evening traffic. The concept relies on imported ingredients, a premium fit-out, and high rent. Marketing depends heavily on social media and delivery apps despite weak nearby residential demand.",
        industry: "food",
        business_model: "brick_and_mortar",
        target_city: "Dammam",
        district: "Industrial Area",
        capital_budget: 900000,
        initial_employees: 9,
        founder_experience: "beginner",
        contact_email: "test@example.com",
        include_competitor_analysis: true,
        include_persona_debate: true,
        specific_questions: [
          "What happens to profit if foot traffic is 40% below plan?",
          "Is delivery demand strong enough to offset location risk?",
        ],
      },
    },
    {
      id: "risky-vr-arcade",
      label: "High-End VR Arcade with Low Budget (High Risk)",
      tone: "risky",
      description:
        "Premium VR arcade with an underfunded budget and limited local demand.",
      data: {
        business_name: "Immersion VR Lounge",
        description:
          "A premium VR arcade concept with expensive hardware, planned for a smaller city with limited entertainment spend. The budget is tight relative to equipment, rent, and staffing needs, and customer volume is uncertain outside weekends and holidays.",
        industry: "tourism",
        business_model: "brick_and_mortar",
        target_city: "Tabuk",
        district: "Downtown",
        capital_budget: 250000,
        initial_employees: 4,
        founder_experience: "beginner",
        contact_email: "test@example.com",
        include_competitor_analysis: true,
        include_persona_debate: true,
        specific_questions: [
          "What is the minimum monthly revenue to cover fixed costs?",
          "Is the equipment capex realistic for this budget?",
        ],
      },
    },
  ];

  const updateFormData = (field: keyof MultiStepFormData, value: unknown) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  const applyQuickTest = (preset: MultiStepFormData) => {
    setFormData(preset);
    setAnalysisResult(null);
    setSubmitError(null);
  };

  const handleSubmit = async () => {
    setIsSubmitting(true);
    setSubmitError(null);

    try {
      const request: FeasibilityRequest = {
        business_name: formData.business_name || "",
        description: formData.description || "",
        target_city: formData.target_city || "",
        district: formData.district,
        capital_budget: Number(formData.capital_budget) || 0,
        industry: formData.industry || "",
        business_model: formData.business_model || "brick_and_mortar",
        initial_employees: Number(formData.initial_employees) || 0,
        founder_experience: formData.founder_experience || "beginner",
        contact_email: formData.contact_email || "",
        specific_questions: formData.specific_questions,
        include_competitor_analysis:
          formData.include_competitor_analysis || false,
        include_persona_debate: formData.include_persona_debate || false,
      };

      const result = await submitFullAnalysis(request);
      setAnalysisResult(result);

      const hasStudy = Boolean(result.study?.data);
      const hasPersonas = Boolean(result.personas?.data);
      const hasCompetitors = Boolean(result.competitors?.data);
      const hasAny = hasStudy || hasPersonas || hasCompetitors;

      if (!hasAny) {
        setSubmitError("All analysis requests failed. See details below.");
      }
    } catch (error) {
      setSubmitError(
        error instanceof Error ? error.message : "An error occurred",
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  const formatPercent = (value: number) => `${Math.round(value * 10) / 10}%`;

  const formatCurrency = (value: number) =>
    new Intl.NumberFormat("en-SA", {
      style: "currency",
      currency: "SAR",
      maximumFractionDigits: 0,
    }).format(value);

  const formatLabel = (value: string) =>
    value.replace(/_/g, " ").replace(/\b\w/g, (ch) => ch.toUpperCase());

  const study = analysisResult?.study?.data ?? null;
  const personas = analysisResult?.personas?.data ?? null;
  const competitors = analysisResult?.competitors?.data ?? null;

  const initialInvestmentTotal = study
    ? study.financial_projections.initial_investment_breakdown.reduce(
        (sum, item) => sum + item.amount_sar,
        0,
      )
    : 0;

  const monthlyOperatingTotal = study
    ? study.financial_projections.monthly_operating_costs.reduce(
        (sum, item) => sum + item.amount_sar,
        0,
      )
    : 0;

  const isFormValid = () =>
    Boolean(
      formData.business_name?.trim().length >= 3 &&
      formData.description?.trim().length >= 50 &&
      formData.industry?.trim().length > 0 &&
      formData.target_city?.trim().length > 0 &&
      Number(formData.capital_budget || 0) >= 0 &&
      Number(formData.initial_employees || 0) >= 0 &&
      /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.contact_email || ""),
    );

  return (
    <div className="space-y-8">
      <Card className="w-full">
        <CardHeader>
          <CardTitle>Quick Test Ideas</CardTitle>
          <CardDescription>
            Click a preset to fill the form, then edit any field before
            submitting.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid md:grid-cols-2 gap-4">
            {quickTestIdeas.map((idea) => (
              <div key={idea.id} className="rounded-lg border p-4 space-y-3">
                <div className="flex items-center justify-between gap-2">
                  <div className="text-sm font-semibold">{idea.label}</div>
                  <span
                    className={`text-xs font-semibold px-2 py-1 rounded-full ${
                      idea.tone === "good"
                        ? "bg-emerald-100 text-emerald-800"
                        : "bg-amber-100 text-amber-800"
                    }`}
                  >
                    {idea.tone === "good" ? "Good Idea" : "High Risk"}
                  </span>
                </div>
                <p className="text-sm text-slate-700">{idea.description}</p>
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => applyQuickTest(idea.data)}
                >
                  Use This Idea
                </Button>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      <Card className="w-full">
        <CardHeader>
          <CardTitle>Feasibility Study Form</CardTitle>
          <CardDescription>
            Fill in all fields below. You can edit any preset before submitting.
          </CardDescription>
        </CardHeader>

        <CardContent>
          <div className="space-y-8">
            <div className="space-y-4">
              <div className="flex items-center gap-2 text-sm font-semibold">
                <Building2 className="h-4 w-4" />
                Business Idea
              </div>
              <div className="space-y-2">
                <Label htmlFor="business_name">
                  Business Name <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="business_name"
                  placeholder="e.g., Al-Rashid Coffee Roasters"
                  value={formData.business_name || ""}
                  onChange={(e) =>
                    updateFormData("business_name", e.target.value)
                  }
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="description">
                  Business Description <span className="text-red-500">*</span>
                </Label>
                <Textarea
                  id="description"
                  placeholder="Describe your business idea in detail (min 50 characters). What problem does it solve? Who are your target customers?"
                  value={formData.description || ""}
                  onChange={(e) =>
                    updateFormData("description", e.target.value)
                  }
                  rows={5}
                />
                <p className="text-xs text-muted-foreground">
                  {formData.description?.length || 0}/5000 characters
                </p>
              </div>

              <div className="grid md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="industry">
                    Industry <span className="text-red-500">*</span>
                  </Label>
                  <Select
                    value={formData.industry}
                    onValueChange={(value) => updateFormData("industry", value)}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select industry" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="food">Food & Beverage</SelectItem>
                      <SelectItem value="retail">Retail</SelectItem>
                      <SelectItem value="tech">Technology</SelectItem>
                      <SelectItem value="healthcare">Healthcare</SelectItem>
                      <SelectItem value="education">Education</SelectItem>
                      <SelectItem value="construction">Construction</SelectItem>
                      <SelectItem value="manufacturing">
                        Manufacturing
                      </SelectItem>
                      <SelectItem value="logistics">Logistics</SelectItem>
                      <SelectItem value="tourism">Tourism</SelectItem>
                      <SelectItem value="other">Other</SelectItem>
                    </SelectContent>
                  </Select>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="business_model">Business Model</Label>
                  <Select
                    value={formData.business_model}
                    onValueChange={(value) =>
                      updateFormData("business_model", value as BusinessModel)
                    }
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select model" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="brick_and_mortar">
                        Physical Store
                      </SelectItem>
                      <SelectItem value="ecommerce">Online Only</SelectItem>
                      <SelectItem value="hybrid">Hybrid</SelectItem>
                      <SelectItem value="service_based">Service</SelectItem>
                      <SelectItem value="b2b">B2B</SelectItem>
                      <SelectItem value="marketplace">Marketplace</SelectItem>
                      <SelectItem value="subscription">Subscription</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div className="flex items-center gap-2 text-sm font-semibold">
                <MapPin className="h-4 w-4" />
                Location
              </div>
              <div className="grid md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="target_city">
                    Target City <span className="text-red-500">*</span>
                  </Label>
                  <Select
                    value={formData.target_city}
                    onValueChange={(value) =>
                      updateFormData("target_city", value)
                    }
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select a city" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="Riyadh">Riyadh</SelectItem>
                      <SelectItem value="Jeddah">Jeddah</SelectItem>
                      <SelectItem value="Dammam">Dammam</SelectItem>
                      <SelectItem value="Khobar">Khobar</SelectItem>
                      <SelectItem value="Mecca">Mecca</SelectItem>
                      <SelectItem value="Medina">Medina</SelectItem>
                      <SelectItem value="Abha">Abha</SelectItem>
                      <SelectItem value="Tabuk">Tabuk</SelectItem>
                      <SelectItem value="Taif">Taif</SelectItem>
                      <SelectItem value="Buraidah">Buraidah</SelectItem>
                    </SelectContent>
                  </Select>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="district">
                    District/Neighborhood (Optional)
                  </Label>
                  <Input
                    id="district"
                    placeholder="e.g., Al-Olaya, Rabwat Al-Riyadh"
                    value={formData.district || ""}
                    onChange={(e) => updateFormData("district", e.target.value)}
                  />
                  <p className="text-xs text-muted-foreground">
                    Providing a specific district helps with competitor analysis
                  </p>
                </div>
              </div>

              <div className="bg-blue-50 p-4 rounded-lg">
                <p className="text-sm text-blue-800">
                  <strong>Why location matters:</strong> Our AI will analyze
                  local competitors, market saturation, and regulatory
                  requirements specific to your chosen city.
                </p>
              </div>
            </div>

            <div className="space-y-4">
              <div className="flex items-center gap-2 text-sm font-semibold">
                <Sparkles className="h-4 w-4" />
                Financials
              </div>
              <div className="grid md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="capital_budget">Capital Budget (SAR)</Label>
                  <Input
                    id="capital_budget"
                    type="number"
                    min={0}
                    placeholder="e.g., 500000"
                    value={formData.capital_budget || ""}
                    onChange={(e) =>
                      updateFormData("capital_budget", Number(e.target.value))
                    }
                  />
                  <p className="text-xs text-muted-foreground">
                    Leave blank to let AI estimate an investment range.
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="initial_employees">Initial Employees</Label>
                  <Input
                    id="initial_employees"
                    type="number"
                    min={0}
                    max={10000}
                    placeholder="e.g., 5"
                    value={formData.initial_employees || ""}
                    onChange={(e) =>
                      updateFormData(
                        "initial_employees",
                        Number(e.target.value),
                      )
                    }
                  />
                  <p className="text-xs text-muted-foreground">
                    Leave blank to let AI suggest a starting headcount.
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="founder_experience">
                    Your Experience Level
                  </Label>
                  <Select
                    value={formData.founder_experience}
                    onValueChange={(value) =>
                      updateFormData(
                        "founder_experience",
                        value as ExperienceLevel,
                      )
                    }
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select experience level" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="beginner">
                        Beginner - No prior experience
                      </SelectItem>
                      <SelectItem value="intermediate">
                        Intermediate - Some experience
                      </SelectItem>
                      <SelectItem value="experienced">
                        Experienced - Significant experience
                      </SelectItem>
                      <SelectItem value="expert">
                        Expert - Industry veteran
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>

            <div className="space-y-4">
              <div className="flex items-center gap-2 text-sm font-semibold">
                <Send className="h-4 w-4" />
                Options & Submit
              </div>
              <div className="space-y-2">
                <Label htmlFor="contact_email">
                  Email Address <span className="text-red-500">*</span>
                </Label>
                <Input
                  id="contact_email"
                  type="email"
                  placeholder="your@email.com"
                  value={formData.contact_email || ""}
                  onChange={(e) =>
                    updateFormData("contact_email", e.target.value)
                  }
                />
                <p className="text-xs text-muted-foreground">
                  We will send your feasibility study to this email
                </p>
              </div>

              <div className="space-y-4">
                <Label>Analysis Options</Label>

                <div className="flex items-start space-x-3">
                  <Checkbox
                    id="include_persona_debate"
                    checked={formData.include_persona_debate}
                    onCheckedChange={(checked) =>
                      updateFormData(
                        "include_persona_debate",
                        checked as boolean,
                      )
                    }
                  />
                  <div className="space-y-1">
                    <Label
                      htmlFor="include_persona_debate"
                      className="font-normal cursor-pointer"
                    >
                      Include Virtual Audience Debate
                    </Label>
                    <p className="text-xs text-muted-foreground">
                      AI personas representing Saudi investors, students, and
                      business owners will debate your idea
                    </p>
                  </div>
                </div>

                <div className="flex items-start space-x-3">
                  <Checkbox
                    id="include_competitor_analysis"
                    checked={formData.include_competitor_analysis}
                    onCheckedChange={(checked) =>
                      updateFormData(
                        "include_competitor_analysis",
                        checked as boolean,
                      )
                    }
                  />
                  <div className="space-y-1">
                    <Label
                      htmlFor="include_competitor_analysis"
                      className="font-normal cursor-pointer"
                    >
                      Include Competitor Analysis
                    </Label>
                    <p className="text-xs text-muted-foreground">
                      We will search for real competitors in your target
                      location using Google Places and web search
                    </p>
                  </div>
                </div>
              </div>

              {submitError && (
                <div className="bg-red-50 text-red-800 p-3 rounded-lg text-sm">
                  Error: {submitError}
                </div>
              )}

              <Button
                onClick={handleSubmit}
                disabled={!isFormValid() || isSubmitting}
              >
                {isSubmitting ? (
                  <>
                    <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                    Analyzing...
                  </>
                ) : (
                  <>
                    <Send className="h-4 w-4 mr-2" />
                    Generate Feasibility Study
                  </>
                )}
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      {analysisResult && (
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Feasibility Study</CardTitle>
              <CardDescription>
                Executive summary and core feasibility metrics.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {analysisResult.errors?.study && (
                <div className="bg-red-50 text-red-800 p-3 rounded-lg text-sm">
                  Error: {analysisResult.errors.study}
                </div>
              )}

              {study && (
                <>
                  <div className="grid sm:grid-cols-3 gap-4">
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Viability Score
                      </div>
                      <div className="text-2xl font-semibold">
                        {Math.round(study.executive_summary.viability_score)}
                      </div>
                    </div>
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Break-even
                      </div>
                      <div className="text-2xl font-semibold">
                        {study.executive_summary.time_to_break_even_months} mo
                      </div>
                    </div>
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        3-Year ROI
                      </div>
                      <div className="text-2xl font-semibold">
                        {formatPercent(
                          study.financial_projections.roi_estimate_3yr * 100,
                        )}
                      </div>
                    </div>
                  </div>

                  <div className="space-y-2">
                    <div className="text-sm font-semibold">Summary</div>
                    <p className="text-sm text-slate-700">
                      {study.executive_summary.summary_text}
                    </p>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div>
                      <div className="text-sm font-semibold">Key Strengths</div>
                      <ul className="text-sm text-slate-700 list-disc pl-5">
                        {study.executive_summary.key_strengths.map(
                          (item, idx) => (
                            <li key={`strength-${idx}`}>{item}</li>
                          ),
                        )}
                      </ul>
                    </div>
                    <div>
                      <div className="text-sm font-semibold">
                        Key Challenges
                      </div>
                      <ul className="text-sm text-slate-700 list-disc pl-5">
                        {study.executive_summary.key_challenges.map(
                          (item, idx) => (
                            <li key={`challenge-${idx}`}>{item}</li>
                          ),
                        )}
                      </ul>
                    </div>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div>
                      <div className="text-sm font-semibold">
                        Market Snapshot
                      </div>
                      <div className="text-sm text-slate-700">
                        Target Market:{" "}
                        {study.market_analysis.target_market_size}
                      </div>
                      <div className="text-sm text-slate-700">
                        Growth Rate: {study.market_analysis.market_growth_rate}
                      </div>
                      <div className="text-sm text-slate-700">
                        Competitive Landscape:{" "}
                        {study.market_analysis.competitive_landscape}
                      </div>
                    </div>
                    <div>
                      <div className="text-sm font-semibold">
                        Recommendations
                      </div>
                      <div className="text-sm text-slate-700">
                        Verdict:{" "}
                        {formatLabel(study.recommendations.go_no_go_verdict)}
                      </div>
                      <ul className="text-sm text-slate-700 list-disc pl-5">
                        {study.recommendations.next_steps
                          .slice(0, 4)
                          .map((item, idx) => (
                            <li key={`next-step-${idx}`}>{item}</li>
                          ))}
                      </ul>
                    </div>
                  </div>

                  <div className="text-xs text-muted-foreground">
                    Sources cited: {study.sources_cited.length}
                  </div>

                  <div className="space-y-4">
                    <div className="text-sm font-semibold">
                      Detailed Cost Breakdown
                    </div>
                    <div className="grid md:grid-cols-2 gap-4">
                      <div className="rounded-lg border p-3 space-y-2">
                        <div className="text-sm font-semibold">
                          Initial Investment
                        </div>
                        {study.financial_projections
                          .initial_investment_breakdown.length === 0 ? (
                          <div className="text-sm text-slate-700">
                            No initial cost items returned.
                          </div>
                        ) : (
                          <ul className="text-sm text-slate-700 space-y-1">
                            {study.financial_projections.initial_investment_breakdown.map(
                              (item, idx) => (
                                <li key={`init-cost-${idx}`}>
                                  {item.category}:{" "}
                                  {formatCurrency(item.amount_sar)}
                                  {item.description
                                    ? ` - ${item.description}`
                                    : ""}
                                </li>
                              ),
                            )}
                          </ul>
                        )}
                        <div className="text-sm font-semibold">
                          Total: {formatCurrency(initialInvestmentTotal)}
                        </div>
                      </div>

                      <div className="rounded-lg border p-3 space-y-2">
                        <div className="text-sm font-semibold">
                          Monthly Operating Costs
                        </div>
                        {study.financial_projections.monthly_operating_costs
                          .length === 0 ? (
                          <div className="text-sm text-slate-700">
                            No monthly cost items returned.
                          </div>
                        ) : (
                          <ul className="text-sm text-slate-700 space-y-1">
                            {study.financial_projections.monthly_operating_costs.map(
                              (item, idx) => (
                                <li key={`monthly-cost-${idx}`}>
                                  {item.category}:{" "}
                                  {formatCurrency(item.amount_sar)}
                                  {` / month (≈ ${formatCurrency(
                                    item.amount_sar * 12,
                                  )} / year)`}
                                  {item.description
                                    ? ` - ${item.description}`
                                    : ""}
                                </li>
                              ),
                            )}
                          </ul>
                        )}
                        <div className="text-sm font-semibold">
                          Total: {formatCurrency(monthlyOperatingTotal)} / month
                        </div>
                      </div>
                    </div>

                    <div className="rounded-lg border p-3 space-y-2">
                      <div className="text-sm font-semibold">
                        Payroll & GOSI Breakdown
                      </div>
                      <div className="grid md:grid-cols-2 gap-2 text-sm text-slate-700">
                        <div>
                          Headcount:{" "}
                          {
                            study.financial_projections.payroll_breakdown
                              .headcount
                          }
                        </div>
                        <div>
                          Avg Salary:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .avg_monthly_salary_sar,
                          )}
                        </div>
                        <div>
                          Base Salaries:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .base_salaries_monthly_sar,
                          )}
                        </div>
                        <div>
                          Overtime:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .overtime_monthly_sar,
                          )}
                        </div>
                        <div>
                          Allowances:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .allowances_monthly_sar,
                          )}
                        </div>
                        <div>
                          GOSI Employer:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .gosi_employer_monthly_sar,
                          )}
                        </div>
                        <div>
                          GOSI Employee (ref):{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .gosi_employee_monthly_sar,
                          )}
                        </div>
                        <div>
                          End of Service:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .end_of_service_accrual_monthly_sar,
                          )}
                        </div>
                        <div className="font-semibold">
                          Total Payroll:{" "}
                          {formatCurrency(
                            study.financial_projections.payroll_breakdown
                              .total_payroll_monthly_sar,
                          )}
                        </div>
                      </div>
                    </div>

                    <div className="rounded-lg border p-3 space-y-2">
                      <div className="text-sm font-semibold">Profit & Loss</div>
                      <div className="grid md:grid-cols-3 gap-3 text-sm text-slate-700">
                        {[
                          {
                            label: "Year 1",
                            data: study.financial_projections
                              .profit_loss_summary.year_1,
                          },
                          {
                            label: "Year 2",
                            data: study.financial_projections
                              .profit_loss_summary.year_2,
                          },
                          {
                            label: "Year 3",
                            data: study.financial_projections
                              .profit_loss_summary.year_3,
                          },
                        ].map((year) => (
                          <div
                            key={year.label}
                            className="rounded-lg bg-slate-50 p-3"
                          >
                            <div className="text-sm font-semibold">
                              {year.label}
                            </div>
                            <div>
                              Revenue: {formatCurrency(year.data.revenue_sar)}
                            </div>
                            <div>
                              COGS: {formatCurrency(year.data.cogs_sar)}
                            </div>
                            <div>
                              Operating:{" "}
                              {formatCurrency(year.data.operating_expenses_sar)}
                            </div>
                            <div className="font-semibold">
                              Net Profit:{" "}
                              {formatCurrency(year.data.net_profit_sar)}
                            </div>
                            <div>
                              Net Margin:{" "}
                              {year.data.net_margin_percent.toFixed(1)}%
                            </div>
                          </div>
                        ))}
                      </div>
                      {study.financial_projections.profit_loss_summary
                        .assumptions.length > 0 && (
                        <div className="text-sm text-slate-700">
                          Assumptions:{" "}
                          {study.financial_projections.profit_loss_summary.assumptions.join(
                            ", ",
                          )}
                        </div>
                      )}
                      {study.financial_projections.financial_assumptions
                        .length > 0 && (
                        <div className="text-sm text-slate-700">
                          Financial Notes:{" "}
                          {study.financial_projections.financial_assumptions.join(
                            ", ",
                          )}
                        </div>
                      )}
                    </div>
                  </div>
                </>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Virtual Audience Debate</CardTitle>
              <CardDescription>
                Persona feedback from the Saudi market.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {analysisResult.errors?.personas && (
                <div className="bg-red-50 text-red-800 p-3 rounded-lg text-sm">
                  Error: {analysisResult.errors.personas}
                </div>
              )}

              {personas && (
                <>
                  <div className="grid sm:grid-cols-2 gap-4">
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Verdict
                      </div>
                      <div className="text-lg font-semibold">
                        {formatLabel(personas.overall_verdict)}
                      </div>
                    </div>
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Consensus
                      </div>
                      <div className="text-sm text-slate-700">
                        {personas.consensus_summary}
                      </div>
                    </div>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div>
                      <div className="text-sm font-semibold">Key Risks</div>
                      <ul className="text-sm text-slate-700 list-disc pl-5">
                        {personas.key_risks.map((item, idx) => (
                          <li key={`risk-${idx}`}>{item}</li>
                        ))}
                      </ul>
                    </div>
                    <div>
                      <div className="text-sm font-semibold">
                        Key Opportunities
                      </div>
                      <ul className="text-sm text-slate-700 list-disc pl-5">
                        {personas.key_opportunities.map((item, idx) => (
                          <li key={`opp-${idx}`}>{item}</li>
                        ))}
                      </ul>
                    </div>
                  </div>

                  <div className="space-y-2">
                    <div className="text-sm font-semibold">
                      Debate Highlights
                    </div>
                    <div className="space-y-2">
                      {personas.debate_transcript.slice(0, 6).map((turn) => (
                        <div
                          key={`${turn.persona_id}-${turn.turn_number}`}
                          className="rounded-lg border p-3"
                        >
                          <div className="text-sm font-semibold">
                            {turn.persona_name}
                          </div>
                          <div className="text-sm text-slate-700">
                            {turn.message}
                          </div>
                        </div>
                      ))}
                    </div>
                    {personas.debate_transcript.length > 6 && (
                      <div className="text-xs text-muted-foreground">
                        Showing 6 of {personas.debate_transcript.length}{" "}
                        comments.
                      </div>
                    )}
                  </div>
                </>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Competitor Analysis</CardTitle>
              <CardDescription>
                Local competitor insights and market saturation.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {analysisResult.errors?.competitors && (
                <div className="bg-red-50 text-red-800 p-3 rounded-lg text-sm">
                  Error: {analysisResult.errors.competitors}
                </div>
              )}

              {competitors && (
                <>
                  <div className="grid sm:grid-cols-3 gap-4">
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Market Saturation
                      </div>
                      <div className="text-2xl font-semibold">
                        {formatPercent(competitors.market_saturation_score)}
                      </div>
                    </div>
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Competitors Found
                      </div>
                      <div className="text-2xl font-semibold">
                        {competitors.competitors.length}
                      </div>
                    </div>
                    <div className="rounded-lg bg-slate-50 p-3">
                      <div className="text-xs text-muted-foreground">
                        Avg Rating
                      </div>
                      <div className="text-2xl font-semibold">
                        {competitors.online_presence_summary.avg_google_rating?.toFixed(
                          1,
                        ) || "N/A"}
                      </div>
                    </div>
                  </div>

                  <div>
                    <div className="text-sm font-semibold">
                      Market Gap Analysis
                    </div>
                    <p className="text-sm text-slate-700">
                      {competitors.market_gap_analysis}
                    </p>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div>
                      <div className="text-sm font-semibold">
                        Pricing Benchmarks
                      </div>
                      <div className="text-sm text-slate-700">
                        Average Range:{" "}
                        {competitors.pricing_benchmarks.average_price_range}
                      </div>
                      <div className="text-sm text-slate-700">
                        Recommendation:{" "}
                        {
                          competitors.pricing_benchmarks
                            .pricing_strategy_recommendation
                        }
                      </div>
                    </div>
                    <div>
                      <div className="text-sm font-semibold">
                        Top Competitors
                      </div>
                      {competitors.competitors.length === 0 ? (
                        <div className="text-sm text-slate-700">
                          No competitors returned.
                        </div>
                      ) : (
                        <ul className="text-sm text-slate-700 space-y-1">
                          {competitors.competitors
                            .slice(0, 5)
                            .map((item, idx) => (
                              <li key={`competitor-${idx}`}>
                                {item.name}{" "}
                                {item.rating
                                  ? `(Rating ${item.rating.toFixed(1)})`
                                  : ""}{" "}
                                - {formatLabel(item.threat_level)}
                              </li>
                            ))}
                        </ul>
                      )}
                    </div>
                  </div>

                  {competitors.differentiation_strategy.length > 0 && (
                    <div>
                      <div className="text-sm font-semibold">
                        How To Stand Out
                      </div>
                      <ul className="text-sm text-slate-700 list-disc pl-5">
                        {competitors.differentiation_strategy.map(
                          (item, idx) => (
                            <li key={`differentiation-${idx}`}>{item}</li>
                          ),
                        )}
                      </ul>
                    </div>
                  )}
                </>
              )}
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
