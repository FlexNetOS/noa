# ⚠️ DEPRECATED - MERGED INTO 001-noa-seed-foundation

**Status**: MERGED
**Merged On**: 2025-12-08
**Merged Into**: [001-noa-seed-foundation](../001-noa-seed-foundation/)

---

## What Happened?

The **002-unified-bootstrap** feature has been **merged into 001-noa-seed-foundation** as **Phase 0**.

This was done because:
1. Bootstrap is a **prerequisite** for all other NOA features
2. Tasks need to be in proper **dependency order** - bootstrap FIRST
3. spec-kit is designed for **single feature workflows**
4. All 150 bootstrap tasks now have proper sequencing with core NOA tasks

---

## Where To Find Everything

| Old Location | New Location |
|--------------|--------------|
| `002-unified-bootstrap/spec.md` | `001-noa-seed-foundation/spec.md` (FR-076 to FR-094) |
| `002-unified-bootstrap/plan.md` | `001-noa-seed-foundation/plan.md` (Phase 0 section) |
| `002-unified-bootstrap/tasks.md` | `001-noa-seed-foundation/tasks.md` (B001-B150) |
| `002-unified-bootstrap/configs/tools.json` | `001-noa-seed-foundation/configs/tools.json` + `configs/bootstrap-tools.json` |
| `002-unified-bootstrap/kernel-independence.md` | `001-noa-seed-foundation/kernel-independence.md` (moved 2025-12-08) |

---

## Task Renumbering

Bootstrap tasks are now prefixed with **B** instead of **T**:

- `T001-T150` (old 002 tasks) → `B001-B150` (Phase 0 in unified spec)
- Original 001 tasks start at `T001` (Phase 1+)

---

## Action Required

**Use the unified spec at `specs/001-noa-seed-foundation/`** for all work.

The bootstrap entry points remain:
- Windows: `.\scripts\bootstrap\bootstrap.ps1`
- Unix: `./scripts/bootstrap/bootstrap.sh`

---

**Do NOT add new files to this directory.**

