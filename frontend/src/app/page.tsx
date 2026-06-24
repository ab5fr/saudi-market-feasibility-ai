import { MultiStepForm } from "@/components/multi-step-form";
import {
  Lightbulb,
  Users,
  FileText,
  MapPin,
  MessageCircle,
} from "lucide-react";
import Link from "next/link";

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-50 to-slate-100">
      {/* Header */}
      <header className="border-b bg-white/50 backdrop-blur-sm sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="bg-primary rounded-lg p-2">
              <Lightbulb className="h-6 w-6 text-primary-foreground" />
            </div>
            <span className="font-bold text-xl text-slate-900">
              Saudi Market AI
            </span>
          </div>
          <nav className="hidden md:flex items-center gap-6 text-sm font-medium text-slate-600">
            <span className="flex items-center gap-1">
              <Users className="h-4 w-4" />
              Virtual Audience
            </span>
            <span className="flex items-center gap-1">
              <FileText className="h-4 w-4" />
              Feasibility Study
            </span>
            <span className="flex items-center gap-1">
              <MapPin className="h-4 w-4" />
              Competitor Analysis
            </span>
            <Link
              href="/chat"
              className="flex items-center gap-1 text-primary hover:underline"
            >
              <MessageCircle className="h-4 w-4" />
              Chat
            </Link>
          </nav>
        </div>
      </header>

      {/* Hero Section */}
      <section className="py-12 md:py-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-4xl mx-auto text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-extrabold text-slate-900 mb-6">
            Validate Your Business Idea
            <span className="block text-primary">for the Saudi Market</span>
          </h1>
          <p className="text-lg md:text-xl text-slate-600 max-w-2xl mx-auto">
            Get AI-powered feasibility studies with virtual Saudi audience
            feedback, RAG-based regulatory analysis, and real competitor
            research.
          </p>
        </div>

        {/* Features Grid */}
        <div className="max-w-5xl mx-auto grid md:grid-cols-4 gap-6 mb-16">
          <div className="bg-white rounded-xl p-6 shadow-sm border">
            <div className="bg-purple-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <Users className="h-6 w-6 text-purple-600" />
            </div>
            <h3 className="font-semibold text-slate-900 mb-2">
              Virtual Audience
            </h3>
            <p className="text-sm text-slate-600">
              AI personas representing Saudi investors, students, and business
              owners debate your idea.
            </p>
          </div>

          <div className="bg-white rounded-xl p-6 shadow-sm border">
            <div className="bg-blue-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <FileText className="h-6 w-6 text-blue-600" />
            </div>
            <h3 className="font-semibold text-slate-900 mb-2">
              RAG-Based Study
            </h3>
            <p className="text-sm text-slate-600">
              Financial and legal advice strictly based on Saudi government
              documents from Monsha&apos;at, Qiwa, etc.
            </p>
          </div>

          <div className="bg-white rounded-xl p-6 shadow-sm border">
            <div className="bg-green-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <MapPin className="h-6 w-6 text-green-600" />
            </div>
            <h3 className="font-semibold text-slate-900 mb-2">
              Competitor Analysis
            </h3>
            <p className="text-sm text-slate-600">
              Real competitor data from Google Places and web search for any
              Saudi city or district.
            </p>
          </div>

          <Link
            href="/chat"
            className="bg-white rounded-xl p-6 shadow-sm border hover:shadow-md hover:border-primary transition-all"
          >
            <div className="bg-orange-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <MessageCircle className="h-6 w-6 text-orange-600" />
            </div>
            <h3 className="font-semibold text-slate-900 mb-2">Document Chat</h3>
            <p className="text-sm text-slate-600">
              Ask questions about Saudi business requirements using our AI
              assistant powered by RAG.
            </p>
          </Link>
        </div>

        {/* Form Section */}
        <div className="max-w-3xl mx-auto">
          <MultiStepForm />
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t bg-white py-8 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto text-center text-sm text-slate-500">
          <p>
            Saudi Market AI - Empowering entrepreneurs with data-driven insights
          </p>
        </div>
      </footer>
    </main>
  );
}
