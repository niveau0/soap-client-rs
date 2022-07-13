# Documentation Structure

This document describes the documentation structure of the soapus project and explains where to maintain different types of information to avoid duplication.

## 📚 Documentation Hierarchy

### 1. **Primary Documentation** (Single Source of Truth)

These files contain the complete, authoritative documentation:

#### `README.md` (Root)
- **Purpose**: Complete project overview and primary documentation
- **Content**:
  - Full feature descriptions with examples
  - Installation instructions
  - Complete Quick Start guide
  - Generated code examples
  - Advanced usage patterns
  - Working examples description
  - Architecture diagrams
- **Audience**: New users, GitHub visitors, comprehensive reference
- **Maintenance**: Update this FIRST when features change

#### `QUICKSTART.md`
- **Purpose**: 5-minute tutorial for getting started
- **Content**:
  - Step-by-step setup (1, 2, 3...)
  - Minimal working example
  - "What's Next?" links
- **Audience**: Impatient users who want to start immediately
- **Maintenance**: Keep synchronized with README Quick Start section

#### `docs/TRACING.md`
- **Purpose**: Deep-dive into observability features
- **Content**:
  - Tracing setup and configuration
  - Log level examples
  - OpenTelemetry integration
  - Metrics configuration
  - Troubleshooting
- **Audience**: Users implementing observability
- **Maintenance**: Update when tracing/metrics features change

#### `docs/API_DOCUMENTATION.md`
- **Purpose**: Runtime API reference and usage patterns
- **Content**:
  - SoapClient API details
  - Error handling patterns
  - Configuration options
  - Best practices
- **Audience**: Users writing production code
- **Maintenance**: Update when runtime API changes

### 2. **Crate-Specific READMEs** (Minimal, Link to Primary)

These files are deliberately minimal to avoid duplication:

#### `soapus-codegen/README.md`
- **Purpose**: crates.io landing page
- **Content**:
  - One-sentence description
  - Installation snippet (2 lines)
  - One minimal `build.rs` example
  - Feature list (bullets only, no explanations)
  - Links to main repository
- **Length**: ~75 lines
- **Rule**: NO duplicate examples from main README

#### `soapus-runtime/README.md`
- **Purpose**: crates.io landing page
- **Content**:
  - One-sentence description
  - Installation snippet
  - One minimal usage example
  - Feature flags list (bullets only)
  - Links to main repository
- **Length**: ~90 lines
- **Rule**: NO duplicate examples from main README

#### `soapus-cli/README.md`
- **Purpose**: crates.io landing page
- **Content**:
  - Installation instructions
  - Command summary (one line each)
  - Use cases (bullets only)
  - Links to main repository
- **Length**: ~60 lines
- **Rule**: NO verbose command examples

### 3. **Example Documentation** (Live Code)

#### `examples/calculator/README.md`
- **Purpose**: Explain the calculator example
- **Content**: Setup and running instructions
- **Rule**: Code is the documentation - keep README minimal

#### `examples/observability/README.md`
- **Purpose**: Explain the full observability stack
- **Content**: Docker setup, endpoints, metrics
- **Maintenance**: Update when stack components change

### 4. **Process Documentation**

#### `TODO.md`
- **Purpose**: Project roadmap and task tracking
- **Content**: Prioritized tasks, phases, status
- **Audience**: Contributors, maintainers

#### `CHANGELOG.md`
- **Purpose**: Release history
- **Content**: What changed in each version
- **Audience**: Users upgrading versions

#### `scripts/PRE_PUBLISH_CHECKLIST.md`
- **Purpose**: Publishing process checklist
- **Content**: Steps before releasing to crates.io
- **Audience**: Maintainers

### 5. **AI Assistant Context**

#### `.github/copilot-instructions.md`
- **Purpose**: Coding style guidelines for AI
- **Content**: Naming conventions, patterns, what to avoid

#### `.github/copilot-chat-context.md`
- **Purpose**: Project context for AI chat sessions
- **Content**: Architecture decisions, recent changes, known issues

