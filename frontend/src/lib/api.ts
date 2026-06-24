import {
  FeasibilityRequest,
  PersonaDebateResponse,
  FeasibilityStudyResponse,
  CompetitorAnalysisResponse,
  ChatRequest,
  ChatResponse,
  ApiResponse,
} from "../types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001/api";

async function readErrorMessage(response: Response): Promise<string> {
  const fallback = `API error: ${response.status}`;

  try {
    const text = await response.text();
    if (!text) {
      return fallback;
    }

    try {
      const parsed = JSON.parse(text) as { error?: string };
      if (parsed?.error) {
        return parsed.error;
      }
    } catch {
      // Not JSON, fall through to return raw text.
    }

    return text;
  } catch {
    return fallback;
  }
}

async function fetchApi<T>(
  endpoint: string,
  options?: RequestInit,
): Promise<ApiResponse<T>> {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    headers: {
      "Content-Type": "application/json",
    },
    ...options,
  });

  if (!response.ok) {
    throw new Error(await readErrorMessage(response));
  }

  return response.json();
}

export async function submitPersonaDebate(
  request: FeasibilityRequest,
): Promise<ApiResponse<PersonaDebateResponse>> {
  return fetchApi<PersonaDebateResponse>("/personas", {
    method: "POST",
    body: JSON.stringify(request),
  });
}

export async function submitRagStudy(
  request: FeasibilityRequest,
): Promise<ApiResponse<FeasibilityStudyResponse>> {
  return fetchApi<FeasibilityStudyResponse>("/rag-study", {
    method: "POST",
    body: JSON.stringify(request),
  });
}

export async function submitCompetitorAnalysis(
  request: FeasibilityRequest,
): Promise<ApiResponse<CompetitorAnalysisResponse>> {
  return fetchApi<CompetitorAnalysisResponse>("/competitors", {
    method: "POST",
    body: JSON.stringify(request),
  });
}

export async function submitChat(
  request: ChatRequest,
): Promise<ApiResponse<ChatResponse>> {
  return fetchApi<ChatResponse>("/chat", {
    method: "POST",
    body: JSON.stringify(request),
  });
}

export async function submitFullAnalysis(request: FeasibilityRequest): Promise<{
  personas?: ApiResponse<PersonaDebateResponse>;
  study?: ApiResponse<FeasibilityStudyResponse>;
  competitors?: ApiResponse<CompetitorAnalysisResponse>;
  errors?: {
    personas?: string;
    study?: string;
    competitors?: string;
  };
}> {
  const results: {
    personas?: ApiResponse<PersonaDebateResponse>;
    study?: ApiResponse<FeasibilityStudyResponse>;
    competitors?: ApiResponse<CompetitorAnalysisResponse>;
  } = {};
  const errors: {
    personas?: string;
    study?: string;
    competitors?: string;
  } = {};

  const run = async <T>(
    key: "personas" | "study" | "competitors",
    promise: Promise<ApiResponse<T>>,
  ) => {
    try {
      const response = await promise;
      if (response.success) {
        if (key === "personas") {
          results.personas = response as ApiResponse<PersonaDebateResponse>;
        } else if (key === "study") {
          results.study = response as ApiResponse<FeasibilityStudyResponse>;
        } else if (key === "competitors") {
          results.competitors =
            response as ApiResponse<CompetitorAnalysisResponse>;
        }
      } else {
        errors[key] = response.error || "Request failed";
      }
    } catch (error) {
      errors[key] = error instanceof Error ? error.message : "Request failed";
    }
  };

  const tasks: Promise<void>[] = [];

  if (request.include_persona_debate) {
    tasks.push(run("personas", submitPersonaDebate(request)));
  }

  tasks.push(run("study", submitRagStudy(request)));

  if (request.include_competitor_analysis) {
    tasks.push(run("competitors", submitCompetitorAnalysis(request)));
  }

  await Promise.all(tasks);

  return {
    ...results,
    errors: Object.keys(errors).length ? errors : undefined,
  };
}
