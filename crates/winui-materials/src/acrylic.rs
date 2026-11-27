//! In-app Acrylic: blurred backdrop, luminosity layer, tint layer, noise.
//!
//! The opacity maths is a port of WinUI's `AcrylicBrush` (`GetEffectiveTintColor`,
//! `GetTintOpacityModifier`, `GetLuminosityColor`); presets are the `Acrylic*FillColor*Brush`
//! theme resources. Rune has no luminosity/colour blend modes, so the luminosity and tint layers
//! composite source-over onto the blurred backdrop.

use rune::{El, el};

use crate::color::{over, to_rune, with_alpha};
use crate::noise::{NOISE_OPACITY, noise_layer};
use crate::theme::{Color, Theme, ThemeMode};

/// Backdrop blur, `AcrylicBrush::sc_blurRadius`, as Gaussian sigma in points.
pub const ACRYLIC_BLUR: f32 = 30.0;

/// One Acrylic recipe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acrylic {
    pub tint: Color,
    pub tint_opacity: f32,
    /// `TintLuminosityOpacity`; `None` derives it from the tint as WinUI does.
    pub luminosity_opacity: Option<f32>,
    /// Solid colour drawn instead when `fallback` is on (no blur available, or disabled).
    pub fallback: Color,
    pub use_fallback: bool,
    pub blur: f32,
    pub noise_opacity: f32,
    /// Corner radius of the panel and its noise, so no clip is needed (a clip would cut the
    /// panel's own shadow).
    pub radius: f32,
}

impl Acrylic {
    pub fn new(tint: Color, tint_opacity: f32, luminosity_opacity: Option<f32>) -> Self {
        Self {
            tint,
            tint_opacity,
            luminosity_opacity,
            fallback: Color { a: 1.0, ..tint },
            use_fallback: false,
            blur: ACRYLIC_BLUR,
            noise_opacity: NOISE_OPACITY,
            radius: 0.0,
        }
    }

    /// `AcrylicInAppFillColorDefaultBrush` (flyouts, menus, nav pane).
    pub fn in_app_default(theme: &Theme) -> Self {
        match theme.mode() {
            ThemeMode::Light => Self::new(Color::argb(0xFFFCFCFC), 0.0, Some(0.85))
                .with_fallback(Color::argb(0xFFF9F9F9)),
            ThemeMode::Dark => Self::new(Color::argb(0xFF2C2C2C), 0.15, Some(0.96))
                .with_fallback(Color::argb(0xFF2C2C2C)),
        }
    }

    /// `AcrylicInAppFillColorBaseBrush`.
    pub fn in_app_base(theme: &Theme) -> Self {
        match theme.mode() {
            ThemeMode::Light => Self::new(Color::argb(0xFFF3F3F3), 0.0, Some(0.9))
                .with_fallback(Color::argb(0xFFEEEEEE)),
            ThemeMode::Dark => Self::new(Color::argb(0xFF202020), 0.5, Some(0.96))
                .with_fallback(Color::argb(0xFF1C1C1C)),
        }
    }

    /// `AccentAcrylicInAppFillColorDefaultBrush`, tinted by the theme's accent fill (tracks
    /// accent changes; WinUI uses `SystemAccentColorDark1`/`Light3` of the same ramp).
    pub fn accent_default(theme: &Theme) -> Self {
        let tint = theme.tokens().accent_fill_color_default;
        let lum = if theme.mode() == ThemeMode::Light {
            0.9
        } else {
            0.8
        };
        Self::new(tint, 0.8, Some(lum))
    }

    pub fn with_fallback(mut self, fallback: Color) -> Self {
        self.fallback = fallback;
        self
    }

