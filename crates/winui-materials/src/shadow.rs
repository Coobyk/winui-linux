//! `ThemeShadow` elevation, ported from `DropShadowRecipe.h` (dxaml/xcp/components/graphics).
//!
//! WinUI derives two black drop shadows (ambient + directional) from an element's
//! `Translation.Z`; `depth` here is that Z value. Composition `BlurRadius` maps to Rune's
//! CSS-style `blur` one-to-one.

use rune::{El, Shadow};
use winui_theme::{Color, Theme, ThemeMode};

/// The Z depths WinUI's own controls use: tooltips 16, flyouts/menus 32, dialogs 128, …
pub const THEME_SHADOW_DEPTHS: [f32; 7] = [2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0];

/// One elevation's shadow parameters (`DropShadowRecipe`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropShadowRecipe {
    /// `min(64, depth / 2)`.
    pub elevation: f32,
    pub ambient_blur_radius: f32,
    pub ambient_y_offset: f32,
    pub ambient_opacity: f32,
    pub directional_blur_radius: f32,
    pub directional_y_offset: f32,
    pub directional_opacity: f32,
}

impl DropShadowRecipe {
    pub fn new(depth: f32, mode: ThemeMode) -> Self {
        let elevation = (depth / 2.0).min(64.0);
        let dark = mode == ThemeMode::Dark;
        let (ambient_blur_radius, ambient_y_offset, ambient_opacity, directional_opacity) =
            if elevation < 2.0 {
                (2.0, 0.0, 0.0, 0.0)
            } else if elevation <= 16.0 {
                (
                    2.0,
                    0.0,
                    0.0,
                    if dark {
                        0.26
                    } else {
                        (elevation / 100.0 + 0.06).min(0.14)
                    },
                )
            } else if dark {
                (elevation / 3.0, 2.0, 0.37, 0.37)
            } else {
                (elevation / 3.0, 2.0, 0.15, 0.19)
            };
        Self {
            elevation,
            ambient_blur_radius,
            ambient_y_offset,
            ambient_opacity,
            directional_blur_radius: elevation,
            directional_y_offset: elevation * 0.5,
            directional_opacity,
        }
    }

    /// Both layers as Rune box shadows, ambient first. Zero-opacity layers are dropped.
    pub fn shadows(&self) -> Vec<Shadow> {
        [
            (
                self.ambient_y_offset,
                self.ambient_blur_radius,
                self.ambient_opacity,
            ),
            (
                self.directional_y_offset,
                self.directional_blur_radius,
                self.directional_opacity,
            ),
        ]
        .into_iter()
        .filter(|&(_, _, a)| a > 0.0)
        .map(|(y, blur, a)| layer(y, blur, a))
        .collect()
    }
}

/// A `ThemeShadow` at Z `depth`; resolve it per theme with [`ThemeShadow::recipe`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeShadow {
    pub depth: f32,
}

impl ThemeShadow {
    pub fn recipe(&self, mode: ThemeMode) -> DropShadowRecipe {
        DropShadowRecipe::new(self.depth, mode)
    }

    /// Adds this shadow's layers to `el` for `theme`'s mode.
    pub fn apply(&self, el: El, theme: &Theme) -> El {
        self.recipe(theme.mode())
            .shadows()
            .into_iter()
            .fold(el, El::box_shadow)
    }
}

/// `ThemeShadow` for an element raised to `Translation.Z = depth`.
pub fn shadow(depth: f32) -> ThemeShadow {
    ThemeShadow { depth }
}

/// Material modifiers on any Rune element.
pub trait MaterialExt {
    /// Casts a WinUI `ThemeShadow` from Z `depth` (see [`THEME_SHADOW_DEPTHS`]).
    fn theme_shadow(self, theme: &Theme, depth: f32) -> Self;
}

impl MaterialExt for El {
    fn theme_shadow(self, theme: &Theme, depth: f32) -> Self {
        shadow(depth).apply(self, theme)
    }
}

/// The directional (key) layer of `ThemeShadow` at Z `depth`, for a single `box_shadow`;
/// transparent when the recipe has none (Z < 4).
pub fn elevation(theme: &Theme, depth: f32) -> Shadow {
    let r = shadow(depth).recipe(theme.mode());
    layer(
        r.directional_y_offset,
        r.directional_blur_radius,
        r.directional_opacity,
    )
}

/// The ambient layer of `ThemeShadow` at Z `depth` (transparent up to Z 32).
pub fn ambient(theme: &Theme, depth: f32) -> Shadow {
    let r = shadow(depth).recipe(theme.mode());
    layer(r.ambient_y_offset, r.ambient_blur_radius, r.ambient_opacity)
}

/// `content` rounded to `radius` and casting the full `ThemeShadow` at Z `depth`.
pub fn card_shadow(theme: &Theme, depth: f32, radius: f32, content: El) -> El {
    content.radius(radius).theme_shadow(theme, depth)
}

fn layer(y: f32, blur: f32, a: f32) -> Shadow {
    Shadow {
        x: 0.0,
        y,
        blur,
        spread: 0.0,
        color: crate::rc(Color::rgba(0, 0, 0, a)),
        inset: false,
    }
}
