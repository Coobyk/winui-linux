//! Theme: mode + accent ramp + resolved tokens.

use crate::accent::AccentRamp;
use crate::color::Color;
use crate::tokens::Tokens;
use crate::typography::TypeRamp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggled(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

/// A resolved Fluent theme. Cheap to rebuild; swap the whole value to switch
/// mode or accent at runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    mode: ThemeMode,
    accent: AccentRamp,
    tokens: Tokens,
    type_ramp: TypeRamp,
}

impl Theme {
    pub fn new(mode: ThemeMode, accent: AccentRamp) -> Self {
        Self {
            mode,
            accent,
            tokens: Tokens::resolve(mode, &accent),
            type_ramp: TypeRamp::FLUENT,
        }
    }

    pub fn light() -> Self {
        Self::new(ThemeMode::Light, AccentRamp::DEFAULT)
    }

    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark, AccentRamp::DEFAULT)
    }

    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }

    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn accent(&self) -> &AccentRamp {
        &self.accent
    }

    pub fn type_ramp(&self) -> &TypeRamp {
        &self.type_ramp
    }

    /// Re-resolve with a different accent colour; the ramp is derived via
    /// [`AccentRamp::derive`].
    pub fn with_accent(&self, accent: Color) -> Self {
        self.with_accent_ramp(AccentRamp::derive(accent))
    }

    /// Re-resolve with an explicit accent ramp (e.g. one read from the OS).
    pub fn with_accent_ramp(&self, accent: AccentRamp) -> Self {
        Self::new(self.mode, accent)
    }

    /// Same accent, different mode.
    pub fn with_mode(&self, mode: ThemeMode) -> Self {
        Self::new(mode, self.accent)
    }

    /// Light <-> dark, keeping the accent.
    pub fn toggled(&self) -> Self {
        self.with_mode(self.mode.toggled())
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}
