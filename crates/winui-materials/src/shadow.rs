//! Fluent elevation: an ambient shadow plus a key (directional) shadow per depth.
//!
//! Depth `d` casts a key shadow `0 d/2 d` and an ambient `0 0 2` (`0 0 8` from depth 28 up), as
//! the Fluent shadow tokens do; alphas are 14%/12% in light theme and 28%/24% in dark.

use rune::{El, Shadow};

use crate::theme::ThemeMode;

/// The standard depths.
pub const DEPTHS: [f32; 7] = [2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0];

/// Semantic depths from the Windows 11 layering guidance.
pub mod depth {
    pub const LAYER: f32 = 1.0;
    pub const CONTROL: f32 = 2.0;
    pub const CARD: f32 = 8.0;
    pub const TOOLTIP: f32 = 16.0;
    pub const FLYOUT: f32 = 32.0;
    pub const DIALOG: f32 = 128.0;
}

/// Shadow pair for one depth.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Elevation {
    pub depth: f32,
    pub ambient: Shadow,
    pub key: Shadow,
}

impl Elevation {
    /// Add both shadows to `el` (ambient first, so the key draws on top).
    pub fn apply(&self, el: El) -> El {
        if self.depth <= 0.0 {
            return el;
        }
        el.box_shadow(self.ambient).box_shadow(self.key)
    }
}

/// `shadow(depth)` from the contract, in light theme.
pub fn shadow(depth: f32) -> Elevation {
    shadow_for(depth, ThemeMode::Light)
}

/// Elevation for `depth` in `mode`.
pub fn shadow_for(depth: f32, mode: ThemeMode) -> Elevation {
    let d = depth.max(0.0);
    let (key_a, ambient_a) = match mode {
        ThemeMode::Light => (0.14, 0.12),
        ThemeMode::Dark => (0.28, 0.24),
    };
    let black = |a: f32| rune::Color {
        a,
        ..rune::Color::BLACK
    };
    let ambient_blur = if d >= 28.0 { 8.0 } else { 2.0 };
    Elevation {
        depth: d,
        ambient: Shadow {
            x: 0.0,
            y: 0.0,
            blur: ambient_blur,
            spread: 0.0,
            color: black(ambient_a),
            inset: false,
        },
        key: Shadow {
            x: 0.0,
            y: d / 2.0,
            blur: d,
            spread: 0.0,
            color: black(key_a),
            inset: false,
        },
    }
}

/// Builder sugar: `el().elevation(8.0, mode)`.
pub trait ElevationExt {
    fn elevation(self, depth: f32, mode: ThemeMode) -> El;
}

impl ElevationExt for El {
    fn elevation(self, depth: f32, mode: ThemeMode) -> El {
        shadow_for(depth, mode).apply(self)
    }
}
