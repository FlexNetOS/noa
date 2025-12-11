# Verification Checklist - NDCL (Phase 19)

- [X] NDCL directories present (`sys/desktop/ndcl`, `sys/desktop/proxy`, `sys/desktop/auth`)
- [X] Desktop app wrappers exist (`bin/chatgpt.cmd`, `bin/claude-desktop.cmd`, `bin/github-desktop.cmd`)
- [X] IDE containment wrappers exist (`bin/cursor.cmd`, `bin/code.cmd`)
- [X] Desktop app installers present (`scripts/bootstrap/installers/desktop-apps/*.ps1`)
- [X] Proxy/auth services source present (`sys/desktop/proxy`, `sys/desktop/auth`)
- [X] Desktop isolation test exists (`tests/desktop/test_isolation.py`)
- [X] Data paths configured in `config/desktop-apps.json`
- [X] NDCL registry schema present (`config/schemas/desktop-apps.json`)
- [ ] Latest hash snapshot generated (`test-results/HASHES.txt`)
- [ ] Triple verification recorded (`test-results/pass_a.log`, `pass_b.log`, `pass_c.log`)
