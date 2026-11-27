//! Reveal: a pointer-tracking hover glow plus a border spotlight.
//!
//! Geometry follows WinUI's `RevealHoverLight` / `RevealBorderLight`: a spotlight 128 points
//! above the surface with a 16.94532° outer cone lights the border within ~39 points of the
//! pointer; the hover light sits 256 points up with a cone sized so its pool spans
//! `clamp(max(w, h) + 12, 16, 512)` points. Colours are the `SystemReveal*` resources.
//!
//! The pointer moves `Live` rects only, so tracking redraws without rebuilding the view.

use rune::{El, Live, el};

use crate::color::to_rune;
use crate::theme::{Color, Theme, ThemeMode};

/// Border light spotlight height above the surface, in points.
pub const BORDER_LIGHT_HEIGHT: f32 = 128.0;
/// Border light outer cone angle, in degrees.
pub const BORDER_LIGHT_CONE_DEG: f32 = 16.945_32;

/// Radius of the lit border pool on the surface.
pub fn border_light_radius() -> f32 {
    BORDER_LIGHT_HEIGHT * BORDER_LIGHT_CONE_DEG.to_radians().tan()
}

/// Radius of the hover pool for an element of `size`.
pub fn hover_light_radius(size: [f32; 2]) -> f32 {
    (size[0].max(size[1]) + 12.0).clamp(16.0, 512.0)
}

/// Reveal colours.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RevealStyle {
    /// Peak of the hover pool.
    pub hover: Color,
    /// Peak of the border spotlight.
    pub border: Color,
    pub border_width: f32,
    pub radius: f32,
}

impl RevealStyle {
    /// List and navigation rows: `SystemRevealListLowColor` hover, `SystemRevealBaseMediumLowColor` border.
    pub fn list(theme: &Theme) -> Self {
        let (hover, border) = match theme.mode() {
            ThemeMode::Dark => (0x18FFFFFF, 0x61FFFFFF),
            ThemeMode::Light => (0x17000000, 0x61000000),
        };
        Self {
            hover: Color::argb(hover),
            border: Color::argb(border),
            border_width: 1.0,
            radius: 4.0,
        }
    }

    /// Buttons: `SystemRevealBaseLowColor` hover.
    pub fn base(theme: &Theme) -> Self {
        let hover = match theme.mode() {
            ThemeMode::Dark => 0x30FFFFFF,
            ThemeMode::Light => 0x2E000000,
        };
        Self {
            hover: Color::argb(hover),
            ..Self::list(theme)
        }
    }
}

/// Pointer state for one reveal surface. Keep it in app state and pass it to [`reveal`] each
/// view; clones share the same live values.
#[derive(Clone)]
pub struct RevealLight {
    pointer: Live<[f32; 2]>,
    size: Live<[f32; 2]>,
    lit: Live<f32>,
    hover_rect: Live<[f32; 4]>,
    /// Spotlight rects local to the top/left, bottom and right border strips.
    edge_top_left: Live<[f32; 4]>,
    edge_bottom: Live<[f32; 4]>,
    edge_right: Live<[f32; 4]>,
}

impl Default for RevealLight {
    fn default() -> Self {
        Self::new()
    }
}

impl RevealLight {
    pub fn new() -> Self {
        Self {
            pointer: Live::new([0.0; 2]),
            size: Live::new([0.0; 2]),
            lit: Live::new(0.0),
            hover_rect: Live::new([0.0; 4]),
            edge_top_left: Live::new([0.0; 4]),
            edge_bottom: Live::new([0.0; 4]),
            edge_right: Live::new([0.0; 4]),
        }
    }

    /// A light for a surface whose size is known up front, so the first frame is already right
    /// (otherwise the size arrives with the first layout).
    pub fn with_size(size: [f32; 2]) -> Self {
        let light = Self::new();
        light.set_size(size);
        light
    }

    /// Light the surface with the pointer at `(x, y)` in its local points.
    pub fn point_at(&self, x: f32, y: f32) {
        self.pointer.set([x, y]);
        self.lit.set(1.0);
        self.place();
    }

