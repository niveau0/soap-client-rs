# Publishing Scripts

This directory contains scripts and documentation for publishing the soapus crates to crates.io.

## 📋 Files

### `publish.sh`

Automated publish script that handles the entire release process:

- Runs all pre-flight checks (tests, clippy, formatting, docs)
- Performs dry-run packaging for all crates
- Publishes crates in the correct order (runtime → codegen → cli)
- Creates and pushes git tags
- Provides post-publish verification checklist

**Usage:**

```bash
# Dry run (recommended first)
./scripts/publish.sh --dry-run

# Actual publish (requires crates.io login)
./scripts/publish.sh
```

**Environment Variables:**
- `DRY_RUN=true` - Same as `--dry-run` flag

### `PRE_PUBLISH_CHECKLIST.md`

Comprehensive checklist to complete before publishing to crates.io. Covers:

- Code quality and testing
- Documentation completeness
- Metadata verification
- Security and legal compliance
- Post-publish verification steps

**Use this checklist** before running `publish.sh` to ensure nothing is missed.

## 🚀 Publishing Process

### 1. Preparation

Complete all items in `PRE_PUBLISH_CHECKLIST.md`:

```bash
# Run all quality checks
cargo test --all --all-features
cargo clippy --all --all-features -- -D warnings
cargo fmt --all -- --check
cargo doc --all --no-deps --all-features
```

### 2. Dry Run

Test the publish process without actually publishing:

```bash
./scripts/publish.sh --dry-run
```

This will:
- Run all tests and checks
- Verify package contents
- Ensure dependencies are correct
- Check that version tags don't exist

### 3. Review

- Ensure git working directory is clean
- Verify CHANGELOG.md has correct release date
- Double-check version numbers match
- Review package contents with `cargo package --list -p <crate-name>`

### 4. Login to crates.io

```bash
cargo login <your-api-token>
```

Get your token from: https://crates.io/settings/tokens

### 5. Publish

Run the actual publish:

```bash
./scripts/publish.sh
```

The script will prompt for confirmation before publishing.

### 6. Verify

After publishing, verify:

1. Crates appear on crates.io:
   - https://crates.io/crates/soapus-runtime
   - https://crates.io/crates/soapus-codegen
   - https://crates.io/crates/soapus-cli

2. Documentation builds on docs.rs:
   - https://docs.rs/soapus-runtime
   - https://docs.rs/soapus-codegen

3. Installation works:
   ```bash
   cargo install soapus-cli
   soapus-cli --version
   ```

4. Create GitHub release with CHANGELOG notes:
   - https://github.com/niveau0/soapus/releases

## 📦 Crate Publishing Order

The crates **must** be published in this order due to dependencies:

1. **soapus-runtime** (no internal dependencies)
2. **soapus-codegen** (depends on runtime)
3. **soapus-cli** (depends on codegen)

The `publish.sh` script handles this automatically with appropriate delays for crates.io indexing.

## 🔧 Troubleshooting

### "Crate already published"

Check if the version already exists on crates.io. You cannot republish the same version.

### "Version mismatch"

Ensure all internal dependencies specify the same version:

```toml
# In soapus-cli/Cargo.toml
soapus-codegen = { version = "0.1", path = "../soapus-codegen" }
```

### "Package too large"

Check the `include` list in `Cargo.toml` to ensure you're not including unnecessary files.

Use `cargo package --list -p <crate-name>` to see what will be included.

### "Git working directory not clean"

Commit or stash all changes before publishing:

```bash
git status
git add .
git commit -m "Prepare for v0.1.0 release"
```

### "Failed to index on crates.io"

Wait a few minutes between publishing crates. The script includes 30-second delays, but sometimes it takes longer.

## 📚 Additional Resources

- [The Cargo Book - Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [API Guidelines - Versioning](https://rust-lang.github.io/api-guidelines/versioning.html)
- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)

## 🤝 Support

If you encounter issues during the publish process:

1. Check the `PRE_PUBLISH_CHECKLIST.md` for common issues
2. Review the error message carefully
3. Check the [Cargo documentation](https://doc.rust-lang.org/cargo/)
4. Open an issue if you believe it's a bug in the scripts

---

**Last Updated:** Before v0.1.0 release  
**Maintained by:** soapus contributors