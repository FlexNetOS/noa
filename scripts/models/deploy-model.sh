#!/bin/bash
# deploy-model.sh - Deploy model to CAS with versioning
# Usage: deploy-model.sh <model-file> <model-name> <version>

set -euo pipefail

# configsuration
NOA_ROOT="${NOA_ROOT:-/n/noa}"
CAS_SCRIPTS="${NOA_ROOT}/scripts/cas"

# Input validation
if [[ $# -lt 3 ]]; then
    echo "Usage: $0 <model-file> <model-name> <version>" >&2
    echo "Example: $0 /models/llama-3.2.gguf llama-3.2-8b v3.2" >&2
    exit 1
fi

MODEL_FILE="$1"
MODEL_NAME="$2"
VERSION="$3"

echo "=== NOA Model Deployment ==="
echo "File: $MODEL_FILE"
echo "Name: $MODEL_NAME"
echo "Version: $VERSION"
echo ""

# Step 1: Validate model file
echo "[1/5] Validating model file..."
if [[ ! -f "$MODEL_FILE" ]]; then
    echo "Error: Model file not found: $MODEL_FILE" >&2
    exit 1
fi

FILE_SIZE=$(stat -c%s "$MODEL_FILE" 2>/dev/null || stat -f%z "$MODEL_FILE" 2>/dev/null || echo 0)
echo "  File size: $(numfmt --to=iec $FILE_SIZE 2>/dev/null || echo ${FILE_SIZE}B)"
echo "  ✓ Model file valid"
echo ""

# Step 2: Store in CAS
echo "[2/5] Storing in CAS..."
METADATA=$(cat <<EOF
{
  "name": "$MODEL_NAME",
  "version": "$VERSION",
  "size_bytes": $FILE_SIZE,
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
)

HASH=$(bash "$CAS_SCRIPTS/store-object.sh" "$MODEL_FILE" model "$METADATA")
echo "  Hash: $HASH"
echo "  ✓ Stored in CAS"
echo ""

# Step 3: Create version tag
echo "[3/5] Creating version tag..."
TAG_NAME="${MODEL_NAME}-${VERSION}"
bash "$CAS_SCRIPTS/create-tag.sh" "$TAG_NAME" "$HASH" "Model: $MODEL_NAME version $VERSION"
echo "  Tag: $TAG_NAME"
echo "  ✓ Version tag created"
echo ""

# Step 4: Update current ref
echo "[4/5] Updating current model pointer..."
REF_NAME="models/${MODEL_NAME}/current"
bash "$CAS_SCRIPTS/update-ref.sh" "$REF_NAME" "$HASH" "Deploy $MODEL_NAME $VERSION"
echo "  Ref: $REF_NAME"
echo "  ✓ Current pointer updated"
echo ""

# Step 5: Update registry
echo "[5/5] Updating registry..."
echo "  Registry: $NOA_ROOT/cas/registry/models.json"
echo "  ✓ Registry updated (simulated)"
echo ""

echo "=== Deployment Complete ==="
echo "Model: $MODEL_NAME ($VERSION)"
echo "Hash: $HASH"
echo "Tag: $TAG_NAME"
echo "Ref: $REF_NAME"
echo ""
echo "To retrieve model:"
echo "  bash $CAS_SCRIPTS/retrieve-object.sh $HASH /output/path"
echo ""
echo "To use in agent template:"
echo "  \"model_path\": \"\${NOA_ROOT}/cas/refs/$REF_NAME\""
