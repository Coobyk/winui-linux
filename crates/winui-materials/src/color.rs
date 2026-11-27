//! Conversions between theme colours (sRGB) and Rune colours (linear).

use crate::theme::Color;

/// Theme sRGB colour to Rune's linear colour.
pub fn to_rune(c: Color) -> rune::Color {
    let d = rune::color::srgb_decode;
    rune::Color {
        r: d(c.r),
        g: d(c.g),
        b: d(c.b),
        a: c.a,
    }
}

/// `c` with its alpha replaced.
pub fn with_alpha(c: Color, a: f32) -> Color {
    Color {
        a: a.clamp(0.0, 1.0),
        ..c
    }
}

/// Straight-alpha source-over of `top` onto `bottom`, in linear light.
pub fn over(top: rune::Color, bottom: rune::Color) -> rune::Color {
    let a = top.a + bottom.a * (1.0 - top.a);
    if a <= 0.0 {
        return rune::Color::TRANSPARENT;
    }
    let ch = |t: f32, b: f32| (t * top.a + b * bottom.a * (1.0 - top.a)) / a;
    rune::Color {
        r: ch(top.r, bottom.r),
        g: ch(top.g, bottom.g),
        b: ch(top.b, bottom.b),
        a,
    }
}
