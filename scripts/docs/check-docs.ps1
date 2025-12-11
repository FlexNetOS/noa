#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "Checking markdown links..." -ForegroundColor Cyan
node scripts/docs/check-links.mjs

Write-Host "Running markdownlint (via npx)..." -ForegroundColor Cyan
npx --yes markdownlint-cli2 `
  "README.md" `
  "QUICKSTART.md" `
  "CONTRIBUTING.md" `
  "config/README.md" `
  "ai/shared/policy/**/*.md" `
  "docs/**/*.md"

Write-Host "Docs checks passed." -ForegroundColor Green


