SHELL := /bin/bash

# Use portable PowerShell from noa_root for Constitution §3.1 compliance
NOA_PWSH := $(shell if [ -f "opt/powershell/pwsh.exe" ]; then echo "opt/powershell/pwsh.exe"; else echo "pwsh"; fi)

.PHONY: install-tools check

install-tools:
	@echo "Installing contained toolchains into $$NOA_ROOT (default: repo root)..."
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/install-all-tools.ps1
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

check:
	@$(NOA_PWSH) -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

