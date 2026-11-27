//! Mica, Acrylic, Reveal and ThemeShadow elevation for Rune, ported from WinUI 3
//! (microsoft/microsoft-ui-xaml `controls/dev/Materials`, `dxaml/xcp/components/graphics`).
//! See docs/ARCHITECTURE.md for the contract.
//!
//! Every material reads its colours from a [`winui_theme::Theme`], so swapping the theme
//! (mode or accent) and rebuilding the view re-tints them.
//!
//! ```no_run
//! use winui_materials::{MaterialExt, acrylic, mica, shadow};
//! use winui_theme::Theme;
//! let theme = Theme::dark();
//! let window = mica(&theme).child(
//!     acrylic(&theme, theme.tokens().solid_background_fill_color_base, 0.5, Some(0.96))
//!         .size(320.0, 200.0)
//!         .radius(8.0)
//!         .theme_shadow(&theme, 32.0),
//! );
//! # let _ = (window, shadow(16.0));
//! ```

mod acrylic;
mod blend;
mod mica;
mod noise;
mod reveal;
mod shadow;

pub use acrylic::{
    AcrylicBrush, BLUR_RADIUS, acrylic, acrylic_brush, acrylic_el, tint_opacity_modifier,
};
pub use blend::{blend_color, blend_luminosity, lum, set_lum};
pub use mica::{MicaBrush, MicaKind, mica, mica_alt, mica_brush};
pub use noise::{NOISE_LEVELS, NOISE_OPACITY, NOISE_SIZE, noise_layer, noise_pixels, noise_tile};
pub use reveal::{Reveal, RevealLight, reveal, reveal_with_colors};
pub use shadow::{
    DropShadowRecipe, MaterialExt, THEME_SHADOW_DEPTHS, ThemeShadow, ambient, card_shadow,
    elevation, shadow,
};

use rune::El;
use winui_theme::{Theme, Tokens};

/// `winui_theme::Color` (sRGB-encoded, straight alpha) → `rune::Color` (linear sRGB, straight
/// alpha). Rune blends in sRGB space by default, matching XAML.
pub fn rc(c: winui_theme::Color) -> rune::Color {
    fn decode(v: f32) -> f32 {
        if v <= 0.040_45 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    rune::Color {
        r: decode(c.r),
        g: decode(c.g),
        b: decode(c.b),
        a: c.a,
    }
}

/// `rune::Color` (linear) → `winui_theme::Color` (sRGB-encoded).
pub fn tc(c: rune::Color) -> winui_theme::Color {
    fn encode(v: f32) -> f32 {
        if v <= 0.003_130_8 {
            v * 12.92
        } else {
            1.055 * v.powf(1.0 / 2.4) - 0.055
        }
    }
    winui_theme::Color {
        r: encode(c.r),
        g: encode(c.g),
        b: encode(c.b),
        a: c.a,
    }
}

/// The theme's tokens.
pub fn tokens(theme: &Theme) -> &Tokens {
    theme.tokens()
}

/// The noise tile filling its parent at `alpha` opacity (Acrylic uses [`NOISE_OPACITY`]).
pub fn noise_overlay(alpha: f32) -> El {
    noise_layer(0.0).opacity(alpha)
}

/// NavigationView expanded-pane background, `AcrylicInAppFillColorDefaultBrush` (same values
/// as `AcrylicBackgroundFillColorDefaultBrush`), filling its parent.
pub fn nav_acrylic(theme: &Theme) -> El {
    AcrylicBrush::background_default(theme)
        .el()
        .key("nav-acrylic")
        .w_full()
        .h_full()
}

/// Resting reveal hairline for list rows: `ControlStrokeColorSecondary` fading downwards.
pub fn reveal_border(theme: &Theme, radius: f32) -> El {
    let stroke = rc(theme.tokens().control_stroke_color_secondary);
    rune::el()
        .key("reveal-border")
        .w_full()
        .h_full()
        .radius(radius)
        .border_gradient(1.0, stroke, stroke.alpha(0.0))
        .inert()
}

/// Pointer-over fill for list/nav items: `SubtleFillColorSecondary` (WinUI 3's replacement
/// for the Reveal background brushes).
pub fn reveal_fill(theme: &Theme) -> rune::Color {
    rc(theme.tokens().subtle_fill_color_secondary)
}
