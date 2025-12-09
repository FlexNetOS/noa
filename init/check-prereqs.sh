#!/usr/bin/env bash
#
# NOA Comprehensive Prerequisites Check (containment-first)
# - Prefers contained installs under NOA_ROOT/bin
# - Optional system-wide fallback with --allow-global
# - Install hints point to scripts/setup/install-all-tools.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
NOA_ROOT="${NOA_ROOT:-$REPO_ROOT}"
NOA_BIN="$NOA_ROOT/bin"

JSON_OUTPUT=false
ALLOW_GLOBAL=false
for arg in "$@"; do
  case "$arg" in
    --json) JSON_OUTPUT=true ;;
    --allow-global) ALLOW_GLOBAL=true ;;
  esac
done

mkdir -p "$NOA_BIN"

# Results
declare -a INSTALLED=()
declare -a MISSING_CRITICAL=()
declare -a MISSING_HIGH=()

version_gte() {
  printf '%s\n%s' "$2" "$1" | sort -V -C
}

check_tool() {
  local name="$1" min_version="$2" severity="$3" install_cmd="$4" version_cmd="$5" category="$6" bin_names="$7"

  local found=""
  IFS=":" read -ra bins <<< "$bin_names"
  for b in "${bins[@]}"; do
    if [[ -x "$NOA_BIN/$b" ]]; then
      found="$NOA_BIN/$b"
      break
    fi
  done

  if [[ -z "$found" && "$ALLOW_GLOBAL" == "true" ]]; then
    if command -v "$(echo "$version_cmd" | awk '{print $1}')" &>/dev/null; then
      found="$(command -v "$(echo "$version_cmd" | awk '{print $1}')" )"
    fi
  fi

  if [[ -n "$found" ]]; then
    local current_version
    current_version=$(eval "$version_cmd" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -1 || echo "unknown")
    if [[ "$current_version" != "unknown" ]] && version_gte "$current_version" "$min_version"; then
      INSTALLED+=("$name:$current_version:$category")
      $JSON_OUTPUT || echo -e "  [OK] $name $current_version ($found)"
    else
      $JSON_OUTPUT || echo -e "  [!!] $name $current_version (need >= $min_version)"
    fi
    return
  fi

  if [[ "$severity" == "CRITICAL" ]]; then
    MISSING_CRITICAL+=("$name:$install_cmd")
    $JSON_OUTPUT || { echo -e "  [X] $name NOT FOUND (CRITICAL)"; echo -e "      Install: $install_cmd"; }
  else
    MISSING_HIGH+=("$name:$install_cmd")
    $JSON_OUTPUT || { echo -e "  [X] $name NOT FOUND (HIGH)"; echo -e "      Install: $install_cmd"; }
  fi
}

check_self_contained() {
  local name="$1" exe_name="$2"
  local tool_path="$NOA_BIN/$exe_name"

  if [[ -f "$tool_path" ]]; then
    INSTALLED+=("$name:self-contained:Self-Contained")
    $JSON_OUTPUT || echo -e "  [OK] $name (self-contained: $tool_path)"
  else
    MISSING_HIGH+=("$name:./scripts/setup/install-all-tools.sh $name")
    $JSON_OUTPUT || echo -e "  [--] $name not in bin/ (optional)"
  fi
}

if ! $JSON_OUTPUT; then
  echo ""
  echo "============================================================"
  echo "NOA Prerequisites Check"
  echo "Constitution: §3.1 (Self-Contained), FR-015 (Security)"
  echo "============================================================"
  echo ""
  echo "NOA_ROOT: $NOA_ROOT"
  echo "NOA_BIN:  $NOA_BIN"
  echo "Mode: Contained-first (AllowGlobal=$ALLOW_GLOBAL)"
  echo "------------------------------------------------------------"
fi

