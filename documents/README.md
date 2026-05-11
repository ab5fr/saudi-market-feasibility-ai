# Document Storage for RAG System

This directory contains all documents used by the RAG (Retrieval-Augmented Generation) system to provide accurate, citeable information in feasibility studies.

## Directory Structure

```
documents/
├── 01-government/           # Saudi government documents (PRIORITY)
│   ├── monshaat/           # SME authority documents
│   ├── qiwa/               # Labor market platform
│   ├── balady/             # Municipal licenses
│   ├── gosi/               # Social insurance
│   ├── ministry_of_commerce/ # Commercial registration
│   └── zatca/              # Tax and Zakat
├── 02-feasibility-templates/ # Feasibility study templates
├── 03-regulations/         # Industry regulations
└── 04-research/            # Market research reports
```

## Where to Put Your Documents

### 1. Government Documents (For RAG Citations)

**Location:** `01-government/[authority]/`

These documents are CRITICAL for the RAG system. The AI will cite these as sources.

**Supported file types:**

- `.txt` - Plain text files (preferred, fastest processing)
- `.md` - Markdown files
- `.pdf` - PDF files (now fully supported - text extraction is automatic)

**Examples:**

```
01-government/monshaat/sme_funding_guide_2024.txt
01-government/monshaat/entrepreneurship_programs.txt
01-government/qiwa/employment_contracts_guide.txt
01-government/balady/commercial_license_requirements.txt
01-government/ministry_of_commerce/cr_registration_steps.txt
01-government/gosi/registration_procedures.txt
```

**What to include:**

- License application procedures
- Fee schedules
- Required documents checklists
- Regulatory compliance requirements
- Government incentive programs
- Saudization (Nitaqat) rules

### 2. Feasibility Study Templates

**Location:** `02-feasibility-templates/`

Template documents that show standard feasibility study formats.

**Examples:**

```
02-feasibility-templates/food_beverage_template.txt
02-feasibility-templates/retail_template.txt
02-feasibility-templates/service_business_template.txt
```

### 3. Industry Regulations

**Location:** `03-regulations/`

Industry-specific regulations and standards.

### 4. Market Research

**Location:** `04-research/`

Market research reports and analysis documents.

## How to Add Documents

### Method 1: Direct File Copy (Recommended for development)

```bash
# Copy your text files to the appropriate directory
cp monshaat_sme_guide.txt documents/01-government/monshaat/
cp license_fees_2024.txt documents/01-government/balady/
```

### Method 2: Using Docker (When running in containers)

The documents folder is mounted as a volume in Docker:

```yaml
volumes:
  - ./documents:/app/documents:ro
```

## Document Format Best Practices

### For Text Files (.txt)

1. **Use clear headers:**

   ```
   # Document Title
   Source: Monsha'at
   Date: 2024

   ## Section Title
   Content here...
   ```

2. **Include metadata at the top:**

   ```
   TITLE: SME Funding Programs
   AUTHORITY: Monsha'at
   URL: https://monshaat.gov.sa/funding
   DATE: 2024-01-15

   [Document content...]
   ```

3. **Structure with clear sections:**
   - Use `##` for main sections
   - Use `###` for subsections
   - Leave blank lines between paragraphs

### Example Document Format

```
TITLE: Commercial Registration Requirements
AUTHORITY: Ministry of Commerce
SOURCE: https://mc.gov.sa/en/registration
DATE: 2024

## Overview
To establish a business in Saudi Arabia, you must obtain a Commercial Registration (CR)...

## Required Documents
1. Copy of ID
2. Lease agreement
3....

## Fees
- Commercial Registration: SAR 500
-...

## Processing Time
3-5 business days for online applications.
```

## Document Processing

When the backend starts, it will:

1. Scan all documents in this directory
2. Chunk them into smaller segments (2000 chars each)
3. Create vector embeddings using OpenAI
4. Store in Qdrant vector database

The documents will then be searchable by the RAG system.

## Important Notes

1. **File encoding:** Use UTF-8 encoding for proper Arabic text support
2. **Large PDFs:** For PDF files over 50MB, consider converting to text first
3. **Updates:** Replace old documents with new versions; the system will re-index
4. **Permissions:** Ensure files are readable (chmod 644)

## Sample Documents Included

The system comes with sample documents for testing. Replace these with real documents:

- `sample_monshaat_sme_guide.txt`
- `sample_commercial_registration.txt`
- `sample_balady_license.txt`

## Troubleshooting

**Documents not being indexed:**

- Check file permissions
- Verify files are in supported formats (.txt, .md)
- Check backend logs: `docker-compose logs backend`

**Arabic text not displaying correctly:**

- Ensure files are saved with UTF-8 encoding
- Use a text editor that supports UTF-8 (VS Code, Notepad++)

**Too many documents causing slow startup:**

- Documents are processed asynchronously
- Large collections (1000+ docs) may take several minutes to index

## API Keys Reminder

Documents alone won't work without API keys! See main README for:

- OpenAI API key (for embeddings)
- Google Gemini API key (for document analysis)
- Anthropic Claude API key (for feasibility studies)
