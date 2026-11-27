//! Shared Fluent recipes: colour conversion, the type ramp as Rune text styles, focus visuals,
//! glyphs and the Rest/PointerOver/Pressed/Disabled fill sets every control resolves from
//! `Theme::tokens()`.
use std::cell::Cell;
use std::rc::Rc;

use rune::{El, Shape, TextStyle};

use crate::shim::icons::{self, Glyph, Symbol};
use crate::shim::theme::{self, radii, Theme, Tokens};

/// A WinUI `CommonStates` visual state that can be pinned for state galleries and
/// snapshots (`VisualStateManager.GoToState`). `Disabled` is `enabled(false)` on each control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VisualState {
    Rest,
    PointerOver,
    Pressed,
}

thread_local! {
    static PINNED: Cell<Option<VisualState>> = const { Cell::new(None) };
}

/// Builds `f` with every control's interactive state frozen to `s`: views resolve their
/// PointerOver / Pressed brushes statically instead of attaching hover and press hooks.
pub fn with_visual_state<R>(s: VisualState, f: impl FnOnce() -> R) -> R {
    let prev = PINNED.with(|p| p.replace(Some(s)));
    let r = f();
    PINNED.with(|p| p.set(prev));
    r
}

/// The state pinned by [`with_visual_state`], if any.
pub fn pinned() -> Option<VisualState> {
    PINNED.with(|p| p.get())
}

/// `rest` / `hover` / `pressed` by the pinned state (`rest` when live).
pub fn by_state<T>(rest: T, hover: T, pressed: T) -> T {
    match pinned() {
        Some(VisualState::PointerOver) => hover,
        Some(VisualState::Pressed) => pressed,
        _ => rest,
    }
}

/// `group_opacity(rest, hover)` honouring a pinned state (Pressed shows the hover layer).
pub fn hover_layer(e: El, rest: f32, hover: f32) -> El {
    match pinned() {
        None => e.group_opacity(rest, hover),
        Some(VisualState::Rest) => e.opacity(rest),
        Some(_) => e.opacity(hover),
    }
}

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
    e.focusable(true)
        .focus_ring(2.0, 1.0, rc(t.tokens().focus_stroke_color_outer))
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
        if enabled { by_state(self.rest, self.hover, self.pressed) } else { self.disabled }
    }
}

/// The control's elevation border (`ControlElevationBorderBrush` /
/// `AccentControlElevationBorderBrush`) sampled at the 32px control height, plus the solid
/// `*BorderBrushPressed` and `*BorderBrushDisabled` strokes.
#[derive(Clone, Copy, Debug)]
pub struct Elevation {
    pub top: theme::Color,
    pub bottom: theme::Color,
    pub pressed: theme::Color,
    pub flat: theme::Color,
}

impl Elevation {
    fn new(g: &theme::LinearGradient, pressed: theme::Color, disabled: theme::Color) -> Self {
        let h = theme::sizes::CONTROL_HEIGHT;
        Elevation { top: g.top(h), bottom: g.bottom(h), pressed, flat: disabled }
    }
}

/// Fill + stroke + foreground recipe for a WinUI button-like surface.
#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub fill: StateSet,
    pub fg: StateSet,
    pub border: Option<Elevation>,
    /// `BackgroundSizing="InnerBorderEdge"`: the stroke composites over the parent, not the
    /// fill (`DefaultButtonStyle`; `AccentButtonStyle` is `OuterBorderEdge`).
    pub inner_edge: bool,
}

/// The stroke that, drawn over `fill`, lands on the same pixel as `stroke` drawn straight over
/// the parent — exact for any parent when both share an RGB (Dark's white-alpha ramps).
pub fn inner_edge(stroke: theme::Color, fill: theme::Color) -> theme::Color {
    let same = (stroke.r - fill.r).abs() < 1e-3 && (stroke.g - fill.g).abs() < 1e-3 && (stroke.b - fill.b).abs() < 1e-3;
    if !same || fill.a <= 0.0 || fill.a >= 1.0 {
        return stroke;
    }
    stroke.with_alpha(((stroke.a - fill.a) / (1.0 - fill.a)).max(0.0))
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
            inner_edge: true,
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
            inner_edge: false,
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
            inner_edge: false,
        }
    }

    /// Paints `e` as this recipe: rest fill and border, plus hover/press fills and the press
    /// foreground when `enabled`; the disabled set otherwise.
    pub fn paint(&self, e: El, enabled: bool) -> El {
        let fill = self.fill.pick(enabled);
        let mut e = e.fill(rc(fill));
        if let Some(mut b) = self.border {
            if self.inner_edge {
                b = Elevation {
                    top: inner_edge(b.top, fill),
                    bottom: inner_edge(b.bottom, fill),
                    pressed: inner_edge(b.pressed, fill),
                    flat: inner_edge(b.flat, fill),
                };
            }
            e = match (enabled, pinned()) {
                (false, _) => e.border(1.0, rc(b.flat)),
                (true, Some(VisualState::Pressed)) => e.border(1.0, rc(b.pressed)),
                (true, _) => e.border_gradient(1.0, rc(b.top), rc(b.bottom)),
            };
        }
        if enabled && pinned().is_none() {
            e = e
                .hover_fill(rc(self.fill.hover))
                .press_fill(rc(self.fill.pressed))
                .press_color(rc(self.fg.pressed));
            if let Some(b) = self.border {
                e = e.press_border(rc(b.pressed));
            }
        }
        e
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
        .child(hover_layer(layer(rest), 1.0, 0.0))
        .child(hover_layer(layer(hover), 0.0, 1.0))
}

/// [`layers`] plus the Pressed fill/stroke, shown when Pressed is pinned. Live presses keep
/// the PointerOver layer: Rune's press hooks restyle only the pressed element, not children.
pub fn layers3(
    size: (f32, f32),
    radius: f32,
    rest: (theme::Color, Option<theme::Color>),
    hover: (theme::Color, Option<theme::Color>),
    pressed: (theme::Color, Option<theme::Color>),
) -> El {
    match pinned() {
        Some(VisualState::Pressed) => layers(size, radius, pressed, pressed),
        _ => layers(size, radius, rest, hover),
    }
}
