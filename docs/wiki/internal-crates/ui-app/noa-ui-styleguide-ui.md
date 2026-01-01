# noa-ui-styleguide-ui Crate

Interactive style guide viewer.

**Location**: `ui/app/crates/noa-ui-styleguide-ui/`

## Overview

Visual documentation for the design system:

- Color swatches
- Typography samples
- Component gallery
- Interactive demos

## Components

### Styleguide

Main styleguide container.

```rust
#[component]
pub fn Styleguide() -> Element {
    rsx! {
        div { class: "styleguide",
            ColorPalette {}
            TypographyScale {}
            SpacingScale {}
            ComponentGallery {}
        }
    }
}
```

### ColorPalette

Color swatch display.

```rust
#[component]
pub fn ColorPalette() -> Element {
    rsx! {
        section { class: "color-palette",
            h2 { "Colors" }
            ColorSwatch { name: "Primary", color: colors::PRIMARY_500 }
            ColorSwatch { name: "Success", color: colors::SUCCESS }
            ColorSwatch { name: "Warning", color: colors::WARNING }
            ColorSwatch { name: "Error", color: colors::ERROR }
        }
    }
}
```

### ComponentGallery

Interactive component demos.

```rust
#[component]
pub fn ComponentGallery() -> Element {
    rsx! {
        section { class: "component-gallery",
            h2 { "Components" }
            ButtonDemo {}
            InputDemo {}
            CardDemo {}
            ModalDemo {}
        }
    }
}
```

## Access

Navigate to `/styleguide` in development mode.

## See Also

- [noa-ui-styleguide-api](noa-ui-styleguide-api.md) — Design tokens
