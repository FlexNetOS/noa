SHELL := /bin/bash

# Use portable PowerShell from noa_root for Constitution §3.1 compliance
NOA_PWSH := $(shell if [ -f "opt/powershell/pwsh.exe" ]; then echo "opt/powershell/pwsh.exe"; else echo "pwsh"; fi)

.PHONY: install-tools check test test-unit test-integration test-all

install-tools:
	@echo "Installing contained toolchains into $$NOA_ROOT (default: repo root)..."
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/install-all-tools.ps1
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

check:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

# Testing
test: test-unit

test-unit:
	@echo "Running unit tests..."
	cargo test -p noa-api-client
	cargo test -p noa-ui-shell
	cargo test -p noa-ui-hived

test-integration:
	@echo "Running integration tests..."
	cargo test --test sandbox_integration_test

test-e2e:
	@echo "Running E2E tests (requires running services)..."
	cargo test --test ui_e2e_test -- --ignored

test-all: test-unit test-integration
	@echo "All tests complete"