## 🔄 Update Workflow

### When Adding a New Feature

1. **Update `README.md`** - Add feature description with example
2. **Update `CHANGELOG.md`** - Add to [Unreleased] section
3. **Update crate README** - Add feature to bullet list (NO example)
4. **Update API docs** - If public API changed
5. **Add/update example** - If feature needs demonstration

### When Changing Existing API

1. **Update `README.md`** - Fix examples
2. **Update `QUICKSTART.md`** - If affects quick start
3. **Update `docs/API_DOCUMENTATION.md`** - Document changes
4. **Update crate `lib.rs`** - Fix rustdoc examples
5. **Check examples/** - Ensure they still compile

### When Publishing a Release

1. **Update `CHANGELOG.md`** - Move [Unreleased] to new version
2. **Update version in `Cargo.toml`**
3. **Run `scripts/publish.sh --dry-run`**
4. **Review all crate READMEs** - Verify links work
5. **Publish with `scripts/publish.sh`**

## 🚫 Anti-Patterns (Avoid These)

### ❌ Don't Duplicate Code Examples
- **Wrong**: Same `build.rs` example in README.md AND codegen/README.md
- **Right**: Full example in README.md, minimal snippet in codegen/README.md + link

### ❌ Don't Duplicate Feature Descriptions
- **Wrong**: Explaining "What is SOAP 1.2" in every README
- **Right**: Explain once in main README, bullet point in crate README

### ❌ Don't Duplicate Installation Instructions
- **Wrong**: Complete installation guide in every README
- **Right**: Full guide in main README, one-liner in crate README

### ❌ Don't Put Examples in Multiple Places
- **Wrong**: Calculator example code in README AND in docs/
- **Right**: Code in `examples/calculator/`, reference in README

## ✅ Best Practices

### ✓ Use Links Instead of Duplication
```markdown
For complete examples, see the [main repository](https://github.com/niveau0/soapus).
```

### ✓ Keep Crate READMEs Minimal
- Installation: 2-3 lines
- Example: ONE minimal snippet
- Features: Bullet list only
- Everything else: Links

### ✓ One Source of Truth
- API documentation → `lib.rs` rustdoc
- Usage examples → `README.md` or `examples/`
- Configuration → `docs/API_DOCUMENTATION.md`

### ✓ Progressive Disclosure
- crates.io README → "What is this? How do I install?"
- Main README → "How do I use this? What can it do?"
- docs/ → "How does this work in detail?"
- examples/ → "Show me working code"

## 📊 Documentation Matrix

| Topic | Main README | Crate README | docs/ | examples/ |
|-------|-------------|--------------|-------|-----------|
| Installation | Full | Minimal | - | - |
| Quick Start | Full | One snippet | - | ✓ |
| Features | Full | List only | - | - |
| API Reference | Overview | - | Full | ✓ |
| Configuration | Examples | - | Full | ✓ |
| Tracing | Overview | Feature flag | Full | ✓ |
| Troubleshooting | Common | - | Full | - |

## 🔍 Finding Documentation

**"How do I install?"** → Main README Installation section

**"How do I get started?"** → QUICKSTART.md

**"What features exist?"** → Main README Features section

**"How do I use feature X?"** → Main README or docs/API_DOCUMENTATION.md

**"How do I enable tracing?"** → docs/TRACING.md

**"Show me real code"** → examples/calculator/ or examples/observability/

**"Which crate do I need?"** → Main README Project Structure

**"What's new?"** → CHANGELOG.md

**"What's planned?"** → TODO.md

## 🎯 Summary

**Golden Rule**: When in doubt, add to main README and link from crate READMEs.

**Why This Structure?**
- ✅ Single source of truth reduces maintenance burden
- ✅ Crate READMEs stay focused and clean
- ✅ Users get complete info in main repository
- ✅ No risk of contradictory documentation
- ✅ Easy to find information

**When to Break the Rules?**
- Never. If you think you need to duplicate, you probably need a link instead.