    /// Pointer left: turn the light off.
    pub fn release(&self) {
        self.lit.set(0.0);
    }

    pub fn is_lit(&self) -> bool {
        self.lit.get() > 0.0
    }

    pub fn pointer(&self) -> [f32; 2] {
        self.pointer.get()
    }

    /// The surface size from its last layout.
    pub fn size(&self) -> [f32; 2] {
        self.size.get()
    }

    /// Hover pool rect in surface points.
    pub fn hover_rect(&self) -> [f32; 4] {
        self.hover_rect.get()
    }

    fn set_size(&self, size: [f32; 2]) {
        self.size.set(size);
        self.place();
    }

    fn place(&self) {
        let [x, y] = self.pointer.get();
        let [w, h] = self.size.get();
        let square = |cx: f32, cy: f32, r: f32| [cx - r, cy - r, 2.0 * r, 2.0 * r];
        let rh = hover_light_radius([w, h]);
        let rb = border_light_radius();
        self.hover_rect.set(square(x, y, rh));
        self.edge_top_left.set(square(x, y, rb));
        self.edge_bottom.set(square(x, y - (h - 1.0).max(0.0), rb));
        self.edge_right.set(square(x - (w - 1.0).max(0.0), y, rb));
    }
}

fn pool(color: Color, radius: f32, rect: &Live<[f32; 4]>, lit: &Live<f32>) -> El {
    let peak = to_rune(color);
    let at = |k: f32| rune::Color {
        a: peak.a * k,
        ..peak
    };
    el().absolute(0.0, 0.0)
        .live_rect(rect)
        .live_opacity(lit)
        .radial_gradient(
            [0.5, 0.5],
            radius.max(1.0),
            [
                (0.0, at(1.0)),
                (0.4, at(0.7)),
                (0.75, at(0.25)),
                (1.0, at(0.0)),
            ],
        )
}

fn strip(pin: [Option<f32>; 4], thick: f32, vertical: bool, light: El) -> El {
    let s = el()
        .pin(pin[0], pin[1], pin[2], pin[3])
        .clip_soft(0.0)
        .child(light);
    if vertical { s.w(thick) } else { s.h(thick) }
}

/// `reveal(theme, light)` from the contract, with list-row colours. Size it and add content
/// children like any `El`; the light draws under them.
pub fn reveal(theme: &Theme, light: &RevealLight) -> El {
    reveal_with(&RevealStyle::list(theme), light)
}

pub fn reveal_with(style: &RevealStyle, light: &RevealLight) -> El {
    let hover_r = hover_light_radius(light.size());
    let border_r = border_light_radius();
    let bw = style.border_width;
    let edge = |rect: &Live<[f32; 4]>| pool(style.border, border_r, rect, &light.lit);
    let (on_layout, on_move, on_hover) = (light.clone(), light.clone(), light.clone());
    el().clip_rounded(style.radius)
        .on_layout_any(move |r, _tx| on_layout.set_size([r[2], r[3]]))
        .on_pointer_move_any(move |p, tx| {
            on_move.point_at(p.x, p.y);
            tx.live_only();
        })
        .on_hover_change_any(move |inside, tx| {
            if !inside {
                on_hover.release();
            }
            tx.live_only();
        })
        .child(pool(style.hover, hover_r, &light.hover_rect, &light.lit))
        .child(strip(
            [Some(0.0), Some(0.0), Some(0.0), None],
            bw,
            false,
            edge(&light.edge_top_left),
        ))
        .child(strip(
            [Some(0.0), Some(0.0), None, Some(0.0)],
            bw,
            true,
            edge(&light.edge_top_left),
        ))
        .child(strip(
            [Some(0.0), None, Some(0.0), Some(0.0)],
            bw,
            false,
            edge(&light.edge_bottom),
        ))
        .child(strip(
            [None, Some(0.0), Some(0.0), Some(0.0)],
            bw,
            true,
            edge(&light.edge_right),
        ))
}
