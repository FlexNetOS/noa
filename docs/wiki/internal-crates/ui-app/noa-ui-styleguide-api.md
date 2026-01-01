# noa-ui-styleguide-api Crate

Style system API and tokens.

**Location**: `ui/app/crates/noa-ui-styleguide-api/`

## Overview

Design system tokens and utilities:

- Color palette
- Spacing scale
- Typography
- Shadows

## Colors

```rust
pub mod colors {
    // Primary
    pub const PRIMARY_50: &str = "#eff6ff";
    pub const PRIMARY_500: &str = "#3b82f6";
    pub const PRIMARY_900: &str = "#1e3a8a";
    
    // Neutral
    pub const NEUTRAL_50: &str = "#fafafa";
    pub const NEUTRAL_900: &str = "#171717";
    
    // Semantic
    pub const SUCCESS: &str = "#22c55e";
    pub const WARNING: &str = "#f59e0b";
    pub const ERROR: &str = "#ef4444";
}
```

## Spacing

```rust
pub mod spacing {
    pub const XS: &str = "0.25rem";   // 4px
    pub const SM: &str = "0.5rem";    // 8px
    pub const MD: &str = "1rem";      // 16px
    pub const LG: &str = "1.5rem";    // 24px
    pub const XL: &str = "2rem";      // 32px
}
```

## Typography

```rust
pub mod typography {
    pub const FONT_SANS: &str = "Inter, system-ui, sans-serif";
    pub const FONT_MONO: &str = "JetBrains Mono, monospace";
    
    pub const TEXT_XS: &str = "0.75rem";
    pub const TEXT_SM: &str = "0.875rem";
    pub const TEXT_BASE: &str = "1rem";
    pub const TEXT_LG: &str = "1.125rem";
    pub const TEXT_XL: &str = "1.25rem";
}
```

## Theme

```rust
pub enum Theme {
    Light,
    Dark,
    System,
}

pub struct ThemeConfig {
    pub background: &'static str,
    pub foreground: &'static str,
    pub primary: &'static str,
    pub accent: &'static str,
}
```

## See Also

- [noa-ui-styleguide-ui](noa-ui-styleguide-ui.md) — Style viewer
