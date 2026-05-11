# How to Add Your Monsha'at PDF Template

You have a PDF feasibility study template from Monsha'at and want to add it to the RAG system. Here's how:

## ✅ PDF Support is Now Enabled!

The backend now supports PDF files directly. You can simply copy your PDF to the correct folder.

## Quick Steps

### Step 1: Place Your PDF in the Correct Folder

```powershell
# Copy your PDF to the Monsha'at folder
copy "C:\Path\To\Your\Monshaat_Template.pdf" "C:\Users\ab5\CascadeProjects\saudi-market-feasibility\documents\01-government\monshaat\"
```

Or use File Explorer:
1. Navigate to `documents\01-government\monshaat\`
2. Copy your PDF file there

### Step 2: Rebuild and Restart Backend

The backend needs to be rebuilt to include the new PDF dependency:

```powershell
# Navigate to backend
cd C:\Users\ab5\CascadeProjects\saudi-market-feasibility\backend

# Rebuild with new PDF support
cargo build

# Run the backend
cargo run
```

### Step 3: PDF Will Be Automatically Indexed

When the backend starts:
1. It will scan all files in `documents/`
2. Extract text from your PDF
3. Create embeddings using OpenAI
4. Store in Qdrant vector database
5. Ready for RAG citations!

**Check the logs to see it processing:**
```
Processing document: /app/documents/01-government/monshaat/Monshaat_Template.pdf
Extracting text from PDF: /app/documents/01-government/monshaat/Monshaat_Template.pdf
Created 42 chunks from document
Stored 42 embeddings in Qdrant
```

---

## 📁 Where Exactly to Put It

### For Monsha'at Documents:
```
documents/01-government/monshaat/
├── sme_funding_programs.txt          (existing)
├── Monshaat_Template.pdf             <-- YOUR PDF HERE
└── other_monshaat_docs.pdf           (more PDFs)
```

### For Other Authorities:
```
documents/01-government/
├── monshaat/              # SME authority
│   └── YOUR_PDF.pdf
├── ministry_of_commerce/  # Commercial registration
│   └── YOUR_PDF.pdf
├── balady/                # Municipal licenses
│   └── YOUR_PDF.pdf
├── gosi/                  # Social insurance
│   └── YOUR_PDF.pdf
├── qiwa/                  # Labor contracts
│   └── YOUR_PDF.pdf
└── zatca/                 # Tax/Zakat
    └── YOUR_PDF.pdf
```

---

## 📝 Alternative: Convert PDF to Text (If Extraction Fails)

If the PDF extraction doesn't work well (scanned PDFs or complex layouts), convert it to text:

### Option 1: Copy-Paste (Quickest)
1. Open PDF in Adobe Reader or browser
2. Select all text (Ctrl+A)
3. Copy (Ctrl+C)
4. Paste into Notepad or VS Code
5. Save as `.txt` file

### Option 2: Online Converter
1. Go to https://pdftotext.com/ (or similar)
2. Upload your PDF
3. Download the .txt file
4. Place in `documents/01-government/monshaat/`

### Option 3: Python Script (For Many PDFs)
```python
# install: pip install PyPDF2
import PyPDF2

with open('Monshaat_Template.pdf', 'rb') as f:
    reader = PyPDF2.PdfReader(f)
    text = ""
    for page in reader.pages:
        text += page.extract_text() + "\n"
    
with open('Monshaat_Template.txt', 'w', encoding='utf-8') as f:
    f.write(text)
```

---

## 🔧 If PDF Extraction Doesn't Work

### Check if PDF is Image-Based
Some PDFs are just scanned images. The extractor can't read these.

**Signs of image-based PDF:**
- Can't select text with mouse
- File size is large (>10MB for few pages)
- Looks like a photocopy

**Solution:** Use OCR (Optical Character Recognition):
- Online: https://www.onlineocr.net/
- Adobe Acrobat Pro: Tools > Scan & OCR

### Common Issues

**Issue: "PDF extraction returned empty text"**
- PDF is image-based (see above)
- PDF is password-protected
- Solution: Convert to text manually

**Issue: "Arabic text is garbled"**
- PDF uses custom fonts
- Solution: Copy-paste method preserves Arabic better

**Issue: Build fails after adding pdf-extract**
```powershell
cd backend
cargo clean
cargo build
```

---

## 🧪 Test Your PDF

After adding the PDF and restarting backend, test it:

```powershell
# Test feasibility study that should cite your document
curl -X POST http://localhost:3001/api/rag-study `
  -H "Content-Type: application/json" `
  -d '{"business_name":"Test SME","description":"Starting a small business in Riyadh","target_city":"Riyadh","capital_budget":200000,"industry":"retail","business_model":"brick_and_mortar","initial_employees":4,"founder_experience":"beginner","contact_email":"test@example.com","include_competitor_analysis":false,"include_persona_debate":false}'
```

Look for citations from your PDF in the `sources_cited` field!

---

## 📊 PDF Processing Details

### What Happens to Your PDF:

1. **Text Extraction:** PDF → Plain text
2. **Cleaning:** Remove extra whitespace, fix encoding
3. **Chunking:** Split into 2000-character segments
4. **Embedding:** Create 3072-dimension vectors using OpenAI
5. **Storage:** Save to Qdrant vector database
6. **Indexing:** Ready for semantic search

### Supported PDF Types:
- ✅ Text-based PDFs (most common)
- ✅ Multi-page PDFs
- ✅ PDFs with tables (text extracted)
- ✅ Mixed Arabic/English PDFs
- ❌ Image-based PDFs (scanned documents)
- ❌ Password-protected PDFs

---

## 💡 Tips for Better Results

### 1. Rename Your PDF Descriptively
```
Good:  Monshaat_Feasibility_Template_2024.pdf
Bad:   doc123.pdf

Good:  SME_Registration_Guide.pdf
Bad:   document.pdf
```

### 2. Add Metadata Header (If Converting to Text)
If you convert to `.txt`, add metadata at the top:
```
TITLE: SME Feasibility Study Template
AUTHORITY: Monsha'at
DATE: 2024
URL: https://monshaat.gov.sa/templates

[Rest of your content...]
```

### 3. Organize by Authority
Put documents in the correct authority folder:
- Monsha'at docs → `monshaat/`
- Ministry of Commerce → `ministry_of_commerce/`
- Municipal → `balady/`

This helps the AI cite the correct authority.

---

## 🚀 Summary

**To add your Monsha'at PDF:**

1. Copy PDF to `documents/01-government/monshaat/`
2. Rebuild backend: `cd backend && cargo build`
3. Restart backend: `cargo run`
4. PDF will be automatically indexed
5. AI will now cite it in feasibility studies!

**Quick command:**
```powershell
copy "C:\Your\Path\Monshaat_Template.pdf" "C:\Users\ab5\CascadeProjects\saudi-market-feasibility\documents\01-government\monshaat\"
```

That's it! The RAG system will now use your Monsha'at template for generating feasibility studies with proper citations.
