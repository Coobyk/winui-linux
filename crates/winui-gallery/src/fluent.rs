//! Theme → Rune glue plus local fallbacks for the winui-materials surface
//! (`mica`, `acrylic`, `shadow`, noise tile). Swap for the real crate once it lands.

use rune::{El, FillRule, Pixels, Shape, TextStyle, canvas, el};
use std::sync::OnceLock;
use winui_icons::{Glyph, Symbol, VIEWBOX};
use winui_theme::{LinearGradient, TextStyle as Ts, Theme, fonts};

/// `winui_theme::Color` (sRGB) → `rune::Color` (linear sRGB, straight alpha).
pub fn c(x: winui_theme::Color) -> rune::Color {
    rune::Color {
        r: decode(x.r),
        g: decode(x.g),
        b: decode(x.b),
        a: x.a,
    }
}

fn decode(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// A ramp entry as a Rune text style.
pub fn font(s: Ts) -> TextStyle {
    let mut t = TextStyle::new(s.size, s.weight as f32);
    t.line_height = s.line_height;
    t
}

/// The CSS family list the app registers as Rune's sans slot.
pub fn families() -> &'static str {
    static F: OnceLock<String> = OnceLock::new();
    F.get_or_init(|| {
        let mut v = vec![
            format!("\"{}\"", fonts::TEXT),
            format!("\"{}\"", fonts::FAMILY),
        ];
        v.extend(fonts::FALLBACKS.iter().map(|f| {
            if f.contains(' ') {
                format!("\"{f}\"")
            } else {
                f.to_string()
            }
        }));
        v.join(", ")
    })
}

/// A Fluent System Icon drawn from its vendored path, `size` points square.
pub fn icon(sym: Symbol, size: f32, color: winui_theme::Color) -> El {
    let rule = match sym.fill_rule() {
        winui_icons::FillRule::EvenOdd => FillRule::EvenOdd,
        winui_icons::FillRule::NonZero => FillRule::NonZero,
    };
    let Glyph::Path(d) = sym.glyph() else {
        return el().size(size, size);
    };
    canvas(
        Shape::new()
            .fill_svg_with(d, rule, c(color))
            .view_box(VIEWBOX, VIEWBOX),
    )
    .size(size, size)
}

/// Apply a `ControlElevationBorderBrush`-style gradient as a 1px border.
pub fn elevation_border(e: El, g: &LinearGradient, height: f32) -> El {
    e.border_gradient(1.0, c(g.top(height)), c(g.bottom(height)))
}

/// Deterministic 64×64 greyscale noise tile (xorshift), alpha in the low range.
pub fn noise_tile() -> &'static Pixels {
    static TILE: OnceLock<Pixels> = OnceLock::new();
    TILE.get_or_init(|| {
        const N: u32 = 64;
        let mut s: u32 = 0x9E37_79B9;
        let mut rgba = Vec::with_capacity((N * N * 4) as usize);
        for _ in 0..N * N {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            let v = (s >> 24) as u8;
            rgba.extend_from_slice(&[v, v, v, 255]);
        }
        let p = Pixels::new();
        p.set(N, N, rgba);
        p
    })
}

/// Mica (base): opaque `SolidBackgroundFillColorBase` + faint noise. No wallpaper tint.
pub fn mica(theme: &Theme) -> El {
    let base = theme.tokens().solid_background_fill_color_base;
    el().w_full().h_full().fill(c(base)).child(
        el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .pixels_tile(noise_tile(), [64.0, 64.0])
            .opacity(0.02),
    )
}

/// Acrylic panel: tint colour at `tint_opacity` over the luminosity layer, plus 2% noise.
/// Defaults match `AcrylicBackgroundFillColorDefaultBrush` when called via [`acrylic_default`].
pub fn acrylic(
    tint: winui_theme::Color,
    tint_opacity: f32,
    luminosity: winui_theme::Color,
    luminosity_opacity: f32,
) -> El {
    let lum = luminosity.with_alpha(luminosity_opacity);
    let top = tint.with_alpha(tint_opacity).over(lum);
    el().fill(c(top)).child(
        el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .pixels_tile(noise_tile(), [64.0, 64.0])
            .opacity(0.02),
    )
}

/// `AcrylicBackgroundFillColorDefaultBrush` (Common_themeresources_any.xaml):
/// light tint #FCFCFC @0.0 / lum 0.85, dark tint #2C2C2C @0.15 / lum 0.96.
pub fn acrylic_default(theme: &Theme) -> El {
    if theme.is_dark() {
        acrylic(
            winui_theme::Color::rgb(0x2C, 0x2C, 0x2C),
            0.15,
            winui_theme::Color::rgb(0x2C, 0x2C, 0x2C),
            0.96,
        )
    } else {
        acrylic(
            winui_theme::Color::rgb(0xFC, 0xFC, 0xFC),
            0.0,
            winui_theme::Color::rgb(0xFC, 0xFC, 0xFC),
            0.85,
        )
    }
}

/// Fluent elevation: key shadow at `depth` (ThemeShadow approximation: y = depth/2, blur = depth).
pub fn shadow(e: El, theme: &Theme, depth: f32) -> El {
    let a = if theme.is_dark() { 0.37 } else { 0.14 };
    e.shadow(
        depth / 2.0,
        depth,
        rune::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a,
        },
    )
}
