SHELL := /bin/bash

.PHONY: install-tools check

install-tools:
	@echo "Installing contained toolchains into $$NOA_ROOT (default: repo root)..."
	@pwsh -NoLogo -NoProfile -File scripts/setup/install-all-tools.ps1
	@pwsh -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

check:
	@pwsh -NoLogo -NoProfile -File scripts/setup/check-prereqs.ps1

