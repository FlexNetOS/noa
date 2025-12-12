#!/bin/bash
# FlexNetOS Organization Deployment Script
# Deploys automation workflows to all repositories in the organization
# Usage: ./deploy-to-org.sh [--dry-run] [--repos "repo1,repo2"]

set -euo pipefail

# Configuration
ORG_NAME="${FLEXNETOS_ORG:-FlexNetOS}"
CONFIG_FILE=".github/org-config/flexnetos-automation.yml"
WORKFLOW_FILES=(
    ".github/workflows/flexnetos-auto-resolve.yml"
    ".github/workflow-templates/flexnetos-reusable-resolver.yml"
)
ACTION_DIR=".github/actions/ai-resolver"
SCRIPT_DIR=".github/scripts"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Parse arguments
DRY_RUN=false
SPECIFIC_REPOS=""
FORCE=false
SKIP_VALIDATION=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --repos)
            SPECIFIC_REPOS="$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --skip-validation)
            SKIP_VALIDATION=true
            shift
            ;;
        --help)
            echo "FlexNetOS Organization Deployment Script"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --dry-run          Show what would be done without making changes"
            echo "  --repos \"a,b,c\"    Deploy to specific repos only"
            echo "  --force            Force deployment even if validation fails"
            echo "  --skip-validation  Skip pre-deployment validation"
            echo "  --help             Show this help message"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check GitHub CLI
    if ! command -v gh &> /dev/null; then
        log_error "GitHub CLI (gh) is required but not installed"
        exit 1
    fi

    # Check authentication
    if ! gh auth status &> /dev/null; then
        log_error "Not authenticated with GitHub. Run 'gh auth login'"
        exit 1
    fi

    # Check organization access
    if ! gh api "orgs/$ORG_NAME" &> /dev/null; then
        log_error "Cannot access organization: $ORG_NAME"
        exit 1
    fi

    log_success "Prerequisites check passed"
}

# Validate configuration
validate_config() {
    if [ "$SKIP_VALIDATION" = true ]; then
        log_warning "Skipping validation"
        return 0
    fi

    log_info "Validating configuration..."

    # Check if config file exists
    if [ ! -f "$CONFIG_FILE" ]; then
        log_error "Configuration file not found: $CONFIG_FILE"
        exit 1
    fi

    # Check if workflow files exist
    for workflow in "${WORKFLOW_FILES[@]}"; do
        if [ ! -f "$workflow" ]; then
            log_error "Workflow file not found: $workflow"
            exit 1
        fi
    done

    # Validate YAML syntax
    if command -v yq &> /dev/null; then
        for workflow in "${WORKFLOW_FILES[@]}"; do
            if ! yq '.' "$workflow" > /dev/null 2>&1; then
                log_error "Invalid YAML in: $workflow"
                exit 1
            fi
        done
    fi

    log_success "Configuration validation passed"
}

# Get list of repositories to deploy to
get_target_repos() {
    log_info "Getting target repositories..."

    if [ -n "$SPECIFIC_REPOS" ]; then
        echo "$SPECIFIC_REPOS" | tr ',' '\n'
        return
    fi

    # Get all repos from organization
    gh repo list "$ORG_NAME" --json name,isArchived --limit 1000 | \
        jq -r '.[] | select(.isArchived == false) | .name'
}

# Check if repo should be excluded
should_exclude_repo() {
    local repo="$1"

    # Read exclusions from config
    local exclusions=$(yq '.repositories.exclude[]' "$CONFIG_FILE" 2>/dev/null || echo "")

    for exclusion in $exclusions; do
        # In [[ ]], RHS pattern matching requires unquoted variable
        # Unquoted $exclusion enables glob patterns like *-temp or test-*
        # Note: [[ ]] does NOT do filesystem pathname expansion on RHS
        if [[ "$repo" == $exclusion ]]; then
            return 0  # Should exclude
        fi
    done

    return 1  # Should not exclude
}

