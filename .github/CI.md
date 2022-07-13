# Continuous Integration (CI)

This document describes the CI/CD setup for the soapus project.

---

## 🚀 GitHub Actions Workflow

The project uses GitHub Actions for continuous integration. The workflow is triggered on:
- **Push to `main` branch**
- **Pull requests to `main` branch**

### Workflow File

`.github/workflows/ci.yml`

---

## 🧪 CI Jobs

### 1. Test Suite (`test`)

Runs all tests across the workspace.

```bash
cargo test --all --verbose
```

**What it tests:**
- Unit tests in `soapus-codegen`
- Unit tests in `soapus-runtime`
- Integration tests
- Doc tests

**Current test count:** ~57 tests

---

### 2. Code Formatting (`fmt`)

Ensures all code follows Rust formatting standards.

```bash
cargo fmt --all -- --check
```

**What it checks:**
- Consistent code formatting
- Uses default `rustfmt` configuration

**Action on failure:** Fails the CI if formatting is inconsistent

**Fix locally:**
```bash
cargo fmt --all
```

---

### 3. Clippy Lints (`clippy`)

Runs Rust linter to catch common mistakes and enforce best practices.

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**What it checks:**
- Code quality issues
- Common mistakes
- Performance issues
- Idiomatic Rust patterns

**Configuration:** Treats all warnings as errors (`-D warnings`)

**Fix locally:**
```bash
cargo clippy --all-targets --all-features --fix
```

---

### 4. Build Examples (`build-examples`)

Verifies that all examples compile successfully.

```bash
cargo build -p calculator --verbose
cargo build -p observability --verbose
cargo build -p soapus-cli --verbose
```

**What it builds:**
- Calculator example (with code generation via `build.rs`)
- Observability example (with all features)
- CLI tool (`soapus-cli`)

**Purpose:**
- Ensures examples stay up-to-date with API changes
- Verifies code generation works correctly
- Checks that generated code compiles

---

### 5. Verify Generated Code (`verify-generated-code`)

Specifically tests that WSDL → Rust code generation produces valid code.

**Steps:**
1. Build calculator example (triggers `build.rs`)
2. Verify generated code exists in `target/debug/build/*/out/soap_client.rs`
3. Verify generated code compiles without errors

**Why separate job:**
- Ensures code generation doesn't silently fail
- Catches issues in generated code early
- Validates that `build.rs` integration works

---

## 📦 Caching Strategy

All jobs use `Swatinem/rust-cache@v2` for intelligent Rust-specific caching:

```yaml
- name: Setup Rust cache
  uses: Swatinem/rust-cache@v2
```

**What it caches:**
- Rust toolchain components
- Cargo registry (`~/.cargo/registry`)
- Cargo index (`~/.cargo/git`)
- Build artifacts (`target/`)
- Automatically determines optimal cache keys

**Benefits:**
- 5-10x faster CI runs compared to no caching
- Intelligent cache invalidation (only rebuilds what changed)
- Automatic cleanup of old cache entries
- Optimized specifically for Rust projects
- Much simpler than manual caching configuration

**Why `Swatinem/rust-cache` instead of manual caching:**
- Industry standard for Rust CI (used by most Rust projects)
- Handles edge cases (incremental compilation, proc-macros, etc.)
- Automatic optimization without manual configuration
- Shared cache across jobs

### Why Rust Toolchain Must Be Installed

**Question:** Can we use a base image with Rust pre-installed?

**Answer:** No, GitHub Actions hosted runners don't support custom base images with Rust pre-installed.

**Alternatives considered:**

1. **Custom Docker container** 
   - ❌ GitHub Actions runners don't persist custom images between runs
   - ❌ Would need to pull image every time (slower than toolchain install)

2. **Self-hosted runner with Rust pre-installed**
   - ❌ Requires infrastructure maintenance
   - ❌ Not suitable for open-source projects
   - ❌ Security concerns

3. **GitHub's default images**
   - ❌ Don't include Rust by default
   - ❌ No official Rust variant available

**Current approach is optimal:**
- `dtolnay/rust-toolchain@stable` installs Rust in ~10-15 seconds
- `Swatinem/rust-cache` caches the toolchain itself
- Subsequent runs use cached toolchain (near-instant)
- Total overhead: ~10s first run, ~2s cached runs

