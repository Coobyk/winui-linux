//! Mica (Windows App SDK `MicaController`, surfaced in WinUI 3 as `MicaBackdrop`).
//!
//! Mica is the Acrylic recipe run over the blurred desktop wallpaper with an opaque luminosity
//! layer, so the result is always opaque. Linux has no wallpaper sampling, so [`mica`] draws
//! the controller's fallback colour — the theme's `SolidBackgroundFillColorBase` — plus the
//! Acrylic noise; [`MicaBrush::composite_over`] gives the tinted colour for a known wallpaper.

use rune::{El, el};
use winui_theme::{Color, Theme};

use crate::acrylic::AcrylicBrush;
use crate::noise::noise_layer;
use crate::rc;

/// `MicaKind` (`Microsoft.UI.Composition.SystemBackdrops.MicaKind`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MicaKind {
    #[default]
    Base,
    BaseAlt,
}

/// The MicaController parameters for one theme and kind.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MicaBrush {
    pub kind: MicaKind,
    pub tint_color: Color,
    pub tint_opacity: f32,
    pub luminosity_opacity: f32,
    pub fallback_color: Color,
}

impl MicaBrush {
    /// `MicaController` defaults: Base tints with `SolidBackgroundFillColorBase`
    /// (#F3F3F3 at 0.5 light, #202020 at 0.8 dark); BaseAlt with #DADADA at 0.5 light and
    /// #0A0A0A at 0.0 dark. Luminosity opacity is 1.0 in every case.
    pub fn new(theme: &Theme, kind: MicaKind) -> Self {
        let base = theme.tokens().solid_background_fill_color_base;
        let (tint_color, tint_opacity) = match (kind, theme.is_dark()) {
            (MicaKind::Base, false) => (base, 0.5),
            (MicaKind::Base, true) => (base, 0.8),
            (MicaKind::BaseAlt, false) => (Color::rgb(0xDA, 0xDA, 0xDA), 0.5),
            (MicaKind::BaseAlt, true) => (Color::rgb(0x0A, 0x0A, 0x0A), 0.0),
        };
        Self {
            kind,
            tint_color,
            tint_opacity,
            luminosity_opacity: 1.0,
            fallback_color: tint_color,
        }
    }

    fn as_acrylic(&self) -> AcrylicBrush {
        AcrylicBrush::new(
            self.tint_color,
            self.tint_opacity,
            Some(self.luminosity_opacity),
            self.fallback_color,
        )
    }

    /// The opaque colour Mica resolves to over a wallpaper whose blurred average is `wallpaper`.
    pub fn composite_over(&self, wallpaper: Color) -> Color {
        self.as_acrylic().composite_over(wallpaper)
    }

    /// An opaque Mica surface: the fallback colour plus noise. Fill the window with it.
    pub fn el(&self) -> El {
        surface(rc(self.fallback_color.with_alpha(1.0)))
    }

    /// Mica tinted for a known wallpaper average (see [`Self::composite_over`]).
    pub fn el_over(&self, wallpaper: Color) -> El {
        surface(rc(self.composite_over(wallpaper)))
    }
}

fn surface(fill: rune::Color) -> El {
    el().key("mica")
        .w_full()
        .h_full()
        .fill(fill)
        .child(noise_layer(0.0))
}

/// The theme's Mica parameters.
pub fn mica_brush(theme: &Theme) -> MicaBrush {
    MicaBrush::new(theme, MicaKind::Base)
}

/// Opaque Mica (`MicaKind::Base`) window surface.
pub fn mica(theme: &Theme) -> El {
    mica_brush(theme).el()
}

/// Opaque Mica Alt (`MicaKind::BaseAlt`, tabbed title bars).
pub fn mica_alt(theme: &Theme) -> El {
    MicaBrush::new(theme, MicaKind::BaseAlt).el()
}
