//! Shared Fluent recipes: colour conversion, the type ramp as Rune text styles, focus visuals,
//! glyphs and the Rest/PointerOver/Pressed/Disabled fill sets every control resolves from
//! `Theme::tokens()`.
use std::rc::Rc;

use rune::{El, Shape, TextStyle};

use crate::shim::icons::{self, Glyph, Symbol};
use crate::shim::theme::{self, radii, Theme, Tokens};

/// `winui_theme::Color` (sRGB) to Rune's linear colour.
pub fn rc(c: theme::Color) -> rune::Color {
    use rune::color::srgb_decode;
    rune::Color { r: srgb_decode(c.r), g: srgb_decode(c.g), b: srgb_decode(c.b), a: c.a }
}

/// A type-ramp entry as a Rune text style.
pub fn font(s: theme::TextStyle) -> TextStyle {
    TextStyle::new(s.size, s.weight as f32).lh(s.line_height)
}

pub fn body(t: &Theme) -> TextStyle {
    font(t.type_ramp().body)
}

pub fn body_strong(t: &Theme) -> TextStyle {
    font(t.type_ramp().body_strong)
}

pub fn caption(t: &Theme) -> TextStyle {
    font(t.type_ramp().caption)
}

/// Text in the body ramp.
pub fn label(s: impl Into<String>, t: &Theme, color: theme::Color) -> El {
    rune::text(s).font(body(t)).color(rc(color))
}

/// WinUI's system focus visual: a 2px `FocusStrokeColorOuter` ring outside the control, drawn
/// for keyboard focus only (`UseSystemFocusVisuals`).
pub fn focus_rect(e: El, t: &Theme, radius: f32) -> El {
    focus_rect_inset(e, t, radius, 0.0)
}

/// [`focus_rect`] for an element `inset` points inside the control's bounds (a face inside its
/// elevation ring): the ring still lands 1-3px outside the control, `FocusVisualMargin="-3"`.
pub fn focus_rect_inset(e: El, t: &Theme, radius: f32, inset: f32) -> El {
    e.focusable(true)
        .focus_ring(2.0, 1.0 + inset, rc(t.tokens().focus_stroke_color_outer))
        .focus_ring_radius(radius + 3.0)
}

/// One control's fill, stroke and foreground in each visual state.
#[derive(Clone, Copy, Debug)]
pub struct StateSet {
    pub rest: theme::Color,
    pub hover: theme::Color,
    pub pressed: theme::Color,
    pub disabled: theme::Color,
}

impl StateSet {
    pub fn pick(&self, enabled: bool) -> theme::Color {
        if enabled { self.rest } else { self.disabled }
    }
}

/// The control's elevation border (`ControlElevationBorderBrush` /
/// `AccentControlElevationBorderBrush`) plus the solid `*BorderBrushPressed` and
/// `*BorderBrushDisabled` strokes.
///
/// The brushes are 3px absolute gradients: the outermost device row on one edge (top in dark,
/// bottom in light and on accent, where the brush is flipped) is the 0.33 stop
/// (`ControlStrokeColorSecondary`) and everything past 3px is the 1.0 stop
/// (`ControlStrokeColorDefault`), so on a 1px border only that one edge differs. `edge` is that
/// colour, `body` the rest of the ring, `edge_bottom` which side it is on.
#[derive(Clone, Copy, Debug)]
pub struct Elevation {
    pub top: theme::Color,
    pub bottom: theme::Color,
    pub pressed: theme::Color,
    pub flat: theme::Color,
    pub edge: theme::Color,
    pub body: theme::Color,
    pub edge_bottom: bool,
}

impl Elevation {
    fn new(g: &theme::LinearGradient, pressed: theme::Color, disabled: theme::Color) -> Self {
        let h = theme::sizes::CONTROL_HEIGHT;
        let edge_bottom = g.flip_y;
        let (edge, body) = if edge_bottom { (g.bottom(h), g.top(h)) } else { (g.top(h), g.bottom(h)) };
        Elevation { top: g.top(h), bottom: g.bottom(h), pressed, flat: disabled, edge, body, edge_bottom }
    }

    /// The 1px ring as an out-of-flow child of a face inset [`INSET`] inside it, so the face's
    /// fill stops at the ring's inner edge (`BackgroundSizing="InnerBorderEdge"`) and the
    /// translucent stroke composites over what is behind the control, not over its fill.
    pub fn ring(&self, enabled: bool, corners: [f32; 4]) -> El {
        let o = -INSET;
        let pinned = || rune::el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)).corners(corners);
        let outer = rune::el().pin(Some(o), Some(o), Some(o), Some(o)).corners(corners);
        if !enabled {
            return outer.child(pinned().border(1.0, rc(self.flat)));
        }
        let (edge_sides, body_sides) =
            if self.edge_bottom { ([0.0, 0.0, 1.0, 0.0], [1.0, 1.0, 0.0, 1.0]) } else { ([1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 1.0, 1.0]) };
        outer
            .child(pinned().border_sides(body_sides, rc(self.body)))
            .child(pinned().border_sides(edge_sides, rc(self.edge)))
    }
}

