# ADR-004: Constitutional Governance

## Status

Accepted

## Context

NOA agents have significant capabilities:
- File system access
- Command execution
- Network access
- LLM inference

We need to:
- Ensure user control
- Prevent abuse
- Maintain transparency
- Enable auditability

## Decision

Implement constitutional governance:
1. CONSTITUTION.md defines immutable rules
2. Governor enforces policies at runtime
3. All actions are logged for audit

## Rationale

1. **User sovereignty**: Users define what's allowed
2. **Transparency**: Actions are visible and logged
3. **Flexibility**: Policies can be customized
4. **Defense in depth**: Multiple enforcement points
5. **Inspired by**: Claude's Constitutional AI approach

## Key Principles

From CONSTITUTION.md:

1. **Data Sovereignty**: All data stays under user control
2. **Minimal Authority**: Least privilege principle
3. **Transparency**: All actions logged
4. **Consent**: Explicit permission for sensitive actions
5. **Reversibility**: Actions should be undoable when possible

## Consequences

### Positive
- User trust
- Auditability
- Fine-grained control
- Clear accountability

### Negative
- Performance overhead
- Complexity
- Policy management

## Mitigations

- Efficient policy evaluation
- Sensible defaults
- Policy templates

## References

- [CONSTITUTION.md](../../../../CONSTITUTION.md)
- [Anthropic Constitutional AI](https://www.anthropic.com/constitutional-ai)