    /// Draw the solid fallback instead of the blurred material.
    pub fn fallback(mut self, on: bool) -> Self {
        self.use_fallback = on;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.0);
        self
    }

    pub fn blur(mut self, sigma: f32) -> Self {
        self.blur = sigma.max(0.0);
        self
    }

    /// The tint layer colour (`GetEffectiveTintColor`).
    pub fn effective_tint(&self) -> Color {
        effective_tint_color(self.tint, self.tint_opacity, self.luminosity_opacity)
    }

    /// The luminosity layer colour (`GetEffectiveLuminosityColor`).
    pub fn effective_luminosity(&self) -> Color {
        let tint = with_alpha(
            self.tint,
            quantize(self.tint.a * self.tint_opacity.clamp(0.0, 1.0)),
        );
        luminosity_color(tint, self.luminosity_opacity)
    }

    /// Luminosity then tint, flattened into the one fill drawn over the blurred backdrop.
    pub fn layer_color(&self) -> rune::Color {
        over(
            to_rune(self.effective_tint()),
            to_rune(self.effective_luminosity()),
        )
    }

    /// The material as a Rune element; size it and add children like any `El`.
    pub fn el(&self) -> El {
        if self.use_fallback {
            return el().radius(self.radius).fill(to_rune(self.fallback));
        }
        el().radius(self.radius)
            .backdrop_blur(self.blur)
            .fill(self.layer_color())
            .child(noise_layer(self.noise_opacity).radius(self.radius))
    }
}

/// `acrylic(theme, tint, tint_opacity, luminosity_opacity)` from the contract. `tint: None` uses
/// the theme's `AcrylicInAppFillColorDefault` tint, so the panel follows theme switches.
pub fn acrylic(
    theme: &Theme,
    tint: Option<Color>,
    tint_opacity: f32,
    luminosity_opacity: Option<f32>,
) -> El {
    let base = Acrylic::in_app_default(theme);
    let tint = tint.unwrap_or(base.tint);
    Acrylic {
        tint,
        tint_opacity,
        luminosity_opacity,
        ..base
    }
    .el()
}

/// WinUI stores colour alpha as a byte and rounds at each step.
fn quantize(a: f32) -> f32 {
    (a.clamp(0.0, 1.0) * 255.0).round() / 255.0
}

pub fn effective_tint_color(
    tint: Color,
    tint_opacity: f32,
    luminosity_opacity: Option<f32>,
) -> Color {
    let op = tint_opacity.clamp(0.0, 1.0);
    let a = match luminosity_opacity {
        Some(_) => tint.a * op,
        None => tint.a * op * tint_opacity_modifier(tint),
    };
    with_alpha(tint, quantize(a))
}

/// Caps tint opacity by brightness: 45% at white, 90% at mid grey, 85% at black, relaxed as
/// saturation rises.
pub fn tint_opacity_modifier(tint: Color) -> f32 {
    const MID: f32 = 0.5;
    const WHITE_MAX: f32 = 0.45;
    const MID_MAX: f32 = 0.90;
    const BLACK_MAX: f32 = 0.85;
    let [_, s, v] = rgb_to_hsv(tint);
    if v == MID {
        return MID_MAX;
    }
    let (lowest, max_dev) = if v > MID {
        (WHITE_MAX, 1.0 - MID)
    } else {
        (BLACK_MAX, MID)
    };
    let mut suppression = MID_MAX - lowest;
    if s > 0.0 {
        suppression *= (1.0 - s * 2.0).max(0.0);
    }
    MID_MAX - suppression * ((v - MID).abs() / max_dev)
}

pub fn luminosity_color(tint: Color, luminosity_opacity: Option<f32>) -> Color {
    if let Some(l) = luminosity_opacity {
        return with_alpha(tint, l.clamp(0.0, 1.0));
    }
    let [h, s, v] = rgb_to_hsv(tint);
    let rgb = hsv_to_rgb([h, s, v.clamp(0.125, 0.965)]);
    let mapped = tint.a * (1.03 - 0.15) + 0.15;
    Color {
        r: rgb[0],
        g: rgb[1],
        b: rgb[2],
        a: mapped.min(1.0),
    }
}

/// `[hue in degrees, saturation, value]`.
pub fn rgb_to_hsv(c: Color) -> [f32; 3] {
    let max = c.r.max(c.g).max(c.b);
    let min = c.r.min(c.g).min(c.b);
    let d = max - min;
    let s = if max > 0.0 { d / max } else { 0.0 };
    let h = if d == 0.0 {
        0.0
    } else if max == c.r {
        60.0 * ((c.g - c.b) / d).rem_euclid(6.0)
    } else if max == c.g {
        60.0 * ((c.b - c.r) / d + 2.0)
    } else {
        60.0 * ((c.r - c.g) / d + 4.0)
    };
    [h, s, max]
}

pub fn hsv_to_rgb([h, s, v]: [f32; 3]) -> [f32; 3] {
    let c = v * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    [r + m, g + m, b + m]
}
