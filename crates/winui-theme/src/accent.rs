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
    /// Windows 11's default ("Blue") accent ramp as `UISettings` reports it for
    /// `#0078D4`. Sources per step are listed in the crate README.
    pub const DEFAULT: Self = Self {
        dark3: Color::from_argb(0xFF001A68),
        dark2: Color::from_argb(0xFF003E92),
        dark1: Color::from_argb(0xFF0067C0),
        base: Color::from_argb(0xFF0078D4),
        light1: Color::from_argb(0xFF0093F9),
        light2: Color::from_argb(0xFF60CDFF),
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
            from_lch_in_gamut(l, c, h)
        };
        Self {
            dark3: step(d.dark3),
            dark2: step(d.dark2),
            dark1: step(d.dark1),
            base: accent,
            light1: step(d.light1),
            light2: step(d.light2),
            light3: step(d.light3),
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
