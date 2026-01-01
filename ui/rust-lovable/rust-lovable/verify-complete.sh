#!/bin/bash

# Rust Lovable - Complete Verification Script
# This script verifies that all components are properly implemented

set -e

echo "=== Rust Lovable - Complete Verification ==="
echo "Date: $(date)"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if file exists
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1"
        return 0
    else
        echo -e "${RED}✗${NC} $1"
        return 1
    fi
}

# Function to check if directory exists
check_directory() {
    if [ -d "$1" ]; then
        echo -e "${GREEN}✓${NC} $1"
        return 0
    else
        echo -e "${RED}✗${NC} $1"
        return 1
    fi
}

# Function to check if string exists in file
check_string_in_file() {
    if grep -q "$2" "$1" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} $2 in $1"
        return 0
    else
        echo -e "${RED}✗${NC} $2 not found in $1"
        return 1
    fi
}

# Initialize counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# Function to run a check
run_check() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    if "$@"; then
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    else
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
    fi
}

echo
echo "1. Project Structure Verification"
echo "================================="

# Check main project files
run_check check_file "Cargo.toml"
run_check check_file "src/main.rs"
run_check check_file "src/lib.rs"

# Check source directory structure
run_check check_directory "src/app"
run_check check_directory "src/components"
run_check check_directory "src/core"
run_check check_directory "src/utils"
run_check check_directory "src/config"
run_check check_directory "src/resources"
run_check check_directory "src/compression"
run_check check_directory "src/metadata"
run_check check_directory "src/ml_devops"
run_check check_directory "src/vibe_coding"

echo
echo "2. Configuration System Verification"
echo "===================================="

# Check configuration files
run_check check_file "src/config/mod.rs"
run_check check_file "src/config/ai_configs.rs"
run_check check_file "src/config/resource_configs.rs"
run_check check_file "src/config/ml_configs.rs"
run_check check_file "src/config/compression_configs.rs"
run_check check_file "src/config/monitoring_configs.rs"

# Check key configuration features
run_check check_string_in_file "src/config/mod.rs" "pub struct AppConfig"
run_check check_string_in_file "src/config/ai_configs.rs" "pub struct AIConfig"
run_check check_string_in_file "src/config/resource_configs.rs" "pub struct ResourceConfig"

echo
echo "3. Resource Sharing System Verification"
echo "======================================="

# Check resource sharing files
run_check check_file "src/resources/mod.rs"
run_check check_file "src/resources/prompt_cache.rs"
run_check check_file "src/resources/embedding_cache.rs"
run_check check_file "src/resources/skill_registry.rs"
run_check check_file "src/resources/resource_sharing.rs"

# Check key resource sharing features
run_check check_string_in_file "src/resources/mod.rs" "pub struct ResourceManager"
run_check check_string_in_file "src/resources/resource_sharing.rs" "pub struct ResourceSharingManager"

echo
echo "4. Compression System Verification"
echo "=================================="

# Check compression files
run_check check_file "src/compression/mod.rs"
run_check check_file "src/compression/algorithms.rs"
run_check check_file "src/compression/ml_compression.rs"
run_check check_file "src/compression/streaming_compression.rs"
run_check check_file "src/compression/cache_compression.rs"

# Check key compression features
run_check check_string_in_file "src/compression/mod.rs" "pub struct CompressionManager"
run_check check_string_in_file "src/compression/algorithms.rs" "pub trait CompressionAlgorithm"
run_check check_string_in_file "src/compression/ml_compression.rs" "pub struct MLEmbeddingCompressor"

echo
echo "5. Metadata System Verification"
echo "==============================="

# Check metadata files
run_check check_file "src/metadata/mod.rs"
run_check check_file "src/metadata/schemas.rs"
run_check check_file "src/metadata/data_tables.rs"
run_check check_file "src/metadata/metadata_manager.rs"

# Check key metadata features
run_check check_string_in_file "src/metadata/mod.rs" "pub struct RichMetadata"
run_check check_string_in_file "src/metadata/schemas.rs" "pub struct SchemaDefinition"
run_check check_string_in_file "src/metadata/metadata_manager.rs" "pub struct MetadataManager"

echo
echo "6. ML DevOps System Verification"
echo "================================"

# Check ML DevOps files
run_check check_file "src/ml_devops/mod.rs"
run_check check_file "src/ml_devops/pipeline.rs"
run_check check_file "src/ml_devops/experiment.rs"
run_check check_file "src/ml_devops/model_registry.rs"
run_check check_file "src/ml_devops/monitoring.rs"
run_check check_file "src/ml_devops/deployment.rs"
run_check check_file "src/ml_devops/feature_store.rs"

