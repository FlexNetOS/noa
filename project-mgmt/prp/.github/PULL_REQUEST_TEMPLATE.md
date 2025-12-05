# PRP-XXX: [Title]

## Summary
<!-- Brief description of what this PR does -->

## Related PRP
- Implements: [PRP-XXX](../PRPs/PRP-XXX.md)
- Next: [PRP-YYY](../PRPs/PRP-YYY.md) (if applicable)

## Changes Made

### Added
-

### Changed
-

### Removed
-

### Fixed
-

## Definition of Done (DOD)

<!-- Copy from PRP and check each item -->
- [ ] All requirements implemented
- [ ] Unit tests written and passing
- [ ] Integration tests written and passing (if applicable)
- [ ] Code linted and formatted (`npm run lint && npm run format`)
- [ ] No TypeScript errors (`npm run typecheck`)
- [ ] Documentation updated (README, CLAUDE.md, JSDoc)
- [ ] PRP progress updated in PRP-XXX.md
- [ ] **CHANGELOG.md updated in [Unreleased] section** ⚠️ MANDATORY

## CHANGELOG Update

**CRITICAL**: This PR updates CHANGELOG.md
- [ ] I have updated CHANGELOG.md in the [Unreleased] section
- [ ] Changes are categorized (Added/Changed/Removed/Fixed/Security)
- [ ] Entry is clear and describes the user-facing impact
- [ ] Follows [Keep a Changelog](https://keepachangelog.com/) format

## Testing

### Unit Tests
<!-- List unit tests added/updated -->
-

### Integration Tests
<!-- List integration tests added/updated (if applicable) -->
-

### Manual Testing
<!-- Steps to manually test this PR -->
1.
2.
3.

## Screenshots/Logs (if applicable)
<!-- Add screenshots or logs demonstrating the changes -->

## Breaking Changes
<!-- List any breaking changes and migration steps -->
- [ ] This PR contains breaking changes
- [ ] Migration guide provided

## Next Steps
<!-- What should be done after this PR is merged -->
-

## Reviewer Checklist

- [ ] CHANGELOG.md updated (MANDATORY)
- [ ] All DOD criteria met
- [ ] Tests passing (CI green - all jobs)
- [ ] Code follows TypeScript strict mode
- [ ] No `any` types without justification
- [ ] Code follows architecture patterns from CLAUDE.md
- [ ] No linter/type errors
- [ ] PRP progress updated
- [ ] Documentation clear and complete
- [ ] Pre-commit hooks pass locally

---

**Deployment**: This PR will be published to npm when a new version tag is created on `main`.
