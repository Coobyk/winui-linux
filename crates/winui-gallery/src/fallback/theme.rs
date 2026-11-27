//! Fallback for the `winui-theme` contract (docs/ARCHITECTURE.md): WinUI 3 resource values
//! from microsoft-ui-xaml `Common_themeresources_any.xaml` (Light / Default=Dark dictionaries).

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// One sRGBA colour (components 0..=1, not linear).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }
    }
    /// XAML `#AARRGGBB` / `#RRGGBB`.
    pub const fn argb(v: u32) -> Self {
        let c = Self::rgb((v >> 16) as u8, (v >> 8) as u8, v as u8);
        Self { a: (v >> 24) as f32 / 255.0, ..c }
    }
    pub const fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }
    pub fn parse(hex: &str) -> Option<Self> {
        let h = hex.strip_prefix('#').unwrap_or(hex);
        let v = u32::from_str_radix(h, 16).ok()?;
        match h.len() {
            6 => Some(Self::argb(0xFF00_0000 | v)),
            8 => Some(Self::argb(v)),
            _ => None,
        }
    }
    pub fn to_hex(self) -> String {
        let q = |c: f32| (c.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("#{:02X}{:02X}{:02X}", q(self.r), q(self.g), q(self.b))
    }
    /// Lightness shift in OKLab-ish space via the rune colour helpers.
    fn shade(self, dl: f32) -> Self {
        let lab = rune::Color::from(self).to_oklab();
        let c = rune::Color::from_oklab([(lab[0] + dl).clamp(0.0, 1.0), lab[1], lab[2], lab[3]]);
        c.into()
    }
}

fn dec(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}
fn enc(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}

impl From<Color> for rune::Color {
    fn from(c: Color) -> rune::Color {
        rune::Color { r: dec(c.r), g: dec(c.g), b: dec(c.b), a: c.a }
    }
}
impl From<rune::Color> for Color {
    fn from(c: rune::Color) -> Color {
        Color { r: enc(c.r), g: enc(c.g), b: enc(c.b), a: c.a }
    }
}

/// System accent ramp (`SystemAccentColor{Light,Dark}{1,2,3}`).
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
    pub const DEFAULT_ACCENT: Color = Color::rgb(0x00, 0x78, 0xD4);

    pub fn new(base: Color) -> Self {
        if base == Self::DEFAULT_ACCENT {
            // Windows' shipped ramp for the default blue.
            return Self {
                base,
                light1: Color::rgb(0x00, 0x93, 0xF9),
                light2: Color::rgb(0x60, 0xCC, 0xFF),
                light3: Color::rgb(0x98, 0xEC, 0xFF),
                dark1: Color::rgb(0x00, 0x5F, 0xB8),
                dark2: Color::rgb(0x00, 0x47, 0x96),
                dark3: Color::rgb(0x00, 0x2F, 0x6C),
            };
        }
        Self {
            base,
            light1: base.shade(0.08),
            light2: base.shade(0.18),
            light3: base.shade(0.28),
            dark1: base.shade(-0.08),
            dark2: base.shade(-0.16),
            dark3: base.shade(-0.26),
        }
    }
}

