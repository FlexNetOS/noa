#!/usr/bin/env bash
set -euo pipefail

echo "Checking markdown links..."
node scripts/docs/check-links.mjs

echo "Running markdownlint (via npx)..."
npx --yes markdownlint-cli2 \
  "README.md" \
  "QUICKSTART.md" \
  "CONTRIBUTING.md" \
  "config/README.md" \
  "ai/shared/policy/**/*.md" \
  "docs/index.md" \
  "docs/00-guides/integrators-getting-started.md" \
  "docs/00-guides/integrations-map.md" \
  "docs/00-guides/provider-integration.md" \
  "docs/00-guides/provider-catalog.md" \
  "docs/00-guides/agent-tool-authoring.md" \
  "docs/00-guides/schemas-and-contracts.md" \
  "docs/run-book/integrator-troubleshooting.md" \
  "docs/api/README.md"

echo "Docs checks passed."


