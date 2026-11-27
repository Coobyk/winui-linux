//! Fluent design tokens: colours, type ramp, spacing, radii, sizes, motion.
//! Pure data — no rendering dependency. See docs/ARCHITECTURE.md for the contract.
//!
//! ```
//! use winui_theme::{Color, Theme, ThemeMode};
//! let dark = Theme::dark();
//! assert_eq!(dark.mode(), ThemeMode::Dark);
//! assert_eq!(dark.tokens().solid_background_fill_color_base, Color::rgb(0x20, 0x20, 0x20));
//! let purple = dark.with_accent(Color::rgb(0x88, 0x17, 0x98));
//! assert_ne!(purple.tokens().accent_fill_color_default, dark.tokens().accent_fill_color_default);
//! ```

mod accent;
mod color;
mod gradient;
mod metrics;
mod theme;
mod tokens;
mod typography;

pub use accent::AccentRamp;
pub use color::Color;
pub use gradient::{GradientMapping, LinearGradient};
pub use metrics::{Thickness, motion, radii, sizes, spacing};
pub use theme::{Theme, ThemeMode};
pub use tokens::Tokens;
pub use typography::{TextStyle, TypeRamp, fonts, weights};