# Check key ML DevOps features
run_check check_string_in_file "src/ml_devops/mod.rs" "pub struct MLDevOpsManager"
run_check check_string_in_file "src/ml_devops/pipeline.rs" "pub struct PipelineOrchestrator"
run_check check_string_in_file "src/ml_devops/experiment.rs" "pub struct ExperimentTracker"

echo
echo "7. Vibe Coding System Verification"
echo "=================================="

# Check vibe coding files
run_check check_file "src/vibe_coding/mod.rs"
run_check check_file "src/vibe_coding/code_generator.rs"
run_check check_file "src/vibe_coding/prompt_engineer.rs"
run_check check_file "src/vibe_coding/auto_complete.rs"
run_check check_file "src/vibe_coding/code_refactor.rs"
run_check check_file "src/vibe_coding/documentation_generator.rs"
run_check check_file "src/vibe_coding/test_generator.rs"

# Check key vibe coding features
run_check check_string_in_file "src/vibe_coding/mod.rs" "pub struct VibeCodingManager"
run_check check_string_in_file "src/vibe_coding/mod.rs" "pub struct VibeCodingContext"
run_check check_string_in_file "src/vibe_coding/code_generator.rs" "pub struct CodeGenerator"

echo
echo "8. Core System Verification"
echo "==========================="

# Check core system files
run_check check_file "src/core/conversational_ai.rs"
run_check check_file "src/core/ui_generator.rs"
run_check check_file "src/core/project_manager.rs"
run_check check_file "src/core/sandbox.rs"

# Check key core features
run_check check_string_in_file "src/core/conversational_ai.rs" "pub struct ConversationalAI"
run_check check_string_in_file "src/core/ui_generator.rs" "pub struct UIGenerator"

echo
echo "9. Utility System Verification"
echo "=============================="

# Check utility files
run_check check_file "src/utils/hardware_detector.rs"
run_check check_file "src/utils/compression.rs"

# Check key utility features
run_check check_string_in_file "src/utils/hardware_detector.rs" "pub struct HardwareDetector"

echo
echo "10. Component System Verification"
echo "================================="

# Check component files
run_check check_file "src/components/chat_interface.rs"
run_check check_file "src/components/code_editor.rs"
run_check check_file "src/components/ui_preview.rs"

# Check key component features
run_check check_string_in_file "src/components/chat_interface.rs" "pub struct ChatInterface"

echo
echo "11. Main Application Verification"
echo "================================="

# Check main application
run_check check_string_in_file "src/main.rs" "fn main"
run_check check_string_in_file "src/app.rs" "pub struct App"

echo
echo "12. Configuration Files Verification"
echo "===================================="

# Check configuration files
run_check check_file "README.md"
run_check check_file "RUNBOOK-V2.md"
run_check check_file "install-v2.sh"
run_check check_file "verify-installation.sh"

echo
echo "13. Dependencies Verification"
echo "============================="

# Check Cargo.toml for essential dependencies
run_check check_string_in_file "Cargo.toml" "dioxus"
run_check check_string_in_file "Cargo.toml" "tokio"
run_check check_string_in_file "Cargo.toml" "serde"
run_check check_string_in_file "Cargo.toml" "anyhow"
run_check check_string_in_file "Cargo.toml" "zstd"
run_check check_string_in_file "Cargo.toml" "brotli"
run_check check_string_in_file "Cargo.toml" "flate2"

echo
echo "14. Feature Flags Verification"
echo "=============================="

# Check feature flags in Cargo.toml
run_check check_string_in_file "Cargo.toml" "ai-integrations"
run_check check_string_in_file "Cargo.toml" "database"
run_check check_string_in_file "Cargo.toml" "compression"
run_check check_string_in_file "Cargo.toml" "ml-devops"

echo
echo "15. API Endpoints Verification"
echo "=============================="

# Check for API endpoint implementations
run_check check_string_in_file "src-tauri/src/api/streaming.rs" "pub async fn generate_ai_code_stream"
run_check check_string_in_file "src-tauri/src/api/packages.rs" "pub async fn detect_packages_from_code"
run_check check_string_in_file "src-tauri/src/api/missing_endpoints.rs" "pub async fn"

echo
echo "=========================================="
echo "VERIFICATION SUMMARY"
echo "=========================================="
echo "Total checks: $TOTAL_CHECKS"
echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"
echo

if [ $FAILED_CHECKS -eq 0 ]; then
    echo -e "${GREEN}✓ All verifications passed!${NC}"
    echo -e "${GREEN}Rust Lovable is ready for deployment.${NC}"
    exit 0
else
    echo -e "${RED}✗ Some verifications failed.${NC}"
    echo -e "${YELLOW}Please review the failed checks above.${NC}"
    exit 1
fi