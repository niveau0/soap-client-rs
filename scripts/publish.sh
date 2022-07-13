#!/usr/bin/env bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "soapus-runtime" ]; then
    error "Must be run from the workspace root directory"
    exit 1
fi

# Configuration
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
DRY_RUN="${DRY_RUN:-false}"

info "🚀 Publishing soapus v${VERSION} to crates.io"
echo

# Parse arguments
for arg in "$@"; do
    case $arg in
        --dry-run)
            DRY_RUN=true
            warning "DRY RUN MODE - No actual publishing will occur"
            ;;
        --help)
            echo "Usage: $0 [--dry-run] [--help]"
            echo
            echo "Options:"
            echo "  --dry-run    Run all checks but don't actually publish"
            echo "  --help       Show this help message"
            echo
            echo "Environment variables:"
            echo "  DRY_RUN=true    Same as --dry-run flag"
            exit 0
            ;;
        *)
            error "Unknown argument: $arg"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo
info "Step 1: Pre-flight checks"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check git status
if [ -n "$(git status --porcelain)" ]; then
    error "Working directory is not clean. Commit or stash changes first."
    exit 1
fi
success "Git working directory is clean"

# Check we're on main/master
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ] && [ "$BRANCH" != "master" ]; then
    warning "Not on main/master branch (current: $BRANCH)"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
else
    success "On branch: $BRANCH"
fi

# Check if version tag exists
if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
    error "Git tag v${VERSION} already exists"
    exit 1
fi
success "Version tag v${VERSION} does not exist yet"

# Check cargo login
if [ "$DRY_RUN" != "true" ]; then
    if ! cargo login --help >/dev/null 2>&1; then
        error "cargo login not available"
        exit 1
    fi
    info "Make sure you're logged in to crates.io (run: cargo login <token>)"
fi

echo
info "Step 2: Run tests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo test --all --all-features
success "All tests passed"

echo
info "Step 3: Run clippy"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo clippy --all --all-features -- -D warnings
success "Clippy checks passed"

echo
info "Step 4: Check formatting"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo fmt --all -- --check
success "Code is formatted correctly"

echo
info "Step 5: Build documentation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo doc --all --no-deps --all-features
success "Documentation built successfully"

echo
info "Step 6: Dry-run packaging"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Runtime (no internal dependencies)
info "Checking soapus-runtime..."
cargo publish --dry-run -p soapus-runtime
success "soapus-runtime package OK"

# Codegen (depends on runtime)
info "Checking soapus-codegen..."
cargo publish --dry-run -p soapus-codegen
success "soapus-codegen package OK"

# CLI (depends on codegen)
info "Checking soapus-cli..."
cargo publish --dry-run -p soapus-cli
success "soapus-cli package OK"

if [ "$DRY_RUN" = "true" ]; then
    echo
    success "✅ All dry-run checks passed!"
    info "To publish for real, run without --dry-run flag"
    exit 0
fi

echo
warning "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
warning "About to publish v${VERSION} to crates.io"
warning "This action CANNOT be undone!"
warning "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
read -p "Are you sure you want to continue? (yes/NO) " -r
echo
if [ "$REPLY" != "yes" ]; then
    info "Aborted by user"
    exit 0
fi

echo
info "Step 7: Publishing to crates.io"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Publish runtime
info "Publishing soapus-runtime v${VERSION}..."
cargo publish -p soapus-runtime
success "soapus-runtime published!"

# Wait for crates.io to index
info "Waiting 30 seconds for crates.io to index..."
sleep 30

# Publish codegen
info "Publishing soapus-codegen v${VERSION}..."
cargo publish -p soapus-codegen
success "soapus-codegen published!"

# Wait for crates.io to index
info "Waiting 30 seconds for crates.io to index..."
sleep 30

# Publish CLI
info "Publishing soapus-cli v${VERSION}..."
cargo publish -p soapus-cli
success "soapus-cli published!"

echo
info "Step 8: Creating git tag"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
git tag -a "v${VERSION}" -m "Release v${VERSION}"
success "Created tag v${VERSION}"

echo
info "Step 9: Pushing to GitHub"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
git push origin "v${VERSION}"
success "Pushed tag to GitHub"

echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
success "🎉 Successfully published v${VERSION} to crates.io!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
info "Next steps:"
echo "  1. Wait a few minutes for crates.io to index the crates"
echo "  2. Verify at https://crates.io/crates/soapus-runtime"
echo "  3. Verify at https://crates.io/crates/soapus-codegen"
echo "  4. Verify at https://crates.io/crates/soapus-cli"
echo "  5. Check docs.rs builds:"
echo "     - https://docs.rs/soapus-runtime/${VERSION}"
echo "     - https://docs.rs/soapus-codegen/${VERSION}"
echo "  6. Test installation: cargo install soapus-cli"
echo "  7. Create GitHub release at:"
echo "     https://github.com/niveau0/soapus/releases/new?tag=v${VERSION}"
echo
