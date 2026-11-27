//! Window-chrome colours that live outside `Common_themeresources_any.xaml`: the caption
//! (minimize / maximize / close) buttons Windows 11 draws, and the solid fallbacks of the
//! acrylic brushes WinUI uses when the backdrop is unavailable.

use crate::color::Color;
use crate::theme::ThemeMode;
use crate::tokens::Tokens;

/// Caption button fills and glyph colours per visual state.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaptionColors {
    /// Minimize / maximize: `SubtleFillColorSecondary` / `SubtleFillColorTertiary`.
    pub fill_pointer_over: Color,
    pub fill_pressed: Color,
    pub foreground: Color,
    pub foreground_pressed: Color,
    pub foreground_inactive: Color,
    /// Close button: Windows 11 `#C42B1C` hover, 90% on press, white glyph.
    pub close_fill_pointer_over: Color,
    pub close_fill_pressed: Color,
    pub close_foreground_pointer_over: Color,
    pub close_foreground_pressed: Color,
}

/// Windows 11's close-button hover red (DWM caption buttons, `AppWindowTitleBar`).
pub const CAPTION_CLOSE_RED: Color = Color::from_argb(0xFFC42B1C);
/// The Windows 10 / Win32 `ButtonCloseHover` red, for apps that draw the legacy chrome.
pub const CAPTION_CLOSE_RED_LEGACY: Color = Color::from_argb(0xFFE81123);

impl CaptionColors {
    pub fn resolve(k: &Tokens) -> Self {
        Self {
            fill_pointer_over: k.subtle_fill_color_secondary,
            fill_pressed: k.subtle_fill_color_tertiary,
            foreground: k.text_fill_color_primary,
            foreground_pressed: k.text_fill_color_secondary,
            foreground_inactive: k.text_fill_color_disabled,
            close_fill_pointer_over: CAPTION_CLOSE_RED,
            close_fill_pressed: CAPTION_CLOSE_RED.with_opacity(0.9),
            close_foreground_pointer_over: Color::WHITE,
            close_foreground_pressed: Color::WHITE.with_opacity(0.7),
        }
    }
}

/// `AcrylicBackgroundFillColorDefault` / `AcrylicInAppFillColorDefault` fallback colours
/// (`AcrylicBrush.FallbackColor`, `Common_themeresources_any.xaml`).
pub fn acrylic_fallback(mode: ThemeMode) -> Color {
    match mode {
        ThemeMode::Dark => Color::from_argb(0xFF2C2C2C),
        ThemeMode::Light => Color::from_argb(0xFFF9F9F9),
    }
}
