#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "Checking markdown links..." -ForegroundColor Cyan
node scripts/docs/check-links.mjs
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running markdownlint (via npx)..." -ForegroundColor Cyan
npx --yes markdownlint-cli2 `
  "README.md" `
  "QUICKSTART.md" `
  "CONTRIBUTING.md" `
  "config/README.md" `
  "ai/shared/policy/**/*.md" `
  "docs/index.md" `
  "docs/00-guides/integrators-getting-started.md" `
  "docs/00-guides/integrations-map.md" `
  "docs/00-guides/provider-integration.md" `
  "docs/00-guides/provider-catalog.md" `
  "docs/00-guides/agent-tool-authoring.md" `
  "docs/00-guides/schemas-and-contracts.md" `
  "docs/run-book/integrator-troubleshooting.md" `
  "docs/api/README.md"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Docs checks passed." -ForegroundColor Green