/// How far a bordered face sits inside its control bounds: the 1px elevation ring.
pub const INSET: f32 = 1.0;

/// Shrinks each corner radius by [`INSET`] for a face inside its ring.
pub fn inner(corners: [f32; 4]) -> [f32; 4] {
    corners.map(|r| (r - INSET).max(0.0))
}

/// Fill + stroke + foreground recipe for a WinUI button-like surface.
#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub fill: StateSet,
    pub fg: StateSet,
    pub border: Option<Elevation>,
}

impl Recipe {
    /// `DefaultButtonStyle`.
    pub fn standard(k: &Tokens) -> Recipe {
        Recipe {
            fill: StateSet {
                rest: k.control_fill_color_default,
                hover: k.control_fill_color_secondary,
                pressed: k.control_fill_color_tertiary,
                disabled: k.control_fill_color_disabled,
            },
            fg: StateSet {
                rest: k.text_fill_color_primary,
                hover: k.text_fill_color_primary,
                pressed: k.text_fill_color_secondary,
                disabled: k.text_fill_color_disabled,
            },
            // ButtonBorderBrush / ButtonBorderBrushPressed / ButtonBorderBrushDisabled.
            border: Some(Elevation::new(&k.control_elevation_border, k.control_fill_color_transparent, k.control_stroke_color_default)),
        }
    }

    /// `AccentButtonStyle`.
    pub fn accent(k: &Tokens) -> Recipe {
        Recipe {
            fill: StateSet {
                rest: k.accent_fill_color_default,
                hover: k.accent_fill_color_secondary,
                pressed: k.accent_fill_color_tertiary,
                disabled: k.accent_fill_color_disabled,
            },
            fg: StateSet {
                rest: k.text_on_accent_fill_color_primary,
                hover: k.text_on_accent_fill_color_primary,
                pressed: k.text_on_accent_fill_color_secondary,
                disabled: k.text_on_accent_fill_color_disabled,
            },
            // AccentButtonBorderBrush / AccentButtonBorderBrushPressed / AccentButtonBorderBrushDisabled.
            border: Some(Elevation::new(&k.accent_control_elevation_border, k.control_fill_color_transparent, k.control_fill_color_transparent)),
        }
    }

    /// Subtle (transparent until hovered): `SubtleButtonStyle`, nav items, list rows.
    pub fn subtle(k: &Tokens) -> Recipe {
        Recipe {
            fill: StateSet {
                rest: k.subtle_fill_color_transparent,
                hover: k.subtle_fill_color_secondary,
                pressed: k.subtle_fill_color_tertiary,
                disabled: k.subtle_fill_color_disabled,
            },
            fg: StateSet {
                rest: k.text_fill_color_primary,
                hover: k.text_fill_color_primary,
                pressed: k.text_fill_color_secondary,
                disabled: k.text_fill_color_disabled,
            },
            border: None,
        }
    }

    /// Paints `e` as this recipe: rest fill and border, plus hover/press fills and the press
    /// foreground when `enabled`; the disabled set otherwise. A bordered recipe draws its ring
    /// [`INSET`] outside `e` (see [`Elevation::ring`]): size `e` to the control minus `2 * INSET`.
    pub fn paint(&self, e: El, enabled: bool) -> El {
        self.paint_corners(e, enabled, [R; 4])
    }

    /// [`Self::paint`] with per-corner control radii (split buttons, grouped parts).
    pub fn paint_corners(&self, e: El, enabled: bool, corners: [f32; 4]) -> El {
        let mut e = e.fill(rc(self.fill.pick(enabled)));
        if let Some(b) = self.border {
            e = e.mx(INSET).my(INSET).corners(inner(corners)).child(b.ring(enabled, corners));
        }
        if enabled {
            e = e
                .hover_fill(rc(self.fill.hover))
                .press_fill(rc(self.fill.pressed))
                .press_color(rc(self.fg.pressed));
        }
        e
    }

    /// Fills only (no ring), for one part of a control whose ring is drawn by its group.
    pub fn paint_fill(&self, e: El, enabled: bool) -> El {
        let e = e.fill(rc(self.fill.pick(enabled)));
        if enabled {
            e.hover_fill(rc(self.fill.hover)).press_fill(rc(self.fill.pressed)).press_color(rc(self.fg.pressed))
        } else {
            e
        }
    }

