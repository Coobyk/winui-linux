//! Reveal highlight (`RevealBrush` + `RevealBorderLight` / `RevealHoverLight`,
//! controls/dev/Materials/Reveal).
//!
//! WinUI lights an element with composition spotlights that follow the pointer from above the
//! surface: a wide hover light fills the background and a narrow border light picks out the
//! edges. The brush colour is what the light "reveals". Here each spotlight's footprint (the
//! cone's intersection with the surface, `height · tan(outerConeAngle)`) is drawn as a radial
//! gradient from the brush colour at the pointer to transparent at the cone's edge.

use rune::{El, PointerEvent, Tx, el};
use winui_theme::{Color, Theme};

use crate::acrylic::full;
use crate::rc;

/// Spotlight parameters from `RevealBorderLight.cpp` / `RevealHoverLight.cpp`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RevealLight {
    pub height: f32,
    pub outer_cone_angle_deg: f32,
    pub constant_attenuation: f32,
    pub linear_attenuation: f32,
}

impl RevealLight {
    /// `RevealBorderLight`, normal.
    pub const BORDER: Self = Self {
        height: 128.0,
        outer_cone_angle_deg: 16.94532,
        constant_attenuation: 1.0,
        linear_attenuation: 3.0,
    };
    /// `RevealBorderLight`, `IsWideLight`.
    pub const BORDER_WIDE: Self = Self {
        height: 256.0,
        outer_cone_angle_deg: 16.7403,
        constant_attenuation: 1.0,
        linear_attenuation: 0.5,
    };

    /// `RevealHoverLight`: height 256 with `outerAngle = atan(clamp(max(w, h) + 12, 16, 512) / 256)`.
    pub fn hover(w: f32, h: f32) -> Self {
        let size = (w.max(h) + 12.0).clamp(16.0, 512.0);
        Self {
            height: 256.0,
            outer_cone_angle_deg: (size / 256.0).atan().to_degrees(),
            constant_attenuation: 2.0,
            linear_attenuation: 0.5,
        }
    }

    /// `RevealHoverLight` while pressed: `atan(47.25 / 256)`.
    pub const PRESS: Self = Self {
        height: 256.0,
        outer_cone_angle_deg: 10.4580,
        constant_attenuation: 2.0,
        linear_attenuation: 0.5,
    };

    /// Radius in points of the lit footprint on the surface.
    pub fn radius(&self) -> f32 {
        self.height * self.outer_cone_angle_deg.to_radians().tan()
    }
}

/// Reveal state to keep in the app model: the pointer in the element's local coordinates while
/// it is over the element. Wire it with [`Reveal::track`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Reveal {
    pub pointer: Option<[f32; 2]>,
    pub pressed: bool,
}

impl Reveal {
    /// Follows the pointer over `target`, storing it in the [`Reveal`] picked by `lens`.
    pub fn track<S: 'static>(target: El, lens: fn(&mut S) -> &mut Reveal) -> El {
        target
            .on_pointer_move_local(move |s: &mut S, p: PointerEvent, _tx: &mut Tx| {
                let r = lens(s);
                r.pointer = Some([p.x, p.y]);
                r.pressed = p.pressing;
            })
            .on_hover_change(move |s: &mut S, over: bool, _tx: &mut Tx| {
                if !over {
                    *lens(s) = Reveal::default();
                }
            })
    }

    /// `SystemControlBackgroundBaseLowRevealBorderBrush` → `SystemBaseLowColor`
    /// (#33000000 light, #33FFFFFF dark).
    pub fn border_color(theme: &Theme) -> Color {
        if theme.is_dark() {
            Color::rgba(255, 255, 255, 0.2)
        } else {
            Color::rgba(0, 0, 0, 0.2)
        }
    }

    /// `SystemControlHighlightListLowRevealBackgroundBrush` → `SystemListLowColor`
    /// (#19000000 light, #19FFFFFF dark).
    pub fn hover_color(theme: &Theme) -> Color {
        let a = 0x19 as f32 / 255.0;
        if theme.is_dark() {
            Color::rgba(255, 255, 255, a)
        } else {
            Color::rgba(0, 0, 0, a)
        }
    }

    /// `SystemControlHighlightAccentRevealBorderBrush` → the theme's `SystemAccentColor`.
    pub fn accent_border_color(theme: &Theme) -> Color {
        theme.accent().base
    }
}

fn glow(color: Color) -> [(f32, rune::Color); 2] {
    [(0.0, rc(color)), (1.0, rc(color.with_alpha(0.0)))]
}

/// Reveal overlay for a `w × h` element with corner `radius`: a hover light over the
/// background and a border light on a 1-point edge, both centred on `state.pointer`, in the
/// theme's Reveal colours. Draws nothing while the pointer is away. Stack it as the last child.
pub fn reveal(theme: &Theme, state: &Reveal, w: f32, h: f32, radius: f32) -> El {
    reveal_with(
        state,
        w,
        h,
        radius,
        Reveal::hover_color(theme),
        Reveal::border_color(theme),
    )
}

/// [`reveal`] with explicit brush colours (e.g. [`Reveal::accent_border_color`]).
pub fn reveal_with_colors(
    state: &Reveal,
    w: f32,
    h: f32,
    radius: f32,
    hover: Color,
    border: Color,
) -> El {
    reveal_with(state, w, h, radius, hover, border)
}

fn reveal_with(state: &Reveal, w: f32, h: f32, radius: f32, hover: Color, border: Color) -> El {
    let layer = full(radius).key("winui-reveal");
    let Some([px, py]) = state.pointer else {
        return layer;
    };
    let hover_light = if state.pressed {
        RevealLight::PRESS
    } else {
        RevealLight::hover(w, h)
    };
    let hr = hover_light.radius();
    let br = RevealLight::BORDER.radius();
    let norm = |x: f32, y: f32, sw: f32, sh: f32| [x / sw.max(1e-3), y / sh.max(1e-3)];
    let bg = full(radius).radial_gradient(norm(px, py, w, h), hr, glow(hover));
    // Edges: 1-point strips, inset past the corner arcs, each lit from the shared pointer.
    let t = 1.0;
    let inset = radius.max(0.0);
    let (iw, ih) = ((w - 2.0 * inset).max(0.0), (h - 2.0 * inset).max(0.0));
    let strip = |x: f32, y: f32, sw: f32, sh: f32| {
        el().absolute(x, y).size(sw, sh).radial_gradient(
            norm(px - x, py - y, sw, sh),
            br,
            glow(border),
        )
    };
    let mut out = layer.child(bg);
    if iw > 0.0 {
        out = out
            .child(strip(inset, 0.0, iw, t))
            .child(strip(inset, h - t, iw, t));
    }
    if ih > 0.0 {
        out = out
            .child(strip(0.0, inset, t, ih))
            .child(strip(w - t, inset, t, ih));
    }
    out
}
