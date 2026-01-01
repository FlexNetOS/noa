# ADR-003: Dioxus for UI

## Status

Accepted

## Context

NOA needs a user interface supporting:
- Desktop application
- Web application
- Consistent experience

Options considered:
1. Electron + React
2. Tauri + vanilla
3. Tauri + Dioxus
4. Qt

## Decision

Use Dioxus with Tauri for the UI.

## Rationale

1. **Rust-native**: Same language as backend
2. **Cross-platform**: Desktop + Web + Mobile
3. **Reactive**: Virtual DOM with signals
4. **Tauri integration**: First-class support
5. **Component-based**: Familiar React-like patterns
6. **Performance**: Native performance, small bundles

## Consequences

### Positive
- Single codebase for all platforms
- Type-safe components
- Fast hot reload
- Small binary size

### Negative
- Smaller ecosystem than React
- Fewer UI component libraries
- Learning curve for non-Rust devs

## Mitigations

- Build component library (rust-lovable)
- Comprehensive style guide
- Good documentation

## References

- [Dioxus Documentation](https://dioxuslabs.com/)
- [Tauri Documentation](https://tauri.app/)
