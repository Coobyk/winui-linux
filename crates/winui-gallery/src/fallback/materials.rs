//! Fallback for the `winui-materials` contract: Mica, Acrylic, elevation shadows, noise.

use super::theme::{Color, Theme};
use rune::{col, El, Pixels, Shadow, el};

const NOISE_SIDE: u32 = 64;

/// Deterministic 64×64 monochrome noise tile (Acrylic's `NoiseAsset_256x256` stand-in).
pub fn noise_rgba() -> Vec<u8> {
    let mut out = Vec::with_capacity((NOISE_SIDE * NOISE_SIDE * 4) as usize);
    let mut x: u32 = 0x9E37_79B9;
    for _ in 0..NOISE_SIDE * NOISE_SIDE {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let v = (x & 0xFF) as u8;
        out.extend_from_slice(&[v, v, v, 0xFF]);
    }
    out
}

thread_local! {
    static NOISE: Pixels = {
        let p = Pixels::new();
        p.set(NOISE_SIDE, NOISE_SIDE, noise_rgba());
        p
    };
}

/// The shared noise tile.
pub fn noise() -> Pixels {
    NOISE.with(|p| p.clone())
}

/// A full-size noise layer at `opacity`, drawn above a material's tint.
pub fn noise_layer(opacity: f32) -> El {
    el().absolute(0.0, 0.0).w_full().h_full().pixels_tile(&noise(), [NOISE_SIDE as f32, NOISE_SIDE as f32]).opacity(opacity)
}

/// Mica window base: opaque theme tint (`SolidBackgroundFillColorBase`) + faint noise.
pub fn mica(theme: &Theme) -> El {
    let base = theme.tokens().solid_background_fill_color_base;
    el().w_full().h_full().fill(base.into()).child(noise_layer(0.02))
}

/// Acrylic tint resolved per Fluent: luminosity layer under a tint layer.
pub fn acrylic_color(theme: &Theme, tint: Color, tint_opacity: f32, luminosity_opacity: f32) -> Color {
    let lum: rune::Color = theme.tokens().solid_background_fill_color_base.into();
    let tint_c: rune::Color = tint.into();
    let mixed = lum.mix(tint_c, tint_opacity.clamp(0.0, 1.0));
    Color::from(mixed).with_alpha(luminosity_opacity.clamp(0.0, 1.0))
}

/// Translucent noise-blended panel. Add children to the returned element.
pub fn acrylic(theme: &Theme, tint: Color, tint_opacity: f32, luminosity_opacity: f32) -> El {
    let c = acrylic_color(theme, tint, tint_opacity, luminosity_opacity);
    col().fill(c.into()).backdrop_blur(30.0).child(noise_layer(0.02))
}

/// The theme's default acrylic (AcrylicBackgroundFillColorDefaultBrush).
pub fn acrylic_default(theme: &Theme) -> El {
    let t = theme.tokens();
    acrylic(theme, t.acrylic_background_tint, t.acrylic_tint_opacity, t.acrylic_luminosity_opacity)
}

/// Fluent elevation: (ambient, key) shadows for depth 2/4/8/16/32/64/128.
pub fn shadow(theme: &Theme, depth: f32) -> [Shadow; 2] {
    let dark = theme.is_dark();
    let (ambient_a, key_a) = if dark { (0.26, 0.37) } else { (0.11, 0.13) };
    let ambient = Shadow { x: 0.0, y: 0.0, blur: depth / 2.0, spread: 0.0, inset: false, color: rune::Color::BLACK.alpha(ambient_a) };
    let key = Shadow { x: 0.0, y: depth / 2.0, blur: depth, spread: 0.0, inset: false, color: rune::Color::BLACK.alpha(key_a) };
    [ambient, key]
}

/// Apply `shadow(depth)` to an element.
pub fn elevate(e: El, theme: &Theme, depth: f32) -> El {
    let [a, k] = shadow(theme, depth);
    e.box_shadow(a).box_shadow(k)
}

/// Reveal-style hover highlight for nav items / list rows (WinUI 3 uses a fill change).
pub fn reveal(e: El, theme: &Theme) -> El {
    let t = theme.tokens();
    e.hover_fill(t.subtle_fill_color_secondary.into()).press_fill(t.subtle_fill_color_tertiary.into())
}
