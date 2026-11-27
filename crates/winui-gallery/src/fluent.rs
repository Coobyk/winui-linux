//! Theme → Rune glue plus local fallbacks for the winui-materials surface
//! (`mica`, `acrylic`, `shadow`, noise tile). Swap for the real crate once it lands.

use rune::{El, FillRule, Shape, TextStyle, canvas, el};
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

/// Mica (base): opaque `SolidBackgroundFillColorBase` + faint noise. No wallpaper tint.
pub fn mica(theme: &Theme) -> El {
    winui_materials::mica(theme)
}

/// `AcrylicBackgroundFillColorDefaultBrush` (Common_themeresources_any.xaml):
/// light tint #FCFCFC @0.0 / lum 0.85, dark tint #2C2C2C @0.15 / lum 0.96.
pub fn acrylic_default(theme: &Theme) -> El {
    winui_materials::nav_acrylic(theme)
}

/// Fluent elevation: key shadow at `depth` (ThemeShadow approximation: y = depth/2, blur = depth).
pub fn shadow(e: El, theme: &Theme, depth: f32) -> El {
    winui_materials::MaterialExt::theme_shadow(e, theme, depth)
}
