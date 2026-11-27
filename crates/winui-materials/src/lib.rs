//! Mica, Acrylic, reveal, and elevation effects in Rune.
//! See docs/ARCHITECTURE.md for the contract.
//!
//! Every material reads its colours from [`winui_theme::Tokens`] so it tracks
//! theme and accent switches. `rune::Color` is linear-light sRGBA while
//! `winui_theme::Color` is display-encoded sRGBA — [`rc`] converts.

use rune::{Color, El, Pixels, Shadow, el};
use winui_theme::{Theme, Tokens};

/// `winui_theme::Color` (sRGBA) → `rune::Color` (linear sRGBA).
pub fn rc(c: winui_theme::Color) -> Color {
    let lin = |c: f32| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    Color {
        r: lin(c.r),
        g: lin(c.g),
        b: lin(c.b),
        a: c.a,
    }
}

/// `rune::Color` → `winui_theme::Color`.
pub fn tc(c: Color) -> winui_theme::Color {
    let enc = |c: f32| {
        if c <= 0.0031308 {
            c * 12.92
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    };
    winui_theme::Color {
        r: enc(c.r),
        g: enc(c.g),
        b: enc(c.b),
        a: c.a,
    }
}

/// Fetch the tokens off a theme, converted for direct use in element styling.
pub fn tokens(theme: &Theme) -> &Tokens {
    theme.tokens()
}

/// A deterministic 64×64 luminance-noise tile — the same one real Mica and
/// Acrylic blend under their tint. Alpha is low: this must read as film grain,
/// never as speckle.
pub fn noise_tile() -> Pixels {
    const N: u32 = 64;
    let mut rgba = vec![0u8; (N * N * 4) as usize];
    // xorshift32 with a fixed seed: stable across runs and platforms.
    let mut s: u32 = 0x9E3779B9;
    for px in rgba.chunks_mut(4) {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        let v = ((s >> 8) & 0xFF) as u8;
        px[0] = v;
        px[1] = v;
        px[2] = v;
        px[3] = v;
    }
    let p = Pixels::new();
    p.set(N, N, rgba);
    p
}

/// Mica: the window base. An opaque theme base colour with the faint noise
/// layer blended on top — real Mica samples the desktop wallpaper, but its
/// visible signature is base + grain, which is what we reproduce.
pub fn mica(theme: &Theme) -> El {
    let t = theme.tokens();
    el().key("mica")
        .w_full()
        .h_full()
        .fill(rc(t.solid_background_fill_color_base))
        .child(noise_overlay(0.02))
}

/// Mica Alt: the slightly lighter variant used under tab strips.
pub fn mica_alt(theme: &Theme) -> El {
    let t = theme.tokens();
    el().key("mica-alt")
        .w_full()
        .h_full()
        .fill(rc(t.solid_background_fill_color_base_alt))
        .child(noise_overlay(0.02))
}

/// In-app acrylic: a translucent tinted layer for nav panes and surfaces that
/// sit over mica. `tint_opacity` and `luminosity_opacity` follow the WinUI
/// acrylic recipe (default ≈ 60% tint over the luminosity layer).
pub fn acrylic(theme: &Theme, tint: winui_theme::Color, tint_opacity: f32, luminosity_opacity: f32) -> El {
    let t = theme.tokens();
    let base = rc(t.layer_on_acrylic_fill_color_default);
    let tint = rc(tint).alpha(tint_opacity);
    el().key("acrylic")
        .w_full()
        .h_full()
        .fill(base.alpha(base.a * luminosity_opacity))
        .child(el().w_full().h_full().fill(tint))
        .child(noise_overlay(0.02))
}

/// Navigation-pane acrylic: reads the theme's `LayerOnAcrylicFill` and applies
/// the standard recipe, so callers don't hand-pick a tint.
pub fn nav_acrylic(theme: &Theme) -> El {
    let t = theme.tokens();
    let c = rc(t.layer_on_acrylic_fill_color_default);
    el().key("nav-acrylic")
        .w_full()
        .h_full()
        .fill(c)
        .child(noise_overlay(0.02))
}

/// The noise layer shared by mica/acrylic: a tiled grain at `alpha` opacity.
pub fn noise_overlay(alpha: f32) -> El {
    el().w_full()
        .h_full()
        .pixels_tile(&noise_tile(), [64.0, 64.0])
        .opacity(alpha)
        .inert()
}

/// Reveal highlight: the hover/focus accent-edge Fluent shows on list rows and
/// NavigationView items. Rune's hover/press handles the interactive part —
/// this is the static "resting" border reveal draws around the item: a hairline
/// that fades at the top and bottom edges like the real spotlight gradient.
pub fn reveal_border(theme: &Theme, radius: f32) -> El {
    let t = theme.tokens();
    let stroke = rc(t.control_stroke_color_secondary);
    el().key("reveal")
        .w_full()
        .h_full()
        .radius(radius)
        .border_gradient(1.0, stroke, stroke.alpha(0.0))
        .inert()
}

/// Hover reveal fill: the soft light a row shows under the pointer. Pass to
/// `.hover_fill` on the item element.
pub fn reveal_fill(theme: &Theme) -> Color {
    rc(theme.tokens().subtle_fill_color_secondary)
}

/// Fluent elevation: WinUI's ThemeShadow is an ambient + key pair; Rune's
/// `box_shadow` takes one shadow, so `elevation` returns the key shadow and
/// `ambient` the matching soft fill. Depths follow the WinUI elevation ramp:
/// 2 (cards), 4, 8 (tooltips/menus), 16 (flyouts), 32, 64, 128 (dialogs).
pub fn elevation(theme: &Theme, depth: f32) -> Shadow {
    let dark = theme.is_dark();
    // WinUI key shadow: tight, offset down, ~19% black in light / ~37% in dark.
    let opacity = if dark { 0.37 } else { 0.19 };
    Shadow {
        x: 0.0,
        y: depth * 0.5,
        blur: depth * 1.25,
        spread: 0.0,
        color: Color::BLACK.alpha(opacity),
        inset: false,
    }
}

/// The ambient half of a Fluent shadow: no offset, very soft, faint. Apply on
/// a second element behind the card for the full ThemeShadow look.
pub fn ambient(theme: &Theme, depth: f32) -> Shadow {
    let dark = theme.is_dark();
    let opacity = if dark { 0.26 } else { 0.13 };
    Shadow {
        x: 0.0,
        y: 0.0,
        blur: depth * 2.5,
        spread: 0.0,
        color: Color::BLACK.alpha(opacity),
        inset: false,
    }
}

/// A card that carries the full Fluent ThemeShadow (ambient + key) at `depth`.
/// Wrap your content's element: `card_shadow(theme, 8.0, radius, content)`.
pub fn card_shadow(theme: &Theme, depth: f32, radius: f32, content: El) -> El {
    el().w_full()
        .h_full()
        .box_shadow(ambient(theme, depth))
        .child(content.box_shadow(elevation(theme, depth)).radius(radius))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_decodes_srgb() {
        // #0078D4 — the stock Windows accent.
        let c = rc(winui_theme::Color::rgb(0x00, 0x78, 0xD4));
        assert!((c.g - lin(0x78)).abs() < 1e-4);
        assert_eq!(c.a, 1.0);
        fn lin(c: u8) -> f32 {
            let c = c as f32 / 255.0;
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    #[test]
    fn noise_is_deterministic() {
        let a = noise_tile().rgba();
        let b = noise_tile().rgba();
        assert_eq!(a, b);
        assert_eq!(a.len(), 64 * 64 * 4);
        assert!(a.iter().any(|&v| v != 0));
    }

    #[test]
    fn elevation_scales_with_depth() {
        let th = Theme::light();
        assert!(elevation(&th, 16.0).blur > elevation(&th, 2.0).blur);
    }
}
