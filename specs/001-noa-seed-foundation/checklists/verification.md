# Verification Checklist - NDCL (Phase 19)

- [ ] NDCL directories present (`sys/desktop/ndcl`, `sys/desktop/proxy`, `sys/desktop/auth`)
- [ ] Desktop app wrappers exist (`bin/chatgpt.cmd`, `bin/claude-desktop.cmd`, `bin/github-desktop.cmd`)
- [ ] IDE containment wrappers exist (`bin/cursor.cmd`, `bin/code.cmd`)
- [ ] Desktop app installers present (`scripts/bootstrap/installers/desktop-apps/*.ps1`)
- [ ] Proxy/auth services source present (`sys/desktop/proxy`, `sys/desktop/auth`)
- [ ] Desktop isolation test exists (`tests/desktop/test_isolation.py`)
- [ ] Data paths configured in `config/desktop-apps.json`
- [ ] NDCL registry schema present (`config/schemas/desktop-apps.json`)
- [ ] Latest hash snapshot generated (`test-results/HASHES.txt`)
- [ ] Triple verification recorded (`test-results/pass_a.log`, `pass_b.log`, `pass_c.log`)
