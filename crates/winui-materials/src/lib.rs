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
pub use shadow::{DropShadowRecipe, MaterialExt, THEME_SHADOW_DEPTHS, ThemeShadow, shadow};

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
