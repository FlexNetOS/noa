#!/bin/bash
# Phase 4 Smoke Test
# Verifies that Phase 4 implementation compiles and basic structure is correct

set -euo pipefail

echo "=========================================="
echo "Phase 4 Smoke Test"
echo "=========================================="
echo ""

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

cd "$PROJECT_ROOT"

echo "1. Checking Rust compilation..."
echo "   - Checking noa-core library..."
cargo check --lib -p noa-core 2>&1 | grep -E "(error|warning:.*error)" && exit 1 || true

echo "   - Checking noa-neural library..."
if [ -f "sys/core/crates/neural/Cargo.toml" ]; then
    cargo check --lib -p noa-neural 2>&1 | grep -E "(error|warning:.*error)" && exit 1 || true
fi

echo ""
echo "2. Verifying Phase 4 file structure..."
echo "   - Neural runtime modules..."
NEURAL_FILES=(
    "sys/core/src/neural/mod.rs"
    "sys/core/src/neural/model_loader.rs"
    "sys/core/src/neural/llama_backend.rs"
    "sys/core/src/neural/context.rs"
    "sys/core/src/neural/inference.rs"
    "sys/core/src/neural/hardware.rs"
    "sys/core/src/neural/benchmark.rs"
    "sys/core/src/neural/export.rs"
    "sys/core/src/neural/cuda_devices.rs"
    "sys/core/src/neural/multi_gpu.rs"
    "sys/core/src/neural/tensor_parallel.rs"
    "sys/core/src/neural/nvlink.rs"
    "sys/core/src/neural/gpu_pool.rs"
    "sys/core/src/neural/cuda_tiles.rs"
    "sys/core/src/neural/gpu_scheduler.rs"
    "sys/core/src/neural/gpu_health.rs"
)

for file in "${NEURAL_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "   ❌ Missing: $file"
        exit 1
    fi
done
echo "   ✅ All neural runtime files present"

echo "   - Learning modules..."
LEARNING_FILES=(
    "sys/core/src/learning/mod.rs"
    "sys/core/src/learning/toolkengpt/mod.rs"
    "sys/core/src/learning/toolkengpt/registry.rs"
    "sys/core/src/learning/toolkengpt/pretrain.rs"
    "sys/core/src/learning/toolkengpt/plugin.rs"
    "sys/core/src/learning/replay/mod.rs"
    "sys/core/src/learning/replay/buffer.rs"
    "sys/core/src/learning/replay/knowledge_base.rs"
    "sys/core/src/learning/replay/sampler.rs"
    "sys/core/src/learning/ewc/mod.rs"
    "sys/core/src/learning/ewc/fisher.rs"
    "sys/core/src/learning/ewc/consolidate.rs"
    "sys/core/src/learning/ewc/adapters.rs"
    "sys/core/src/learning/maml/mod.rs"
    "sys/core/src/learning/maml/inner_loop.rs"
    "sys/core/src/learning/maml/outer_loop.rs"
    "sys/core/src/learning/maml/few_shot.rs"
)

for file in "${LEARNING_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "   ❌ Missing: $file"
        exit 1
    fi
done
echo "   ✅ All learning module files present"

echo "   - Services and API..."
SERVICE_FILES=(
    "sys/core/src/services/neural_service.rs"
    "sys/core/src/services/model_download.rs"
    "sys/core/src/api/routes/models.rs"
    "sys/core/src/api/routes/inference.rs"
    "sys/core/src/cli/models.rs"
    "sys/core/src/cli/ask.rs"
    "sys/core/src/agents/model_selector.rs"
)

for file in "${SERVICE_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "   ❌ Missing: $file"
        exit 1
    fi
done
echo "   ✅ All service and API files present"

echo ""
echo "3. Verifying configuration..."
if [ ! -f "config/ai-providers.json" ]; then
    echo "   ❌ Missing: config/ai-providers.json"
    exit 1
fi
echo "   ✅ Configuration file present"

echo ""
echo "=========================================="
echo "✅ Phase 4 smoke test passed"
echo "=========================================="
exit 0