/// WinUI brush tokens, field names = resource names lower-cased.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub text_fill_color_primary: Color,
    pub text_fill_color_secondary: Color,
    pub text_fill_color_tertiary: Color,
    pub text_fill_color_disabled: Color,
    pub accent_text_fill_color_primary: Color,
    pub accent_text_fill_color_secondary: Color,
    pub text_on_accent_fill_color_primary: Color,
    pub text_on_accent_fill_color_secondary: Color,
    pub text_on_accent_fill_color_disabled: Color,
    pub accent_fill_color_default: Color,
    pub accent_fill_color_secondary: Color,
    pub accent_fill_color_tertiary: Color,
    pub accent_fill_color_disabled: Color,
    pub control_fill_color_default: Color,
    pub control_fill_color_secondary: Color,
    pub control_fill_color_tertiary: Color,
    pub control_fill_color_disabled: Color,
    pub control_fill_color_input_active: Color,
    pub control_strong_fill_color_default: Color,
    pub control_strong_fill_color_disabled: Color,
    pub control_solid_fill_color_default: Color,
    pub control_alt_fill_color_secondary: Color,
    pub control_alt_fill_color_tertiary: Color,
    pub control_alt_fill_color_quarternary: Color,
    pub subtle_fill_color_transparent: Color,
    pub subtle_fill_color_secondary: Color,
    pub subtle_fill_color_tertiary: Color,
    pub control_stroke_color_default: Color,
    pub control_stroke_color_secondary: Color,
    pub control_stroke_color_on_accent_default: Color,
    pub control_stroke_color_on_accent_secondary: Color,
    pub control_strong_stroke_color_default: Color,
    pub control_strong_stroke_color_disabled: Color,
    pub card_stroke_color_default: Color,
    pub surface_stroke_color_default: Color,
    pub surface_stroke_color_flyout: Color,
    pub divider_stroke_color_default: Color,
    pub focus_stroke_color_outer: Color,
    pub focus_stroke_color_inner: Color,
    pub card_background_fill_color_default: Color,
    pub card_background_fill_color_secondary: Color,
    pub layer_fill_color_default: Color,
    pub layer_fill_color_alt: Color,
    pub layer_on_mica_base_alt_fill_color_default: Color,
    pub solid_background_fill_color_base: Color,
    pub solid_background_fill_color_secondary: Color,
    pub solid_background_fill_color_tertiary: Color,
    pub solid_background_fill_color_quarternary: Color,
    pub smoke_fill_color_default: Color,
    pub system_fill_color_success: Color,
    pub system_fill_color_caution: Color,
    pub system_fill_color_critical: Color,
    pub system_fill_color_attention: Color,
    pub system_fill_color_solid_neutral: Color,
    pub system_fill_color_attention_background: Color,
    pub system_fill_color_success_background: Color,
    pub system_fill_color_caution_background: Color,
    pub system_fill_color_critical_background: Color,
    /// AcrylicBackgroundFillColorDefault: tint + tint opacity + luminosity opacity.
    pub acrylic_background_tint: Color,
    pub acrylic_tint_opacity: f32,
    pub acrylic_luminosity_opacity: f32,
}

