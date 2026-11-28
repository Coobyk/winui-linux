//! Accent colour ramp (`SystemAccentColor` + Light1..3 / Dark1..3).

use crate::color::Color;

/// The seven `SystemAccentColor*` resources WinUI resolves accent brushes from.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccentRamp {
    pub dark3: Color,
    pub dark2: Color,
    pub dark1: Color,
    /// `SystemAccentColor`, the user-picked accent.
    pub base: Color,
    pub light1: Color,
    pub light2: Color,
    pub light3: Color,
}

impl AccentRamp {
    /// Windows 11's default ("Blue") accent ramp: the shell's default `AccentPalette`
    /// (`99EBFF 4CC2FF 0091F8 0078D4 0067C0 003E92 001A68`) as `UISettings` reports it.
    /// Matches the reference screenshots: dark-theme accent fills sample `#4CC2FF` (Light2),
    /// accent text `#99EBFF` (Light3); light-theme accent fills are `#0067C0` (Dark1).
    pub const DEFAULT: Self = Self {
        dark3: Color::from_argb(0xFF001A68),
        dark2: Color::from_argb(0xFF003E92),
        dark1: Color::from_argb(0xFF0067C0),
        base: Color::from_argb(0xFF0078D4),
        light1: Color::from_argb(0xFF0091F8),
        light2: Color::from_argb(0xFF4CC2FF),
        light3: Color::from_argb(0xFF99EBFF),
    };

    /// Fixed variant ramp WinUI's own test harness uses with an accent override
    /// (`SystemThemingInterop::GetSystemVariantAccentColor`); Windows 10 era blue.
    pub const WINUI_TEST_OVERRIDE: Self = Self {
        dark3: Color::from_argb(0xFF002642),
        dark2: Color::from_argb(0xFF004275),
        dark1: Color::from_argb(0xFF005A9E),
        base: Color::from_argb(0xFF0078D7),
        light1: Color::from_argb(0xFF429CE3),
        light2: Color::from_argb(0xFF76B9ED),
        light3: Color::from_argb(0xFFA6D8FF),
    };

    /// Derive a full ramp from one accent colour.
    ///
    /// Windows computes the ramp in the shell (not in WinUI) and the algorithm is
    /// not public. We transfer the default ramp's per-step offsets in OKLCH —
    /// lightness delta, chroma ratio, hue delta — onto `accent`, reducing chroma
    /// until each step fits sRGB. The default ramp's blue→cyan hue drift is
    /// faded out as the accent's hue moves away from blue, so e.g. purple
    /// accents stay purple. `derive(DEFAULT.base)` reproduces `DEFAULT`.
    pub fn derive(accent: Color) -> Self {
        let accent = accent.with_alpha(1.0);
        let d = Self::DEFAULT;
        if accent.eq_rgba8(d.base) {
            return d;
        }
        let base = lch(d.base);
        let a = lch(accent);
        let hue_weight = (a[2] - base[2]).cos().max(0.0) * (a[1] / base[1].max(1e-4)).min(1.0);
        let step = |reference: Color| {
            let r = lch(reference);
            let l = (a[0] + (r[0] - base[0])).clamp(0.0, 1.0);
            let c = if base[1] > 1e-4 {
                a[1] * r[1] / base[1]
            } else {
                a[1]
            };
            let h = a[2] + hue_weight * wrap(r[2] - base[2]);
            quantized(from_lch_in_gamut(l, c, h))
        };
        // Light mode puts white text on Dark1, dark mode black text on Light2
        // (`TextOnAccentFillColorPrimary`); keep both legible (WCAG AA 4.5:1).
        let dark1 = with_contrast(step(d.dark1), Color::rgb(255, 255, 255), true);
        let dark2 = not_lighter_than(step(d.dark2), dark1);
        let light2 = with_contrast(step(d.light2), Color::rgb(0, 0, 0), false);
        let light3 = not_darker_than(step(d.light3), light2);
        Self {
            dark3: not_lighter_than(step(d.dark3), dark2),
            dark2,
            dark1,
            base: accent,
            light1: step(d.light1),
            light2,
            light3,
        }
    }

    /// Steps darkest to lightest.
    pub fn steps(&self) -> [Color; 7] {
        [
            self.dark3,
            self.dark2,
            self.dark1,
            self.base,
            self.light1,
            self.light2,
            self.light3,
        ]
    }
}

impl Default for AccentRamp {
    fn default() -> Self {
        Self::DEFAULT
    }
}

const MIN_TEXT_CONTRAST: f32 = 4.5;

/// Move `c` in OKLCH lightness (darker or lighter) just far enough to reach
/// [`MIN_TEXT_CONTRAST`] against `text`, measured after 8-bit quantization.
fn with_contrast(c: Color, text: Color, darker: bool) -> Color {
    let c = quantized(c);
    if c.contrast_ratio(text) >= MIN_TEXT_CONTRAST {
        return c;
    }
    let [l, ch, h] = lch(c);
    let (mut lo, mut hi) = if darker { (0.0, l) } else { (l, 1.0) };
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        let ok = quantized(from_lch_in_gamut(mid, ch, h)).contrast_ratio(text) >= MIN_TEXT_CONTRAST;
        if ok == darker { lo = mid } else { hi = mid }
    }
    quantized(from_lch_in_gamut(if darker { lo } else { hi }, ch, h))
}

/// Snap to 8-bit so the contrast guarantee survives hex/ARGB export.
fn quantized(c: Color) -> Color {
    let [r, g, b, _] = c.to_rgba8();
    Color::rgb(r, g, b)
}

/// Exported steps are 8-bit, so ordering is enforced on quantized colours.
fn not_lighter_than(c: Color, limit: Color) -> Color {
    clamp_lightness(c, lch(limit)[0], true)
}

fn not_darker_than(c: Color, limit: Color) -> Color {
    clamp_lightness(c, lch(limit)[0], false)
}

fn clamp_lightness(c: Color, bound: f32, at_most: bool) -> Color {
    let [l, ch, h] = lch(c);
    let ok = |l: f32| if at_most { l <= bound } else { l >= bound };
    if ok(l) {
        return c;
    }
    let mut target = bound;
    let nudge = if at_most { -0.0005 } else { 0.0005 };
    loop {
        let q = quantized(from_lch_in_gamut(target, ch, h));
        if ok(lch(q)[0]) || !(0.0..=1.0).contains(&target) {
            return q;
        }
        target += nudge;
    }
}

fn wrap(dh: f32) -> f32 {
    use core::f32::consts::{PI, TAU};
    (dh + PI).rem_euclid(TAU) - PI
}

fn lch(c: Color) -> [f32; 3] {
    let [l, a, b] = c.to_oklab();
    [l, a.hypot(b), b.atan2(a)]
}

fn from_lch_in_gamut(l: f32, c: f32, h: f32) -> Color {
    let make = |c: f32| Color::from_oklab([l, c * h.cos(), c * h.sin()], 1.0);
    let full = make(c);
    if full.in_gamut() {
        return full.clamped();
    }
    let (mut lo, mut hi) = (0.0f32, c);
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        if make(mid).in_gamut() {
            lo = mid
        } else {
            hi = mid
        }
    }
    make(lo).clamped()
}
