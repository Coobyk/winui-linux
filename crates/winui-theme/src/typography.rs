//! Fluent type ramp (Segoe UI Variable).
//!
//! Sizes and weights: `TextBlock_themeresources.xaml` (`*TextBlockFontSize`,
//! `*TextBlockStyle`). Line heights: the Windows type ramp in
//! learn.microsoft.com "Typography in Windows" (size/line height in epx).

/// One ramp entry. `weight` is the OpenType weight (400 Regular, 600 Semibold).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyle {
    pub size: f32,
    pub weight: u16,
    pub line_height: f32,
}

impl TextStyle {
    pub const fn new(size: f32, weight: u16, line_height: f32) -> Self {
        Self {
            size,
            weight,
            line_height,
        }
    }

    /// The Segoe UI Variable optical-size face Windows uses for this size:
    /// Small below 14px, Text for 14–19px, Display from 20px up.
    pub fn optical_family(&self) -> &'static str {
        if self.size < 14.0 {
            fonts::SMALL
        } else if self.size < 20.0 {
            fonts::TEXT
        } else {
            fonts::DISPLAY
        }
    }

    pub const fn with_weight(self, weight: u16) -> Self {
        Self { weight, ..self }
    }
}

/// `FontWeights` values used by the ramp.
pub mod weights {
    pub const REGULAR: u16 = 400;
    pub const SEMIBOLD: u16 = 600;
    pub const BOLD: u16 = 700;
}

/// The WinUI 3 type ramp.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeRamp {
    pub caption: TextStyle,
    pub body: TextStyle,
    pub body_strong: TextStyle,
    pub body_large: TextStyle,
    pub body_large_strong: TextStyle,
    pub subtitle: TextStyle,
    pub title: TextStyle,
    pub title_large: TextStyle,
    pub display: TextStyle,
}

impl TypeRamp {
    pub const FLUENT: Self = {
        use weights::{REGULAR, SEMIBOLD};
        Self {
            caption: TextStyle::new(12.0, REGULAR, 16.0),
            body: TextStyle::new(14.0, REGULAR, 20.0),
            body_strong: TextStyle::new(14.0, SEMIBOLD, 20.0),
            body_large: TextStyle::new(18.0, REGULAR, 24.0),
            body_large_strong: TextStyle::new(18.0, SEMIBOLD, 24.0),
            subtitle: TextStyle::new(20.0, SEMIBOLD, 28.0),
            title: TextStyle::new(28.0, SEMIBOLD, 36.0),
            title_large: TextStyle::new(40.0, SEMIBOLD, 52.0),
            display: TextStyle::new(68.0, SEMIBOLD, 92.0),
        }
    };

    /// `(WinUI style name, style)` pairs, smallest first.
    pub fn named(&self) -> [(&'static str, TextStyle); 9] {
        [
            ("Caption", self.caption),
            ("Body", self.body),
            ("BodyStrong", self.body_strong),
            ("BodyLarge", self.body_large),
            ("BodyLargeStrong", self.body_large_strong),
            ("Subtitle", self.subtitle),
            ("Title", self.title),
            ("TitleLarge", self.title_large),
            ("Display", self.display),
        ]
    }
}

impl Default for TypeRamp {
    fn default() -> Self {
        Self::FLUENT
    }
}

/// Font families.
pub mod fonts {
    /// `ContentControlThemeFontFamily` resolves to Segoe UI Variable on Windows 11.
    pub const FAMILY: &str = "Segoe UI Variable";
    pub const SMALL: &str = "Segoe UI Variable Small";
    pub const TEXT: &str = "Segoe UI Variable Text";
    pub const DISPLAY: &str = "Segoe UI Variable Display";
    /// Metric-compatible fallbacks for Linux, in preference order.
    pub const FALLBACKS: &[&str] = &[
        "Segoe UI",
        "Inter",
        "Noto Sans",
        "DejaVu Sans",
        "sans-serif",
    ];
    /// `SymbolThemeFontFamily` = "Segoe Fluent Icons,Segoe MDL2 Assets".
    pub const SYMBOL: &[&str] = &["Segoe Fluent Icons", "Segoe MDL2 Assets"];
    /// `ControlContentThemeFontSize`.
    pub const CONTROL_CONTENT_SIZE: f32 = 14.0;
    /// `ToolTipContentThemeFontSize`.
    pub const TOOLTIP_SIZE: f32 = 12.0;
}