impl Tokens {
    fn resolve(mode: ThemeMode, a: &AccentRamp) -> Self {
        let c = Color::argb;
        match mode {
            ThemeMode::Light => Tokens {
                text_fill_color_primary: c(0xE4000000),
                text_fill_color_secondary: c(0x9E000000),
                text_fill_color_tertiary: c(0x72000000),
                text_fill_color_disabled: c(0x5C000000),
                accent_text_fill_color_primary: a.dark2,
                accent_text_fill_color_secondary: a.dark3,
                text_on_accent_fill_color_primary: c(0xFFFFFFFF),
                text_on_accent_fill_color_secondary: c(0xB3FFFFFF),
                text_on_accent_fill_color_disabled: c(0xFFFFFFFF),
                accent_fill_color_default: a.dark1,
                accent_fill_color_secondary: a.dark1.with_alpha(0.9),
                accent_fill_color_tertiary: a.dark1.with_alpha(0.8),
                accent_fill_color_disabled: c(0x37000000),
                control_fill_color_default: c(0xB3FFFFFF),
                control_fill_color_secondary: c(0x80F9F9F9),
                control_fill_color_tertiary: c(0x4DF9F9F9),
                control_fill_color_disabled: c(0x4DF9F9F9),
                control_fill_color_input_active: c(0xFFFFFFFF),
                control_strong_fill_color_default: c(0x72000000),
                control_strong_fill_color_disabled: c(0x51000000),
                control_solid_fill_color_default: c(0xFFFFFFFF),
                control_alt_fill_color_secondary: c(0x06000000),
                control_alt_fill_color_tertiary: c(0x0F000000),
                control_alt_fill_color_quarternary: c(0x18000000),
                subtle_fill_color_transparent: c(0x00FFFFFF),
                subtle_fill_color_secondary: c(0x09000000),
                subtle_fill_color_tertiary: c(0x06000000),
                control_stroke_color_default: c(0x0F000000),
                control_stroke_color_secondary: c(0x29000000),
                control_stroke_color_on_accent_default: c(0x14FFFFFF),
                control_stroke_color_on_accent_secondary: c(0x66000000),
                control_strong_stroke_color_default: c(0x72000000),
                control_strong_stroke_color_disabled: c(0x37000000),
                card_stroke_color_default: c(0x0F000000),
                surface_stroke_color_default: c(0x66757575),
                surface_stroke_color_flyout: c(0x0F000000),
                divider_stroke_color_default: c(0x0F000000),
                focus_stroke_color_outer: c(0xE4000000),
                focus_stroke_color_inner: c(0xB3FFFFFF),
                card_background_fill_color_default: c(0xB3FFFFFF),
                card_background_fill_color_secondary: c(0x80F6F6F6),
                layer_fill_color_default: c(0x80FFFFFF),
                layer_fill_color_alt: c(0xFFFFFFFF),
                layer_on_mica_base_alt_fill_color_default: c(0xB3FFFFFF),
                solid_background_fill_color_base: c(0xFFF3F3F3),
                solid_background_fill_color_secondary: c(0xFFEEEEEE),
                solid_background_fill_color_tertiary: c(0xFFF9F9F9),
                solid_background_fill_color_quarternary: c(0xFFFFFFFF),
                smoke_fill_color_default: c(0x4D000000),
                system_fill_color_success: c(0xFF0F7B0F),
                system_fill_color_caution: c(0xFF9D5D00),
                system_fill_color_critical: c(0xFFC42B1C),
                system_fill_color_attention: a.dark1,
                system_fill_color_solid_neutral: c(0xFF8A8A8A),
                system_fill_color_attention_background: c(0x80F6F6F6),
                system_fill_color_success_background: c(0xFFDFF6DD),
                system_fill_color_caution_background: c(0xFFFFF4CE),
                system_fill_color_critical_background: c(0xFFFDE7E9),
                acrylic_background_tint: c(0xFFFCFCFC),
                acrylic_tint_opacity: 0.0,
                acrylic_luminosity_opacity: 0.85,
            },
            ThemeMode::Dark => Tokens {
                text_fill_color_primary: c(0xFFFFFFFF),
                text_fill_color_secondary: c(0xC5FFFFFF),
                text_fill_color_tertiary: c(0x87FFFFFF),
                text_fill_color_disabled: c(0x5DFFFFFF),
                accent_text_fill_color_primary: a.light3,
                accent_text_fill_color_secondary: a.light3,
                text_on_accent_fill_color_primary: c(0xFF000000),
                text_on_accent_fill_color_secondary: c(0x80000000),
                text_on_accent_fill_color_disabled: c(0x87FFFFFF),
                accent_fill_color_default: a.light2,
                accent_fill_color_secondary: a.light2.with_alpha(0.9),
                accent_fill_color_tertiary: a.light2.with_alpha(0.8),
                accent_fill_color_disabled: c(0x28FFFFFF),
                control_fill_color_default: c(0x0FFFFFFF),
                control_fill_color_secondary: c(0x15FFFFFF),
                control_fill_color_tertiary: c(0x08FFFFFF),
                control_fill_color_disabled: c(0x0BFFFFFF),
                control_fill_color_input_active: c(0xB31E1E1E),
                control_strong_fill_color_default: c(0x8BFFFFFF),
                control_strong_fill_color_disabled: c(0x3FFFFFFF),
                control_solid_fill_color_default: c(0xFF454545),
                control_alt_fill_color_secondary: c(0x19000000),
                control_alt_fill_color_tertiary: c(0x0BFFFFFF),
                control_alt_fill_color_quarternary: c(0x12FFFFFF),
                subtle_fill_color_transparent: c(0x00FFFFFF),
                subtle_fill_color_secondary: c(0x0FFFFFFF),
                subtle_fill_color_tertiary: c(0x0AFFFFFF),
                control_stroke_color_default: c(0x12FFFFFF),
                control_stroke_color_secondary: c(0x18FFFFFF),
                control_stroke_color_on_accent_default: c(0x14FFFFFF),
                control_stroke_color_on_accent_secondary: c(0x23000000),
                control_strong_stroke_color_default: c(0x8BFFFFFF),
                control_strong_stroke_color_disabled: c(0x28FFFFFF),
                card_stroke_color_default: c(0x19000000),
                surface_stroke_color_default: c(0x66757575),
                surface_stroke_color_flyout: c(0x33000000),
                divider_stroke_color_default: c(0x15FFFFFF),
                focus_stroke_color_outer: c(0xFFFFFFFF),
                focus_stroke_color_inner: c(0xB3000000),
                card_background_fill_color_default: c(0x0DFFFFFF),
                card_background_fill_color_secondary: c(0x08FFFFFF),
                layer_fill_color_default: c(0x4C3A3A3A),
                layer_fill_color_alt: c(0x0DFFFFFF),
                layer_on_mica_base_alt_fill_color_default: c(0x733A3A3A),
                solid_background_fill_color_base: c(0xFF202020),
                solid_background_fill_color_secondary: c(0xFF1C1C1C),
                solid_background_fill_color_tertiary: c(0xFF282828),
                solid_background_fill_color_quarternary: c(0xFF2C2C2C),
                smoke_fill_color_default: c(0x4D000000),
                system_fill_color_success: c(0xFF6CCB5F),
                system_fill_color_caution: c(0xFFFCE100),
                system_fill_color_critical: c(0xFFFF99A4),
                system_fill_color_attention: a.light2,
                system_fill_color_solid_neutral: c(0xFF9D9D9D),
                system_fill_color_attention_background: c(0x08FFFFFF),
                system_fill_color_success_background: c(0xFF393D1B),
                system_fill_color_caution_background: c(0xFF433519),
                system_fill_color_critical_background: c(0xFF442726),
                acrylic_background_tint: c(0xFF2C2C2C),
                acrylic_tint_opacity: 0.15,
                acrylic_luminosity_opacity: 0.96,
            },
        }
    }
}