    pub fn fg(&self, enabled: bool) -> theme::Color {
        self.fg.pick(enabled)
    }
}

/// A Fluent glyph `size` points square.
pub fn glyph(s: Symbol, size: f32, color: theme::Color) -> El {
    match s.glyph() {
        Glyph::Path(d) => {
            let rule = match s.fill_rule() {
                icons::FillRule::EvenOdd => rune::FillRule::EvenOdd,
                icons::FillRule::NonZero => rune::FillRule::NonZero,
            };
            let vb = icons::VIEWBOX;
            rune::canvas(Shape::new().fill_svg_with(d, rule, rc(color)).view_box(vb, vb)).size(size, size)
        }
        Glyph::Font(ch) => rune::text(ch.to_string())
            .font(TextStyle::new(size, 400.0).lh(size))
            .color(rc(color))
            .size(size, size)
            .center(),
    }
}

/// A builder hook: attaches app handlers (generic over the app's state `S`) to an element,
/// given a payload the control computes while building its view.
pub struct Hook<V: Clone + 'static>(Vec<Rc<dyn Fn(El, V) -> El>>);

impl<V: Clone + 'static> Default for Hook<V> {
    fn default() -> Self {
        Hook(Vec::new())
    }
}

impl<V: Clone + 'static> Clone for Hook<V> {
    fn clone(&self) -> Self {
        Hook(self.0.clone())
    }
}

impl<V: Clone + 'static> Hook<V> {
    /// Run `f(state, payload)` when the element is pressed (pointer, Enter or Space).
    pub fn press<S: 'static>(&mut self, f: impl Fn(&mut S, V) + 'static) {
        let f = Rc::new(f);
        self.0.push(Rc::new(move |e: El, v: V| {
            let f = f.clone();
            e.on_press(move |s: &mut S| f(s, v.clone()))
        }));
    }

    pub fn raw(&mut self, f: impl Fn(El, V) -> El + 'static) {
        self.0.push(Rc::new(f));
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn attach(&self, mut e: El, v: V) -> El {
        for h in &self.0 {
            e = h(e, v.clone());
        }
        e
    }
}

/// The 4px control corner.
pub const R: f32 = radii::CONTROL;
/// The 8px overlay corner.
pub const R_OVERLAY: f32 = radii::OVERLAY;

/// A flyout / popup surface (`FlyoutPresenter`): acrylic-tinted solid, 1px flyout stroke, 8px
/// corners, `FlyoutThemeShadow` elevation.
/// `AcrylicBackgroundFillColorDefaultBrush`'s solid fallback (`#F9F9F9` light, `#2C2C2C`
/// dark) until a materials backdrop is drawn behind flyouts.
pub fn acrylic(t: &Theme) -> theme::Color {
    if t.is_dark() { theme::Color::rgb(0x2C, 0x2C, 0x2C) } else { theme::Color::rgb(0xF9, 0xF9, 0xF9) }
}

pub fn flyout_surface(t: &Theme) -> El {
    let k = t.tokens();
    rune::col()
        .fill(rc(acrylic(t)))
        .border(1.0, rc(k.surface_stroke_color_flyout))
        .radius(R_OVERLAY)
        .shadow(8.0, 16.0, rune::Color::BLACK.alpha(if t.is_dark() { 0.26 } else { 0.14 }))
}

/// A card surface (`CardBackgroundFillColorDefault` + `CardStrokeColorDefault`).
pub fn card(t: &Theme) -> El {
    let k = t.tokens();
    rune::col()
        .fill(rc(k.card_background_fill_color_default))
        .border(1.0, rc(k.card_stroke_color_default))
        .radius(R)
}

/// Rest and hover fills as two stacked layers: the hover one shows while the nearest
/// `hover_group` ancestor (the whole control, label included) is hovered, as WinUI's
/// `PointerOver` visual state covers the full hit target.
pub fn layers(size: (f32, f32), radius: f32, rest: (theme::Color, Option<theme::Color>), hover: (theme::Color, Option<theme::Color>)) -> El {
    let layer = |(fill, stroke): (theme::Color, Option<theme::Color>)| {
        let e = rune::el().absolute(0.0, 0.0).size(size.0, size.1).radius(radius).fill(rc(fill));
        match stroke {
            Some(s) => e.border(1.0, rc(s)),
            None => e,
        }
    };
    rune::el()
        .size(size.0, size.1)
        .child(layer(rest).group_opacity(1.0, 0.0))
        .child(layer(hover).group_opacity(0.0, 1.0))
}
