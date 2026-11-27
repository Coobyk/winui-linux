//! Local stand-in for `winui_theme` (docs/ARCHITECTURE.md § winui-theme), with the contract's
//! public shape: `ThemeMode`, `Color`, `Tokens`, `Theme`, `TypeRamp`, `spacing`, `radii`,
//! `sizes`. Values come from microsoft-ui-xaml `dev/CommonStyles/Common_themeresources_any.xaml`.
//! The integrator swaps this module for `pub use winui_theme::*;` (see the crate README).

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// One sRGBA colour (straight alpha, sRGB-encoded channels).
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
        Color { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }
    }
    /// XAML `#AARRGGBB`.
    const fn argb(v: u32) -> Self {
        let a = (v >> 24) as u8;
        Self::rgba((v >> 16) as u8, (v >> 8) as u8, v as u8, a as f32 / 255.0)
    }
    pub const fn with_alpha(self, a: f32) -> Self {
        Color { a, ..self }
    }
}

/// The Fluent token surface: WinUI resource names, lower-cased (`TextFillColorPrimary` →
/// `text_fill_color_primary`).
#[derive(Clone, Debug)]
pub struct Tokens {
    pub text_fill_color_primary: Color,
    pub text_fill_color_secondary: Color,
    pub text_fill_color_tertiary: Color,
    pub text_fill_color_disabled: Color,
    pub text_fill_color_inverse: Color,
    pub accent_text_fill_color_primary: Color,
    pub accent_text_fill_color_secondary: Color,
    pub accent_text_fill_color_tertiary: Color,
    pub accent_text_fill_color_disabled: Color,
    pub text_on_accent_fill_color_primary: Color,
    pub text_on_accent_fill_color_secondary: Color,
    pub text_on_accent_fill_color_disabled: Color,
    pub control_fill_color_default: Color,
    pub control_fill_color_secondary: Color,
    pub control_fill_color_tertiary: Color,
    pub control_fill_color_disabled: Color,
    pub control_fill_color_transparent: Color,
    pub control_fill_color_input_active: Color,
    pub control_strong_fill_color_default: Color,
    pub control_strong_fill_color_disabled: Color,
    pub control_solid_fill_color_default: Color,
    pub control_alt_fill_color_transparent: Color,
    pub control_alt_fill_color_secondary: Color,
    pub control_alt_fill_color_tertiary: Color,
    pub control_alt_fill_color_quarternary: Color,
    pub control_alt_fill_color_disabled: Color,
    pub subtle_fill_color_transparent: Color,
    pub subtle_fill_color_secondary: Color,
    pub subtle_fill_color_tertiary: Color,
    pub subtle_fill_color_disabled: Color,
    pub accent_fill_color_default: Color,
    pub accent_fill_color_secondary: Color,
    pub accent_fill_color_tertiary: Color,
    pub accent_fill_color_disabled: Color,
    pub control_stroke_color_default: Color,
    pub control_stroke_color_secondary: Color,
    pub control_stroke_color_on_accent_default: Color,
    pub control_stroke_color_on_accent_secondary: Color,
    pub control_stroke_color_on_accent_disabled: Color,
    pub control_strong_stroke_color_default: Color,
    pub control_strong_stroke_color_disabled: Color,
    pub card_stroke_color_default: Color,
    pub card_stroke_color_default_solid: Color,
    pub surface_stroke_color_default: Color,
    pub surface_stroke_color_flyout: Color,
    pub divider_stroke_color_default: Color,
    pub focus_stroke_color_outer: Color,
    pub focus_stroke_color_inner: Color,
    pub card_background_fill_color_default: Color,
    pub card_background_fill_color_secondary: Color,
    pub smoke_fill_color_default: Color,
    pub layer_fill_color_default: Color,
    pub layer_fill_color_alt: Color,
    pub layer_on_mica_base_alt_fill_color_default: Color,
    pub layer_on_mica_base_alt_fill_color_secondary: Color,
    pub layer_on_mica_base_alt_fill_color_tertiary: Color,
    pub solid_background_fill_color_base: Color,
    pub solid_background_fill_color_secondary: Color,
    pub solid_background_fill_color_tertiary: Color,
    pub solid_background_fill_color_quarternary: Color,
    /// `AcrylicBackgroundFillColorDefault`'s tint, flattened (flyouts, tooltips).
    pub acrylic_background_fill_color_default: Color,
    pub system_fill_color_attention: Color,
    pub system_fill_color_success: Color,
    pub system_fill_color_caution: Color,
    pub system_fill_color_critical: Color,
    pub system_fill_color_neutral: Color,
    pub system_fill_color_attention_background: Color,
    pub system_fill_color_success_background: Color,
    pub system_fill_color_caution_background: Color,
    pub system_fill_color_critical_background: Color,
}

