//! sRGBA colour.

use core::fmt;

/// One sRGBA colour, components in `0.0..=1.0`, straight (non-premultiplied) alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0.0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    /// Opaque colour from 8-bit channels.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// 8-bit channels plus a `0.0..=1.0` alpha.
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    /// XAML-style `0xAARRGGBB`.
    pub const fn from_argb(argb: u32) -> Self {
        let a = (argb >> 24) as u8;
        Self::rgba(
            (argb >> 16) as u8,
            (argb >> 8) as u8,
            argb as u8,
            a as f32 / 255.0,
        )
    }

    /// Parse `#RGB`, `#RRGGBB` or XAML-style `#AARRGGBB` (leading `#` optional).
    pub fn from_hex(s: &str) -> Option<Self> {
        let h = s.strip_prefix('#').unwrap_or(s);
        if !h.is_ascii() {
            return None;
        }
        let v = u32::from_str_radix(h, 16).ok()?;
        match h.len() {
            3 => {
                let n = |x: u32| ((x & 0xF) * 0x11) as u8;
                Some(Self::rgb(n(v >> 8), n(v >> 4), n(v)))
            }
            6 => Some(Self::from_argb(0xFF00_0000 | v)),
            8 => Some(Self::from_argb(v)),
            _ => None,
        }
    }

    /// Same colour, alpha replaced.
    pub const fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    /// Same colour, alpha multiplied (XAML `Brush.Opacity`).
    pub const fn with_opacity(self, opacity: f32) -> Self {
        Self {
            a: self.a * opacity,
            ..self
        }
    }

    /// Channels rounded to 8 bits: `[r, g, b, a]`.
    pub fn to_rgba8(self) -> [u8; 4] {
        let q = |c: f32| (c.clamp(0.0, 1.0) * 255.0).round() as u8;
        [q(self.r), q(self.g), q(self.b), q(self.a)]
    }

    /// Packed `0xAARRGGBB`.
    pub fn to_argb(self) -> u32 {
        let [r, g, b, a] = self.to_rgba8();
        u32::from_be_bytes([a, r, g, b])
    }

    /// Component-wise linear interpolation in sRGB space.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let m = |a: f32, b: f32| a + (b - a) * t;
        Self {
            r: m(self.r, other.r),
            g: m(self.g, other.g),
            b: m(self.b, other.b),
            a: m(self.a, other.a),
        }
    }

    /// Source-over composite of `self` onto `dst` (sRGB space, as XAML/D2D blend).
    pub fn over(self, dst: Self) -> Self {
        let a = self.a + dst.a * (1.0 - self.a);
        if a <= 0.0 {
            return Self::TRANSPARENT;
        }
        let c = |s: f32, d: f32| (s * self.a + d * dst.a * (1.0 - self.a)) / a;
        Self {
            r: c(self.r, dst.r),
            g: c(self.g, dst.g),
            b: c(self.b, dst.b),
            a,
        }
    }

    /// Relative luminance (WCAG 2.x), ignoring alpha.
    pub fn luminance(self) -> f32 {
        0.2126 * to_linear(self.r) + 0.7152 * to_linear(self.g) + 0.0722 * to_linear(self.b)
    }

    /// WCAG contrast ratio between two opaque colours (`1.0..=21.0`).
    pub fn contrast_ratio(self, other: Self) -> f32 {
        let (a, b) = (self.luminance() + 0.05, other.luminance() + 0.05);
        if a > b { a / b } else { b / a }
    }

    /// OKLab `[L, a, b]` of the colour, ignoring alpha.
    pub fn to_oklab(self) -> [f32; 3] {
        let (r, g, b) = (to_linear(self.r), to_linear(self.g), to_linear(self.b));
        let l = (0.412_221_47 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
        let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
        let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
        [
            0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
            1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
            0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
        ]
    }

    /// Colour from OKLab, unclamped (may be out of sRGB gamut; see [`Self::in_gamut`]).
    pub fn from_oklab([l, a, b]: [f32; 3], alpha: f32) -> Self {
        let l_ = (l + 0.396_337_78 * a + 0.215_803_76 * b).powi(3);
        let m_ = (l - 0.105_561_346 * a - 0.063_854_17 * b).powi(3);
        let s_ = (l - 0.089_484_18 * a - 1.291_485_5 * b).powi(3);
        Self {
            r: from_linear(4.076_741_7 * l_ - 3.307_711_6 * m_ + 0.230_969_94 * s_),
            g: from_linear(-1.268_438 * l_ + 2.609_757_4 * m_ - 0.341_319_38 * s_),
            b: from_linear(-0.004_196_086_3 * l_ - 0.703_418_6 * m_ + 1.707_614_7 * s_),
            a: alpha,
        }
    }

    /// Whether every channel lies in `0.0..=1.0` (with a small tolerance).
    pub fn in_gamut(self) -> bool {
        const E: f32 = 1e-4;
        [self.r, self.g, self.b]
            .iter()
            .all(|c| (-E..=1.0 + E).contains(c))
    }

    /// Channels clamped to `0.0..=1.0`.
    pub fn clamped(self) -> Self {
        let c = |v: f32| v.clamp(0.0, 1.0);
        Self {
            r: c(self.r),
            g: c(self.g),
            b: c(self.b),
            a: c(self.a),
        }
    }

    /// Equal after 8-bit quantisation.
    pub fn eq_rgba8(self, other: Self) -> bool {
        self.to_rgba8() == other.to_rgba8()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

/// `#AARRGGBB`, XAML style.
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:08X}", self.to_argb())
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self::rgba(r, g, b, a as f32 / 255.0)
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

fn to_linear(c: f32) -> f32 {
    if c <= 0.040_45 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn from_linear(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.signum() * c.abs().powf(1.0 / 2.4) - 0.055
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_print() {
        assert_eq!(
            Color::from_hex("#0078D4"),
            Some(Color::rgb(0x00, 0x78, 0xD4))
        );
        assert_eq!(
            Color::from_hex("C5FFFFFF").unwrap().to_rgba8(),
            [255, 255, 255, 0xC5]
        );
        assert_eq!(Color::from_hex("#fff"), Some(Color::WHITE));
        assert_eq!(Color::from_hex("#12345"), None);
        assert_eq!(Color::from_argb(0x5DFFFFFF).to_string(), "#5DFFFFFF");
    }

    #[test]
    fn oklab_roundtrip() {
        for hex in [
            "#0078D4", "#4CC2FF", "#001A68", "#FCE100", "#202020", "#FFFFFF", "#000000",
        ] {
            let c = Color::from_hex(hex).unwrap();
            let back = Color::from_oklab(c.to_oklab(), 1.0);
            assert!(c.eq_rgba8(back), "{hex} -> {back}");
        }
    }

    #[test]
    fn compositing() {
        let c = Color::from_argb(0x0FFFFFFF).over(Color::from_argb(0xFF202020));
        assert_eq!(c.a, 1.0);
        assert_eq!(c.to_rgba8(), [0x2D, 0x2D, 0x2D, 0xFF]);
        assert!((Color::BLACK.contrast_ratio(Color::WHITE) - 21.0).abs() < 1e-3);
    }
}
