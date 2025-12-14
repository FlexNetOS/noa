SHELL := /bin/bash

# Use portable PowerShell from noa_root for Constitution §3.1 compliance
NOA_PWSH := $(shell if [ -f "opt/powershell/pwsh.exe" ]; then echo "opt/powershell/pwsh.exe"; else echo "pwsh"; fi)

.PHONY: install-tools check
.PHONY: config-validate quality security check-all

install-tools:
	@echo "Installing contained toolchains into $$NOA_ROOT (default: repo root)..."
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/install-all-tools.ps1
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

check:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

config-validate:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/validate/validate-configs.ps1

quality:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/validate/quality-gates.ps1

security:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/validate/security-gates.ps1

check-all:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/check-all.ps1

