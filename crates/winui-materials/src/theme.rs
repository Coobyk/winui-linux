//! Local stand-in for the `winui_theme` contract (docs/ARCHITECTURE.md).
//!
//! `winui-theme` is being built in parallel and has no public API yet, so this module mirrors
//! the contract types (`ThemeMode`, `Color`, `Tokens`, `Theme`) with the token fields materials
//! read. Values are the real WinUI resources (`Common_themeresources_any.xaml`, default accent
//! `#0078D4`). Once `winui_theme` ships, replace this module's body with
//! `pub use winui_theme::{Color, Theme, ThemeMode, Tokens};`; field names already follow the
//! contract's lower-cased WinUI resource names.

/// Light or dark.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// One sRGB colour with straight alpha, components in `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 1.0)
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }
    /// XAML `#AARRGGBB`.
    pub const fn argb(argb: u32) -> Self {
        let [a, r, g, b] = argb.to_be_bytes();
        Self::rgba(r, g, b, a as f32 / 255.0)
    }
}

/// The token subset materials read.
#[derive(Clone, Debug, PartialEq)]
pub struct Tokens {
    pub solid_background_fill_color_base: Color,
    pub solid_background_fill_color_base_alt: Color,
    pub solid_background_fill_color_secondary: Color,
    pub solid_background_fill_color_tertiary: Color,
    pub accent_fill_color_default: Color,
    pub text_fill_color_primary: Color,
    pub text_fill_color_secondary: Color,
    pub control_stroke_color_default: Color,
    pub card_background_fill_color_default: Color,
    pub card_stroke_color_default: Color,
    pub subtle_fill_color_secondary: Color,
    pub layer_fill_color_default: Color,
}

/// Mode + accent + resolved tokens.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    mode: ThemeMode,
    accent: Color,
    tokens: Tokens,
}

const DEFAULT_ACCENT: Color = Color::rgb(0x00, 0x78, 0xD4);

impl Theme {
    pub fn light() -> Self {
        Self::resolve(ThemeMode::Light, DEFAULT_ACCENT)
    }
    pub fn dark() -> Self {
        Self::resolve(ThemeMode::Dark, DEFAULT_ACCENT)
    }
    pub fn mode(&self) -> ThemeMode {
        self.mode
    }
    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }
    pub fn accent(&self) -> Color {
        self.accent
    }
    pub fn with_accent(&self, accent: Color) -> Self {
        Self::resolve(self.mode, accent)
    }

    fn resolve(mode: ThemeMode, accent: Color) -> Self {
        let a = Color::argb;
        let tokens = match mode {
            ThemeMode::Light => Tokens {
                solid_background_fill_color_base: a(0xFFF3F3F3),
                solid_background_fill_color_base_alt: a(0xFFDADADA),
                solid_background_fill_color_secondary: a(0xFFEEEEEE),
                solid_background_fill_color_tertiary: a(0xFFF9F9F9),
                // SystemAccentColorDark1 for the default accent.
                accent_fill_color_default: shade(accent, -0.2),
                text_fill_color_primary: a(0xE4000000),
                text_fill_color_secondary: a(0x9E000000),
                control_stroke_color_default: a(0x0F000000),
                card_background_fill_color_default: a(0xB3FFFFFF),
                card_stroke_color_default: a(0x0F000000),
                subtle_fill_color_secondary: a(0x09000000),
                layer_fill_color_default: a(0x80FFFFFF),
            },
            ThemeMode::Dark => Tokens {
                solid_background_fill_color_base: a(0xFF202020),
                solid_background_fill_color_base_alt: a(0xFF0A0A0A),
                solid_background_fill_color_secondary: a(0xFF1C1C1C),
                solid_background_fill_color_tertiary: a(0xFF282828),
                // SystemAccentColorLight2 for the default accent.
                accent_fill_color_default: shade(accent, 0.6),
                text_fill_color_primary: a(0xFFFFFFFF),
                text_fill_color_secondary: a(0xC5FFFFFF),
                control_stroke_color_default: a(0x12FFFFFF),
                card_background_fill_color_default: a(0x0DFFFFFF),
                card_stroke_color_default: a(0x19000000),
                subtle_fill_color_secondary: a(0x0FFFFFFF),
                layer_fill_color_default: a(0x4C3A3A3A),
            },
        };
        Self {
            mode,
            accent,
            tokens,
        }
    }
}

/// Approximate accent ramp step: the shim's stand-in for winui-theme's real ramp. The default
/// accent resolves to the shipped `#005FB8` (Dark1) / `#60CDFF` (Light2) exactly.
fn shade(accent: Color, amount: f32) -> Color {
    if accent == DEFAULT_ACCENT {
        return if amount < 0.0 {
            Color::rgb(0x00, 0x5F, 0xB8)
        } else {
            Color::rgb(0x60, 0xCD, 0xFF)
        };
    }
    let mix = |c: f32| {
        if amount < 0.0 {
            c * (1.0 + amount)
        } else {
            c + (1.0 - c) * amount
        }
    };
    Color {
        r: mix(accent.r),
        g: mix(accent.g),
        b: mix(accent.b),
        a: accent.a,
    }
}
