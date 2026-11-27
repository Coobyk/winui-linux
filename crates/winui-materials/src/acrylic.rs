//! `Microsoft.UI.Xaml.Media.AcrylicBrush` (controls/dev/Materials/Acrylic/AcrylicBrush.cpp):
//! blurred backdrop → Luminosity blend → Color blend with the tint → 2% noise, source-over.

use rune::{El, el};
use winui_theme::{Color, Theme};

use crate::blend::{blend_color, blend_luminosity};
use crate::noise::{NOISE_LEVELS, NOISE_OPACITY, noise_layer};
use crate::rc;

/// Parameters of one AcrylicBrush. Fields mirror the XAML properties.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AcrylicBrush {
    pub tint_color: Color,
    pub tint_opacity: f32,
    /// `TintLuminosityOpacity`; `None` derives it from the tint like WinUI does.
    pub tint_luminosity_opacity: Option<f32>,
    /// Shown when acrylic is disabled (battery saver, policy) and the backdrop's opaque base.
    pub fallback_color: Color,
}

/// `AcrylicBrush::sc_blurRadius`: Gaussian standard deviation in DIPs.
pub const BLUR_RADIUS: f32 = 30.0;

fn to_byte(v: f32) -> f32 {
    (v.clamp(0.0, 1.0) * 255.0).round()
}

fn hsv(c: Color) -> (f32, f32, f32) {
    let max = c.r.max(c.g).max(c.b);
    let min = c.r.min(c.g).min(c.b);
    let d = max - min;
    let h = if d == 0.0 {
        0.0
    } else if max == c.r {
        60.0 * ((c.g - c.b) / d).rem_euclid(6.0)
    } else if max == c.g {
        60.0 * ((c.b - c.r) / d + 2.0)
    } else {
        60.0 * ((c.r - c.g) / d + 4.0)
    };
    (h, if max == 0.0 { 0.0 } else { d / max }, max)
}

fn from_hsv(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match (h / 60.0) as u32 % 6 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (r + m, g + m, b + m)
}

/// `GetTintOpacityModifier`: caps tint opacity at 45% for white, 90% for mid grey and 85% for
/// black, cancelled out as saturation rises.
pub fn tint_opacity_modifier(tint: Color) -> f32 {
    let (mid, white_max, mid_max, black_max) = (0.5f32, 0.45f32, 0.90f32, 0.85f32);
    let (_, s, v) = hsv(tint);
    if v == mid {
        return mid_max;
    }
    let (lowest, max_dev) = if v > mid {
        (white_max, 1.0 - mid)
    } else {
        (black_max, mid)
    };
    let mut suppression = mid_max - lowest;
    if s > 0.0 {
        suppression *= (1.0 - s * 2.0).max(0.0);
    }
    mid_max - suppression * ((v - mid).abs() / max_dev)
}

impl AcrylicBrush {
    pub fn new(
        tint_color: Color,
        tint_opacity: f32,
        tint_luminosity_opacity: Option<f32>,
        fallback_color: Color,
    ) -> Self {
        Self {
            tint_color,
            tint_opacity,
            tint_luminosity_opacity,
            fallback_color,
        }
    }

    /// `GetEffectiveTintColor`: the tint with its alpha scaled by `TintOpacity` (and by
    /// [`tint_opacity_modifier`] when no luminosity opacity is set), rounded to a byte.
    pub fn effective_tint_color(&self) -> Color {
        let t = self.tint_color;
        let k = match self.tint_luminosity_opacity {
            Some(_) => self.tint_opacity,
            None => self.tint_opacity * tint_opacity_modifier(t),
        };
        t.with_alpha((to_byte(t.a) * k).round() / 255.0)
    }

    /// `GetEffectiveLuminosityColor` / `GetLuminosityColor`.
    pub fn effective_luminosity_color(&self) -> Color {
        let t = self.tint_color;
        let tint_a = (to_byte(t.a) * self.tint_opacity).round() / 255.0;
        match self.tint_luminosity_opacity {
            Some(l) => t.with_alpha(l.clamp(0.0, 1.0)),
            None => {
                let (h, s, v) = hsv(t);
                let (r, g, b) = from_hsv(h, s, v.clamp(0.125, 0.965));
                let a = (tint_a * (1.03 - 0.15) + 0.15).min(1.0);
                Color { r, g, b, a }
            }
        }
    }