/// The Windows accent ramp (`SystemAccentColor` and its Light1–3 / Dark1–3 steps).
#[derive(Clone, Copy, Debug)]
pub struct AccentRamp {
    pub base: Color,
    pub light1: Color,
    pub light2: Color,
    pub light3: Color,
    pub dark1: Color,
    pub dark2: Color,
    pub dark3: Color,
}

impl AccentRamp {
    /// Windows 11's default blue, `#0078D4`.
    pub const DEFAULT: AccentRamp = AccentRamp {
        base: Color::rgb(0x00, 0x78, 0xD4),
        light1: Color::rgb(0x00, 0x93, 0xF9),
        light2: Color::rgb(0x60, 0xCC, 0xFF),
        light3: Color::rgb(0x98, 0xEC, 0xFF),
        dark1: Color::rgb(0x00, 0x5F, 0xB8),
        dark2: Color::rgb(0x00, 0x43, 0x90),
        dark3: Color::rgb(0x00, 0x26, 0x42),
    };

    /// A plausible ramp around `base` (steps in lightness, like Windows' accent generator).
    pub fn from_base(base: Color) -> AccentRamp {
        if base == Self::DEFAULT.base {
            return Self::DEFAULT;
        }
        let mix = |t: f32, to: f32| Color {
            r: base.r + (to - base.r) * t,
            g: base.g + (to - base.g) * t,
            b: base.b + (to - base.b) * t,
            a: 1.0,
        };
        AccentRamp {
            base,
            light1: mix(0.2, 1.0),
            light2: mix(0.45, 1.0),
            light3: mix(0.7, 1.0),
            dark1: mix(0.2, 0.0),
            dark2: mix(0.42, 0.0),
            dark3: mix(0.68, 0.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextStyle {
    pub size: f32,
    pub weight: u16,
    pub line_height: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TypeRamp {
    pub caption: TextStyle,
    pub body: TextStyle,
    pub body_strong: TextStyle,
    pub subtitle: TextStyle,
    pub title: TextStyle,
    pub title_large: TextStyle,
    pub display: TextStyle,
}

/// WinUI's `TypographyThemeResources` (Segoe UI Variable ramp).
pub const TYPE_RAMP: TypeRamp = TypeRamp {
    caption: TextStyle { size: 12.0, weight: 400, line_height: 16.0 },
    body: TextStyle { size: 14.0, weight: 400, line_height: 20.0 },
    body_strong: TextStyle { size: 14.0, weight: 600, line_height: 20.0 },
    subtitle: TextStyle { size: 20.0, weight: 600, line_height: 28.0 },
    title: TextStyle { size: 28.0, weight: 600, line_height: 36.0 },
    title_large: TextStyle { size: 40.0, weight: 600, line_height: 52.0 },
    display: TextStyle { size: 68.0, weight: 600, line_height: 92.0 },
};

#[derive(Clone, Debug)]
pub struct Theme {
    mode: ThemeMode,
    accent: AccentRamp,
    tokens: Tokens,
}

impl Theme {
    pub fn light() -> Self {
        Self::resolve(ThemeMode::Light, AccentRamp::DEFAULT)
    }
    pub fn dark() -> Self {
        Self::resolve(ThemeMode::Dark, AccentRamp::DEFAULT)
    }
    pub fn of(mode: ThemeMode) -> Self {
        Self::resolve(mode, AccentRamp::DEFAULT)
    }
    pub fn mode(&self) -> ThemeMode {
        self.mode
    }
    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }
    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }
    pub fn accent(&self) -> &AccentRamp {
        &self.accent
    }
    pub fn type_ramp(&self) -> &TypeRamp {
        &TYPE_RAMP
    }
    /// Re-resolve with a different accent colour.
    pub fn with_accent(&self, accent: Color) -> Self {
        Self::resolve(self.mode, AccentRamp::from_base(accent))
    }

    fn resolve(mode: ThemeMode, a: AccentRamp) -> Self {
        let tokens = match mode {
            ThemeMode::Light => light(&a),
            ThemeMode::Dark => dark(&a),
        };
        Theme { mode, accent: a, tokens }
    }
}

fn light(a: &AccentRamp) -> Tokens {
    let c = Color::argb;
    Tokens {
        text_fill_color_primary: c(0xE4000000),
        text_fill_color_secondary: c(0x9E000000),
        text_fill_color_tertiary: c(0x72000000),
        text_fill_color_disabled: c(0x5C000000),
        text_fill_color_inverse: c(0xFFFFFFFF),
        accent_text_fill_color_primary: a.dark2,
        accent_text_fill_color_secondary: a.dark3,
        accent_text_fill_color_tertiary: a.dark1,
        accent_text_fill_color_disabled: c(0x5C000000),
        text_on_accent_fill_color_primary: c(0xFFFFFFFF),
        text_on_accent_fill_color_secondary: c(0xB3FFFFFF),
        text_on_accent_fill_color_disabled: c(0xFFFFFFFF),
        control_fill_color_default: c(0xB3FFFFFF),
        control_fill_color_secondary: c(0x80F9F9F9),
        control_fill_color_tertiary: c(0x4DF9F9F9),
        control_fill_color_disabled: c(0x4DF9F9F9),
        control_fill_color_transparent: c(0x00FFFFFF),
        control_fill_color_input_active: c(0xFFFFFFFF),
        control_strong_fill_color_default: c(0x72000000),
        control_strong_fill_color_disabled: c(0x51000000),
        control_solid_fill_color_default: c(0xFFFFFFFF),
        control_alt_fill_color_transparent: c(0x00FFFFFF),
        control_alt_fill_color_secondary: c(0x06000000),
        control_alt_fill_color_tertiary: c(0x0F000000),
        control_alt_fill_color_quarternary: c(0x18000000),
        control_alt_fill_color_disabled: c(0x00FFFFFF),
        subtle_fill_color_transparent: c(0x00FFFFFF),
        subtle_fill_color_secondary: c(0x09000000),
        subtle_fill_color_tertiary: c(0x06000000),
        subtle_fill_color_disabled: c(0x00FFFFFF),
        accent_fill_color_default: a.dark1,
        accent_fill_color_secondary: a.dark1.with_alpha(0.9),
        accent_fill_color_tertiary: a.dark1.with_alpha(0.8),
        accent_fill_color_disabled: c(0x37000000),
        control_stroke_color_default: c(0x0F000000),
        control_stroke_color_secondary: c(0x29000000),
        control_stroke_color_on_accent_default: c(0x14FFFFFF),
        control_stroke_color_on_accent_secondary: c(0x66000000),
        control_stroke_color_on_accent_disabled: c(0x0F000000),
        control_strong_stroke_color_default: c(0x72000000),
        control_strong_stroke_color_disabled: c(0x37000000),
        card_stroke_color_default: c(0x0F000000),
        card_stroke_color_default_solid: c(0xFFEBEBEB),
        surface_stroke_color_default: c(0x66757575),
        surface_stroke_color_flyout: c(0x0F000000),
        divider_stroke_color_default: c(0x14000000),
        focus_stroke_color_outer: c(0xE4000000),
        focus_stroke_color_inner: c(0xB3FFFFFF),
        card_background_fill_color_default: c(0xB3FFFFFF),
        card_background_fill_color_secondary: c(0x80F6F6F6),
        smoke_fill_color_default: c(0x4D000000),
        layer_fill_color_default: c(0x80FFFFFF),
        layer_fill_color_alt: c(0xFFFFFFFF),
        layer_on_mica_base_alt_fill_color_default: c(0xB3FFFFFF),
        layer_on_mica_base_alt_fill_color_secondary: c(0x0A000000),
        layer_on_mica_base_alt_fill_color_tertiary: c(0xFFF9F9F9),
        solid_background_fill_color_base: c(0xFFF3F3F3),
        solid_background_fill_color_secondary: c(0xFFEEEEEE),
        solid_background_fill_color_tertiary: c(0xFFF9F9F9),
        solid_background_fill_color_quarternary: c(0xFFFFFFFF),
        acrylic_background_fill_color_default: c(0xFFFCFCFC),
        system_fill_color_attention: a.dark1,
        system_fill_color_success: c(0xFF0F7B0F),
        system_fill_color_caution: c(0xFF9D5D00),
        system_fill_color_critical: c(0xFFC42B1C),
        system_fill_color_neutral: c(0x72000000),
        system_fill_color_attention_background: c(0x80F6F6F6),
        system_fill_color_success_background: c(0xFFDFF6DD),
        system_fill_color_caution_background: c(0xFFFFF4CE),
        system_fill_color_critical_background: c(0xFFFDE7E9),
    }
}

fn dark(a: &AccentRamp) -> Tokens {
    let c = Color::argb;
    Tokens {
        text_fill_color_primary: c(0xFFFFFFFF),
        text_fill_color_secondary: c(0xC5FFFFFF),
        text_fill_color_tertiary: c(0x87FFFFFF),
        text_fill_color_disabled: c(0x5DFFFFFF),
        text_fill_color_inverse: c(0xE4000000),
        accent_text_fill_color_primary: a.light3,
        accent_text_fill_color_secondary: a.light3,
        accent_text_fill_color_tertiary: a.light2,
        accent_text_fill_color_disabled: c(0x5DFFFFFF),
        text_on_accent_fill_color_primary: c(0xFF000000),
        text_on_accent_fill_color_secondary: c(0x80000000),
        text_on_accent_fill_color_disabled: c(0x87FFFFFF),
        control_fill_color_default: c(0x0FFFFFFF),
        control_fill_color_secondary: c(0x15FFFFFF),
        control_fill_color_tertiary: c(0x08FFFFFF),
        control_fill_color_disabled: c(0x0BFFFFFF),
        control_fill_color_transparent: c(0x00FFFFFF),
        control_fill_color_input_active: c(0xB31E1E1E),
        control_strong_fill_color_default: c(0x8BFFFFFF),
        control_strong_fill_color_disabled: c(0x3FFFFFFF),
        control_solid_fill_color_default: c(0xFF454545),
        control_alt_fill_color_transparent: c(0x00FFFFFF),
        control_alt_fill_color_secondary: c(0x19000000),
        control_alt_fill_color_tertiary: c(0x0BFFFFFF),
        control_alt_fill_color_quarternary: c(0x12FFFFFF),
        control_alt_fill_color_disabled: c(0x00FFFFFF),
        subtle_fill_color_transparent: c(0x00FFFFFF),
        subtle_fill_color_secondary: c(0x0FFFFFFF),
        subtle_fill_color_tertiary: c(0x0AFFFFFF),
        subtle_fill_color_disabled: c(0x00FFFFFF),
        accent_fill_color_default: a.light2,
        accent_fill_color_secondary: a.light2.with_alpha(0.9),
        accent_fill_color_tertiary: a.light2.with_alpha(0.8),
        accent_fill_color_disabled: c(0x28FFFFFF),
        control_stroke_color_default: c(0x12FFFFFF),
        control_stroke_color_secondary: c(0x18FFFFFF),
        control_stroke_color_on_accent_default: c(0x14FFFFFF),
        control_stroke_color_on_accent_secondary: c(0x23000000),
        control_stroke_color_on_accent_disabled: c(0x33000000),
        control_strong_stroke_color_default: c(0x8BFFFFFF),
        control_strong_stroke_color_disabled: c(0x28FFFFFF),
        card_stroke_color_default: c(0x19000000),
        card_stroke_color_default_solid: c(0xFF1C1C1C),
        surface_stroke_color_default: c(0x66757575),
        surface_stroke_color_flyout: c(0x33000000),
        divider_stroke_color_default: c(0x15FFFFFF),
        focus_stroke_color_outer: c(0xFFFFFFFF),
        focus_stroke_color_inner: c(0xB3000000),
        card_background_fill_color_default: c(0x0DFFFFFF),
        card_background_fill_color_secondary: c(0x08FFFFFF),
        smoke_fill_color_default: c(0x4D000000),
        layer_fill_color_default: c(0x4C3A3A3A),
        layer_fill_color_alt: c(0x0DFFFFFF),
        layer_on_mica_base_alt_fill_color_default: c(0x733A3A3A),
        layer_on_mica_base_alt_fill_color_secondary: c(0x0FFFFFFF),
        layer_on_mica_base_alt_fill_color_tertiary: c(0xFF2C2C2C),
        solid_background_fill_color_base: c(0xFF202020),
        solid_background_fill_color_secondary: c(0xFF1C1C1C),
        solid_background_fill_color_tertiary: c(0xFF282828),
        solid_background_fill_color_quarternary: c(0xFF2C2C2C),
        acrylic_background_fill_color_default: c(0xFF2C2C2C),
        system_fill_color_attention: a.light2,
        system_fill_color_success: c(0xFF6CCB5F),
        system_fill_color_caution: c(0xFFFCE100),
        system_fill_color_critical: c(0xFFFF99A4),
        system_fill_color_neutral: c(0x8BFFFFFF),
        system_fill_color_attention_background: c(0x08FFFFFF),
        system_fill_color_success_background: c(0xFF393D1B),
        system_fill_color_caution_background: c(0xFF433519),
        system_fill_color_critical_background: c(0xFF442726),
    }
}

pub mod spacing {
    pub const XXS: f32 = 2.0;
    pub const XS: f32 = 4.0;
    pub const S: f32 = 8.0;
    pub const M: f32 = 12.0;
    pub const L: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 28.0;
}

pub mod radii {
    pub const CONTROL: f32 = 4.0;
    pub const OVERLAY: f32 = 8.0;
    pub const CIRCLE: f32 = 999.0;
}

pub mod sizes {
    pub const CONTROL_HEIGHT: f32 = 32.0;
    pub const NAV_PANE_OPEN: f32 = 320.0;
    pub const NAV_PANE_COMPACT: f32 = 48.0;
}
