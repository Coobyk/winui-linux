//! Mica: the opaque window backdrop.
//!
//! Real Mica tints a blurred wallpaper with the base colour at luminosity opacity 1.0, which
//! without a wallpaper resolves to the base colour itself. We draw that colour plus faint grain.

use rune::{El, el};

use crate::color::to_rune;
use crate::noise::{NOISE_OPACITY, noise_layer};
use crate::theme::Theme;

/// Mica flavour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MicaKind {
    /// `SolidBackgroundFillColorBase`: window backgrounds.
    #[default]
    Base,
    /// `SolidBackgroundFillColorBaseAlt`: tabbed title bars.
    Alt,
}

/// The Mica fill colour for `kind`, from the theme's tokens.
pub fn mica_color(theme: &Theme, kind: MicaKind) -> rune::Color {
    let t = theme.tokens();
    to_rune(match kind {
        MicaKind::Base => t.solid_background_fill_color_base,
        MicaKind::Alt => t.solid_background_fill_color_base_alt,
    })
}

/// `mica(theme)` from the contract: opaque base colour + faint noise.
pub fn mica(theme: &Theme) -> El {
    mica_with(theme, MicaKind::Base)
}

/// Mica Alt.
pub fn mica_alt(theme: &Theme) -> El {
    mica_with(theme, MicaKind::Alt)
}

pub fn mica_with(theme: &Theme, kind: MicaKind) -> El {
    el().fill(mica_color(theme, kind))
        .child(noise_layer(NOISE_OPACITY))
}