**Conclusion:** The current setup is the fastest possible approach for GitHub Actions hosted runners.

---

## 🎯 Examples Testing Strategy

### Why Examples Are Build-Only

Examples are **compiled but not executed** in CI because:

1. **External Dependencies**
   - Calculator example calls real SOAP service (http://www.dneonline.com)
   - Observability example requires Docker (Jaeger, Prometheus, Grafana)
   - Network calls are unreliable in CI

2. **Focus on Code Quality**
   - CI ensures code **compiles** ✅
   - CI ensures code **generation works** ✅
   - Runtime behavior tested **manually** or **locally**

3. **Fast Feedback**
   - Build-only tests run in ~1-2 minutes
   - Full integration tests would take 5-10+ minutes
   - Developers get faster feedback on PRs

### Example-Specific Handling

| Example | CI Strategy | Runtime Testing |
|---------|-------------|-----------------|
| **calculator** | ✅ Build with code generation | Manual/local only |
| **observability** | ✅ Build (no Docker) | Manual with `docker-compose up` |
| **soapus-cli** | ✅ Build CLI binary | Manual testing with real WSDLs |

### Local Testing

To test examples locally:

```bash
# Calculator example
cd examples/calculator
cargo run

# Observability example (requires Docker)
cd examples/observability
docker-compose up -d
cargo run
curl http://localhost:3000/calculate?a=5&b=3&op=add

# CLI tool
cargo run -p soapus-cli -- info testdata/wsdl/calculator.wsdl
```

---

## ✅ Pre-commit Checklist

Before pushing code, run locally:

```bash
# 1. Format code
cargo fmt --all

# 2. Run tests
cargo test --all

# 3. Run clippy
cargo clippy --all-targets --all-features

# 4. Build examples
cargo build -p calculator
cargo build -p observability
cargo build -p soapus-cli
```

Or use this one-liner:

```bash
cargo fmt --all && cargo test --all && cargo clippy --all-targets --all-features && cargo build -p calculator -p observability -p soapus-cli
```

---

## 🔧 Troubleshooting CI Failures

### Test Failures

```
error: test failed, to rerun pass `--lib --test <test_name>`
```

**Solution:**
1. Run tests locally: `cargo test --all`
2. Fix failing tests
3. Commit and push

### Format Check Failures

```
Diff in <file> at line <N>
```

**Solution:**
```bash
cargo fmt --all
git add .
git commit -m "Fix formatting"
git push
```

### Clippy Failures

```
error: <clippy warning>
```

**Solution:**
```bash
cargo clippy --all-targets --all-features --fix
# Review changes
git add .
git commit -m "Fix clippy warnings"
git push
```

### Build Failures in Examples

```
error: could not compile `calculator`
```

**Possible causes:**
- API changes broke examples
- Missing dependencies
- Generated code has issues

**Solution:**
1. Build example locally: `cargo build -p calculator`
2. Check generated code: `cat target/debug/build/calculator-*/out/soap_client.rs`
3. Fix code generation or example usage
4. Commit and push

---

## 📊 CI Status Badge

Add to README.md:

```markdown
[![CI](https://github.com/niveau0/soapus/actions/workflows/ci.yml/badge.svg)](https://github.com/niveau0/soapus/actions/workflows/ci.yml)
```

---

## 🚀 Future Improvements

### Planned (Priority 2)
- [ ] Code coverage reporting (codecov.io or coveralls.io)
- [ ] Automated releases on tags
- [ ] Benchmark regression testing
- [ ] Security audit (cargo-audit)
- [ ] Dependency update checks (Dependabot)

### Considered (Low Priority)
- [ ] Cross-platform testing (Windows, macOS, Linux)
- [ ] Multiple Rust versions (stable, beta, nightly)
- [ ] Integration tests with mock SOAP server
- [ ] Generated code formatting check

---

## 📝 Maintenance

### Updating Dependencies

When updating dependencies that might break CI:

1. Update `Cargo.toml`
2. Run `cargo update`
3. Run full CI suite locally
4. Push and monitor CI

### Updating Rust Version

The workflow uses `dtolnay/rust-toolchain@stable`, which automatically uses the latest stable Rust.

To pin a specific version:
```yaml
- uses: dtolnay/rust-toolchain@1.70.0
```

---

**Last Updated:** 2024-12-17  
**Status:** ✅ CI fully configured and operational