    /// The brush resolved over a (blurred) backdrop colour, exactly as the effect graph does it:
    /// backdrop over the opaque fallback → Luminosity blend → Color blend → noise source-over
    /// (noise taken at its mean grey, as a flat region averages it).
    pub fn composite_over(&self, backdrop: Color) -> Color {
        let tinted = self.composite_under_noise(backdrop);
        let mean = NOISE_LEVELS.iter().map(|&(v, p)| v as f32 * p).sum::<f32>() / 255.0;
        Color {
            r: mean,
            g: mean,
            b: mean,
            a: NOISE_OPACITY,
        }
        .over(tinted)
    }

    /// [`Self::composite_over`] without the noise layer. An opaque effective tint replaces the
    /// backdrop, blur and luminosity steps (`shouldBrushBeOpaque` in `AcrylicBrush.cpp`).
    pub fn composite_under_noise(&self, backdrop: Color) -> Color {
        let tint = self.effective_tint_color();
        if tint.a >= 1.0 {
            return tint;
        }
        let base = backdrop.over(self.fallback_color.with_alpha(1.0));
        let lum = blend_luminosity(base, self.effective_luminosity_color());
        blend_color(lum, tint)
    }

    /// The brush as a Rune element: `backdrop_blur(30)`, then the luminosity layer and the
    /// effective tint as fills, then the noise tile. Rune has no non-separable blend modes, so
    /// the Luminosity and Color steps draw source-over; for the neutral theme tints the result
    /// matches [`Self::composite_over`] to within a few levels.
    pub fn el(&self) -> El {
        self.el_rounded(0.0)
    }

    /// [`Self::el`] with corner radius `radius` applied to every layer.
    pub fn el_rounded(&self, radius: f32) -> El {
        el().backdrop_blur(BLUR_RADIUS)
            .radius(radius)
            .clip_rounded(radius)
            .child(full(radius).fill(rc(self.effective_luminosity_color())))
            .child(full(radius).fill(rc(self.effective_tint_color())))
            .child(noise_layer(radius))
    }

    /// `AcrylicBackgroundFillColorDefaultBrush` (Common/AcrylicBrush_themeresources.xaml).
    pub fn background_default(theme: &Theme) -> Self {
        let t = theme.tokens();
        if theme.is_dark() {
            Self::new(
                Color::rgb(0x2C, 0x2C, 0x2C),
                0.15,
                Some(0.96),
                Color::rgb(0x2C, 0x2C, 0x2C),
            )
        } else {
            Self::new(
                Color::rgb(0xFC, 0xFC, 0xFC),
                0.0,
                Some(0.85),
                t.solid_background_fill_color_tertiary,
            )
        }
    }

    /// `AcrylicBackgroundFillColorBaseBrush`: tint = SolidBackgroundFillColorBase,
    /// fallback = SolidBackgroundFillColorSecondary.
    pub fn background_base(theme: &Theme) -> Self {
        let t = theme.tokens();
        let (op, lum) = if theme.is_dark() {
            (0.5, 0.96)
        } else {
            (0.0, 0.9)
        };
        Self::new(
            t.solid_background_fill_color_base,
            op,
            Some(lum),
            t.solid_background_fill_color_secondary,
        )
    }

    /// `AccentAcrylicBackgroundFillColorDefaultBrush`: tracks the theme's accent ramp.
    pub fn accent_default(theme: &Theme) -> Self {
        let a = theme.accent();
        if theme.is_dark() {
            Self::new(a.dark1, 0.8, Some(0.8), a.dark1)
        } else {
            Self::new(a.light3, 0.8, Some(0.9), a.light3)
        }
    }

    /// `AccentAcrylicBackgroundFillColorBaseBrush`.
    pub fn accent_base(theme: &Theme) -> Self {
        let a = theme.accent();
        if theme.is_dark() {
            Self::new(a.dark2, 0.8, Some(0.8), a.dark2)
        } else {
            Self::new(a.light3, 0.8, Some(0.9), a.light3)
        }
    }
}

pub(crate) fn full(radius: f32) -> El {
    el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .radius(radius)
        .inert()
}

/// The theme's default Acrylic (`AcrylicBackgroundFillColorDefaultBrush`).
pub fn acrylic_brush(theme: &Theme) -> AcrylicBrush {
    AcrylicBrush::background_default(theme)
}

/// An Acrylic panel with an explicit tint. The fallback colour comes from the theme
/// (`SolidBackgroundFillColorBase`), so it flips with light/dark.
pub fn acrylic(
    theme: &Theme,
    tint: Color,
    tint_opacity: f32,
    luminosity_opacity: Option<f32>,
) -> El {
    AcrylicBrush::new(
        tint,
        tint_opacity,
        luminosity_opacity,
        theme.tokens().solid_background_fill_color_base,
    )
    .el()
}

/// [`acrylic_brush`] as an element.
pub fn acrylic_el(theme: &Theme) -> El {
    acrylic_brush(theme).el()
}
