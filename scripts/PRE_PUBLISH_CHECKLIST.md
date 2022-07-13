# Pre-Publish Checklist for crates.io

Complete this **manual** checklist before running `scripts/publish.sh`.

> **Note**: Automated checks (tests, clippy, formatting, doc build) are handled by CI and `publish.sh`. This checklist focuses on what requires human review.

---

## ✅ Prerequisites

- [ ] **CI is passing** on GitHub Actions
- [ ] **Git working directory is clean** (no uncommitted changes)
- [ ] **On `main` branch**
- [ ] **Latest changes pulled** from remote
- [ ] **Version tag does NOT exist** (`git tag -l | grep v0.1.0`)
- [ ] **Logged in to crates.io** (`cargo login <token>`)

---

## 📝 Documentation Review

- [ ] **README.md** - Features and examples are current
- [ ] **CHANGELOG.md** - Release notes complete with today's date
- [ ] **Version number** updated in `Cargo.toml` (workspace)
- [ ] **Crate READMEs** - Quick check for broken links:
  - `soapus-codegen/README.md`
  - `soapus-runtime/README.md`
  - `soapus-cli/README.md`
- [ ] **Examples** - Code comments and READMEs accurate

---

## 🔐 Security & Compliance

- [ ] **No secrets** - No hardcoded API keys, tokens, or passwords
- [ ] **No sensitive data** - No personal info in code or tests
- [ ] **License reviewed** - Dependencies are MIT-compatible

---

## 🔍 Semantic Review

- [ ] **README links** point to correct URLs (not placeholders or localhost)
- [ ] **Examples** reference correct crate versions (not git deps after publish)
- [ ] **No TODOs/FIXMEs** in public API rustdoc
- [ ] **CHANGELOG.md** has release date (replace `2024-12-XX` with actual date)

---

## 🚀 Pre-Publish Verification

- [ ] **Dry run passed**:
  ```bash
  ./scripts/publish.sh --dry-run
  ```
- [ ] **Review dry-run output** - No unexpected warnings or files

---

## 📦 Publish

- [ ] **Run publish script**:
  ```bash
  ./scripts/publish.sh
  ```
- [ ] **Confirm at prompts** (type "yes" when asked)

---

## 🎬 Post-Publish Verification

After `publish.sh` completes:

- [ ] **Crates appear on crates.io** (wait ~2 minutes):
  - https://crates.io/crates/soapus-runtime
  - https://crates.io/crates/soapus-codegen
  - https://crates.io/crates/soapus-cli

- [ ] **docs.rs builds** (wait ~5-10 minutes):
  - https://docs.rs/soapus-runtime
  - https://docs.rs/soapus-codegen

- [ ] **CLI installation works**:
  ```bash
  cargo install soapus-cli
  soapus-cli --version
  ```

- [ ] **Create GitHub release**:
  - Go to: https://github.com/niveau0/soapus/releases/new
  - Tag: `v0.1.0`
  - Title: `v0.1.0 - Initial Release`
  - Description: Copy from CHANGELOG.md

- [ ] **Update README badges** (if version-specific)

---

## 🎯 Optional Announcements

- [ ] Reddit r/rust
- [ ] Twitter/X
- [ ] Discord servers
- [ ] This Week in Rust submission

---

## 📊 Version-Specific Notes

### v0.1.0 (Initial Release)

- [ ] AI-generated notice is in main README
- [ ] Production readiness disclaimer included
- [ ] Known limitations documented in README

---

**Last Updated:** Before v0.1.0 release  
**Next Review:** Before v0.2.0 release
