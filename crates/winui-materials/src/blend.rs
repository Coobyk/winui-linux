//! W3C Compositing non-separable blend modes, as used by Win2D's `BlendEffect`
//! (the Acrylic and Mica recipes blend with `Luminosity` and `Color`).

use winui_theme::Color;

/// Luma `0.3 R + 0.59 G + 0.11 B` (W3C Compositing Level 1 `Lum`).
pub fn lum(c: Color) -> f32 {
    0.3 * c.r + 0.59 * c.g + 0.11 * c.b
}

fn clip_color(c: Color) -> Color {
    let l = lum(c);
    let n = c.r.min(c.g).min(c.b);
    let x = c.r.max(c.g).max(c.b);
    let mut ch = [c.r, c.g, c.b];
    for v in &mut ch {
        if n < 0.0 {
            *v = l + (*v - l) * l / (l - n);
        }
        if x > 1.0 {
            *v = l + (*v - l) * (1.0 - l) / (x - l);
        }
    }
    Color {
        r: ch[0],
        g: ch[1],
        b: ch[2],
        a: c.a,
    }
}

/// `SetLum(c, l)`: `c`'s hue and saturation at luma `l`.
pub fn set_lum(c: Color, l: f32) -> Color {
    let d = l - lum(c);
    clip_color(Color {
        r: c.r + d,
        g: c.g + d,
        b: c.b + d,
        a: c.a,
    })
}

/// Source-over with a blend function `B(cb, cs)` onto an opaque backdrop.
fn composite(backdrop: Color, src: Color, mixed: Color) -> Color {
    let a = src.a;
    let m = |b: f32, x: f32| b * (1.0 - a) + x * a;
    Color {
        r: m(backdrop.r, mixed.r),
        g: m(backdrop.g, mixed.g),
        b: m(backdrop.b, mixed.b),
        a: 1.0,
    }
}

/// `Luminosity` blend of `src` over an opaque `backdrop`: backdrop hue/saturation, source luma.
pub fn blend_luminosity(backdrop: Color, src: Color) -> Color {
    composite(backdrop, src, set_lum(backdrop, lum(src)))
}

/// `Color` blend of `src` over an opaque `backdrop`: source hue/saturation, backdrop luma.
pub fn blend_color(backdrop: Color, src: Color) -> Color {
    composite(backdrop, src, set_lum(src, lum(backdrop)))
}