# Build toolchains (critical)
check_tool "Rust (rustc)" "1.83.0" "CRITICAL" "./scripts/setup/install-all-tools.sh rust" "rustc --version" "Build" "rustc:rustc.exe"
check_tool "Cargo" "1.83.0" "CRITICAL" "./scripts/setup/install-all-tools.sh rust" "cargo --version" "Build" "cargo:cargo.exe"
check_tool "Go" "1.23.0" "CRITICAL" "./scripts/setup/install-all-tools.sh go" "go version" "Build" "go:go.exe"
check_tool "Node.js" "20.0.0" "CRITICAL" "./scripts/setup/install-all-tools.sh node" "node --version" "Build" "node:node.exe"
check_tool "Python" "3.12.0" "CRITICAL" "./scripts/setup/install-all-tools.sh python" "python3 --version" "Build" "python:python.exe"
check_tool "protoc" "28.0.0" "CRITICAL" "./scripts/setup/install-all-tools.sh protoc" "protoc --version" "Build" "protoc:protoc.exe"

! $JSON_OUTPUT && { echo ""; echo "2. Code Quality Tools (HIGH)"; echo "------------------------------------------------------------"; }
check_tool "rustfmt" "1.0.0" "HIGH" "./scripts/setup/install-all-tools.sh rust" "rustfmt --version" "Quality" "rustfmt:rustfmt.exe"
check_tool "clippy" "0.1.0" "HIGH" "./scripts/setup/install-all-tools.sh rust" "cargo clippy --version" "Quality" "cargo-clippy:cargo-clippy.exe"
check_tool "golangci-lint" "1.62.0" "HIGH" "./scripts/setup/install-all-tools.sh golangci-lint" "golangci-lint --version" "Quality" "golangci-lint:golangci-lint.exe"
check_tool "eslint" "9.0.0" "HIGH" "./scripts/setup/install-all-tools.sh eslint" "eslint --version" "Quality" "eslint:eslint.cmd"
check_tool "ruff" "0.8.0" "HIGH" "./scripts/setup/install-all-tools.sh ruff" "ruff --version" "Quality" "ruff:ruff.exe"

! $JSON_OUTPUT && { echo ""; echo "3. Security Tools (HIGH)"; echo "------------------------------------------------------------"; }
check_tool "Gitleaks" "8.21.0" "HIGH" "./scripts/setup/install-all-tools.sh gitleaks" "gitleaks version" "Security" "gitleaks:gitleaks.exe"
check_tool "Trivy" "0.57.0" "HIGH" "./scripts/setup/install-all-tools.sh trivy" "trivy --version" "Security" "trivy:trivy.exe"
check_tool "Grype" "0.84.0" "HIGH" "./scripts/setup/install-all-tools.sh grype" "grype version" "Security" "grype:grype.exe"
check_tool "Semgrep" "1.97.0" "HIGH" "./scripts/setup/install-all-tools.sh semgrep" "semgrep --version" "Security" "semgrep:semgrep.exe"

! $JSON_OUTPUT && { echo ""; echo "4. Self-Contained Utilities (noa_root/bin)"; echo "------------------------------------------------------------"; }
check_self_contained "jq" "jq"
check_self_contained "ripgrep" "rg"
check_self_contained "fd" "fd"
check_self_contained "bat" "bat"

! $JSON_OUTPUT && { echo ""; echo "5. Basic Prerequisites"; echo "------------------------------------------------------------"; }
check_tool "Git" "2.40.0" "CRITICAL" "./scripts/setup/install-all-tools.sh git" "git --version" "Basic" "git:git.exe"
check_tool "GitHub CLI" "2.40.0" "HIGH" "./scripts/setup/install-all-tools.sh gh" "gh --version" "Basic" "gh:gh.exe"

if $JSON_OUTPUT; then
  echo "{"
  echo "  \"noa_root\": \"$NOA_ROOT\","
  echo "  \"installed\": ${#INSTALLED[@]},"
  echo "  \"missing_critical\": ${#MISSING_CRITICAL[@]},"
  echo "  \"missing_high\": ${#MISSING_HIGH[@]}"
  echo "}"
else
  echo ""
  echo "============================================================"
  echo "Summary"
  echo "============================================================"
  echo "Installed:        ${#INSTALLED[@]}"
  echo "Missing CRITICAL: ${#MISSING_CRITICAL[@]}"
  echo "Missing HIGH:     ${#MISSING_HIGH[@]}"
fi

if [[ ${#MISSING_CRITICAL[@]} -gt 0 ]]; then
  exit 1
elif [[ ${#MISSING_HIGH[@]} -gt 0 ]]; then
  exit 2
else
  exit 0
fi