# Deploy to a single repository
deploy_to_repo() {
    local repo="$1"
    local full_repo="$ORG_NAME/$repo"

    log_info "Deploying to $full_repo..."

    if [ "$DRY_RUN" = true ]; then
        log_info "[DRY RUN] Would deploy to $full_repo"
        return 0
    fi

    # Create temporary directory
    local temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT

    # Clone the repository
    if ! gh repo clone "$full_repo" "$temp_dir" -- --depth 1 2>/dev/null; then
        log_warning "Failed to clone $full_repo, skipping..."
        return 1
    fi

    # Create directories
    mkdir -p "$temp_dir/.github/workflows"
    mkdir -p "$temp_dir/.github/actions/ai-resolver"
    mkdir -p "$temp_dir/.github/scripts"
    mkdir -p "$temp_dir/.github/state"

    # Copy workflow files
    for workflow in "${WORKFLOW_FILES[@]}"; do
        cp "$workflow" "$temp_dir/$workflow"
    done

    # Copy action
    cp -r "$ACTION_DIR"/* "$temp_dir/.github/actions/ai-resolver/"

    # Copy scripts
    cp "$SCRIPT_DIR"/*.py "$temp_dir/.github/scripts/" 2>/dev/null || true

    # Create .gitkeep for state directory
    touch "$temp_dir/.github/state/.gitkeep"

    # Commit and push
    cd "$temp_dir"
    git config user.name "FlexNetOS Bot"
    git config user.email "bot@flexnetos.dev"

    git add .github/

    if git diff --cached --quiet; then
        log_info "No changes needed for $full_repo"
        return 0
    fi

    git commit -m "🤖 Deploy FlexNetOS automation workflows

Automated deployment of:
- Auto-resolve CI workflow
- Reusable resolver workflow
- AI resolver action
- Web app auth scripts

This enables fully automated issue resolution and PR merging.
"

    if git push origin HEAD; then
        log_success "Deployed to $full_repo"
    else
        log_error "Failed to push to $full_repo"
        return 1
    fi
}

# Setup organization secrets
setup_org_secrets() {
    log_info "Setting up organization secrets..."

    if [ "$DRY_RUN" = true ]; then
        log_info "[DRY RUN] Would setup organization secrets"
        return 0
    fi

    # Check if FLEXNETOS_BOT_TOKEN exists
    if ! gh secret list --org "$ORG_NAME" 2>/dev/null | grep -q "FLEXNETOS_BOT_TOKEN"; then
        log_warning "FLEXNETOS_BOT_TOKEN not set. You need to set this manually:"
        log_warning "  gh secret set FLEXNETOS_BOT_TOKEN --org $ORG_NAME"
    fi

    log_success "Organization secrets check complete"
}

# Create organization-level workflow caller
create_org_workflow_caller() {
    log_info "Creating organization workflow caller..."

    if [ "$DRY_RUN" = true ]; then
        log_info "[DRY RUN] Would create organization workflow caller"
        return 0
    fi

    # The caller workflow that repos can use
    local caller_workflow=".github/workflows/caller-flexnetos-automation.yml"

    cat > "$caller_workflow" << 'EOF'
# FlexNetOS Automation Caller
# Include this in your repository to enable automated issue resolution

name: FlexNetOS Automation

on:
  pull_request:
    types: [opened, synchronize, reopened, review_requested]
  pull_request_review:
    types: [submitted]

jobs:
  auto-resolve:
    uses: FlexNetOS/.github/.github/workflows/flexnetos-reusable-resolver.yml@main
    with:
      pr_number: ${{ github.event.pull_request.number }}
      max_iterations: 10
      ai_provider: copilot
      auto_merge: true
      security_scan: true
      quality_gates: true
    secrets:
      BOT_TOKEN: ${{ secrets.FLEXNETOS_BOT_TOKEN }}
EOF

    log_success "Created caller workflow template"
}

# Generate deployment report
generate_report() {
    local total=$1
    local success=$2
    local failed=$3
    local skipped=$4

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "         FlexNetOS Deployment Report"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "  Organization: $ORG_NAME"
    echo "  Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
    echo ""
    echo "  📊 Results:"
    echo "     Total Repositories: $total"
    echo "     ✅ Successful: $success"
    echo "     ❌ Failed: $failed"
    echo "     ⏭️  Skipped: $skipped"
    echo ""

    if [ "$DRY_RUN" = true ]; then
        echo "  ⚠️  This was a DRY RUN - no changes were made"
        echo ""
    fi

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Main execution
main() {
    echo ""
    echo "🚀 FlexNetOS Organization Deployment"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    if [ "$DRY_RUN" = true ]; then
        log_warning "Running in DRY RUN mode"
    fi

    # Prerequisites
    check_prerequisites

    # Validate
    validate_config

    # Setup secrets
    setup_org_secrets

    # Create caller workflow
    create_org_workflow_caller

    # Get repos
    local repos=$(get_target_repos)
    local total=$(echo "$repos" | wc -l)
    local success=0
    local failed=0
    local skipped=0

    log_info "Found $total repositories"

    # Deploy to each repo
    while IFS= read -r repo; do
        if [ -z "$repo" ]; then
            continue
        fi

        if should_exclude_repo "$repo"; then
            log_info "Skipping excluded repo: $repo"
            ((skipped++))
            continue
        fi

        if deploy_to_repo "$repo"; then
            ((success++))
        else
            ((failed++))
        fi

        # Rate limiting
        sleep 1
    done <<< "$repos"

    # Report
    generate_report "$total" "$success" "$failed" "$skipped"

    if [ "$failed" -gt 0 ] && [ "$FORCE" != true ]; then
        exit 1
    fi
}

# Run
main

