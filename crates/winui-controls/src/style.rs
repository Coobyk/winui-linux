//! Shared Fluent recipes: colour conversion, the type ramp as Rune text styles, focus visuals,
//! glyphs and the Rest/PointerOver/Pressed/Disabled fill sets every control resolves from
//! `Theme::tokens()`.
use std::rc::Rc;

use rune::{El, Shape, TextStyle};

use crate::shim::icons::{Glyph, Symbol};
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
        if enabled { self.rest } else { self.disabled }
    }
}

/// The control's elevation border (`ControlElevationBorderBrush`): darker bottom edge at rest.
#[derive(Clone, Copy, Debug)]
pub struct Elevation {
    pub top: theme::Color,
    pub bottom: theme::Color,
    pub flat: theme::Color,
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
            border: Some(Elevation {
                top: k.control_stroke_color_default,
                bottom: k.control_stroke_color_secondary,
                flat: k.control_stroke_color_default,
            }),
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
            border: Some(Elevation {
                top: k.control_stroke_color_on_accent_default,
                bottom: k.control_stroke_color_on_accent_secondary,
                flat: k.control_fill_color_transparent,
            }),
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
    /// foreground when `enabled`; the disabled set otherwise.
    pub fn paint(&self, e: El, enabled: bool) -> El {
        let mut e = e.fill(rc(self.fill.pick(enabled)));
        if let Some(b) = self.border {
            e = if enabled { e.border_gradient(1.0, rc(b.top), rc(b.bottom)) } else { e.border(1.0, rc(b.flat)) };
        }
        if enabled {
            e = e
                .hover_fill(rc(self.fill.hover))
                .press_fill(rc(self.fill.pressed))
                .press_color(rc(self.fg.pressed));
            if let Some(b) = self.border {
                e = e.press_border(rc(b.flat));
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
            let vb = s.view_box();
            rune::canvas(Shape::new().fill_svg(d, rc(color)).view_box(vb, vb)).size(size, size)
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
pub fn flyout_surface(t: &Theme) -> El {
    let k = t.tokens();
    rune::col()
        .fill(rc(k.acrylic_background_fill_color_default))
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