/// Fluent type ramp (WinUI `*TextBlockStyle`).
#[derive(Clone, Copy, Debug)]
pub struct TextStyle {
    pub size: f32,
    pub weight: u16,
    pub line_height: f32,
}

impl TextStyle {
    pub fn rune(self) -> rune::tok::TextStyle {
        let mut s = rune::tok::TextStyle::new(self.size, self.weight as f32);
        s.line_height = self.line_height;
        s
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TypeRamp {
    pub caption: TextStyle,
    pub body: TextStyle,
    pub body_strong: TextStyle,
    pub body_large: TextStyle,
    pub subtitle: TextStyle,
    pub title: TextStyle,
    pub title_large: TextStyle,
    pub display: TextStyle,
}

pub const TYPE_RAMP: TypeRamp = TypeRamp {
    caption: TextStyle { size: 12.0, weight: 400, line_height: 16.0 },
    body: TextStyle { size: 14.0, weight: 400, line_height: 20.0 },
    body_strong: TextStyle { size: 14.0, weight: 600, line_height: 20.0 },
    body_large: TextStyle { size: 18.0, weight: 400, line_height: 24.0 },
    subtitle: TextStyle { size: 20.0, weight: 600, line_height: 28.0 },
    title: TextStyle { size: 28.0, weight: 600, line_height: 36.0 },
    title_large: TextStyle { size: 40.0, weight: 600, line_height: 52.0 },
    display: TextStyle { size: 68.0, weight: 600, line_height: 92.0 },
};

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    mode: ThemeMode,
    accent: AccentRamp,
    tokens: Tokens,
}

impl Theme {
    pub fn new(mode: ThemeMode, accent: Color) -> Self {
        let accent = AccentRamp::new(accent);
        Self { mode, accent, tokens: Tokens::resolve(mode, &accent) }
    }
    pub fn light() -> Self {
        Self::new(ThemeMode::Light, AccentRamp::DEFAULT_ACCENT)
    }
    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark, AccentRamp::DEFAULT_ACCENT)
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
    pub fn with_accent(&self, accent: Color) -> Self {
        Self::new(self.mode, accent)
    }
    pub fn type_ramp(&self) -> &TypeRamp {
        &TYPE_RAMP
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
    /// NavigationView `OpenPaneLength` as the Gallery sets it.
    pub const NAV_PANE_EXPANDED: f32 = 320.0;
    /// `CompactPaneLength`.
    pub const NAV_PANE_COMPACT: f32 = 48.0;
    pub const NAV_ITEM_HEIGHT: f32 = 36.0;
    pub const TITLE_BAR_HEIGHT: f32 = 48.0;
}
