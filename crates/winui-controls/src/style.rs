//! Shared plumbing: colour conversion, Fluent fonts, visual states, focus visuals, handlers.

use rune::{Color as RColor, El, Family, Shadow, TextStyle as RStyle};
use winui_theme::{Color, LinearGradient, TextStyle, Theme, ThemeMode, Tokens, fonts, sizes};

/// The XAML `VisualStateManager` common states every control renders.
///
/// [`VisualState::Rest`] is the live state: hover and press visuals are attached to the
/// element and the runtime switches to them under the pointer. The other states force that
/// state's static visuals, so a scene or test can show (and snapshot) them without a pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum VisualState {
    #[default]
    Rest,
    PointerOver,
    Pressed,
    Disabled,
}

impl VisualState {
    pub const ALL: [VisualState; 4] = [Self::Rest, Self::PointerOver, Self::Pressed, Self::Disabled];

    pub fn name(self) -> &'static str {
        match self {
            Self::Rest => "Rest",
            Self::PointerOver => "PointerOver",
            Self::Pressed => "Pressed",
            Self::Disabled => "Disabled",
        }
    }
}

/// One brush per common state, as a `*Background` / `*BackgroundPointerOver` /
/// `*BackgroundPressed` / `*BackgroundDisabled` resource quartet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Brushes {
    pub rest: Color,
    pub over: Color,
    pub pressed: Color,
    pub disabled: Color,
}

impl Brushes {
    pub const fn new(rest: Color, over: Color, pressed: Color, disabled: Color) -> Self {
        Self { rest, over, pressed, disabled }
    }
    pub const fn same(c: Color) -> Self {
        Self::new(c, c, c, c)
    }
    pub fn get(&self, s: VisualState) -> Color {
        match s {
            VisualState::Rest => self.rest,
            VisualState::PointerOver => self.over,
            VisualState::Pressed => self.pressed,
            VisualState::Disabled => self.disabled,
        }
    }
}

/// Theme sRGB colour to Rune's linear colour.
pub fn rc(c: Color) -> RColor {
    let d = rune::color::srgb_decode;
    RColor { r: d(c.r), g: d(c.g), b: d(c.b), a: c.a }
}

const TEXT_FAMILY: &str = "Segoe UI Variable Text, Segoe UI Variable, Segoe UI, Selawik";
const SMALL_FAMILY: &str = "Segoe UI Variable Small, Segoe UI Variable, Segoe UI, Selawik";
const DISPLAY_FAMILY: &str = "Segoe UI Variable Display, Segoe UI Variable, Segoe UI, Selawik";

/// A Fluent type-ramp style as a Rune text style, in its optical Segoe UI Variable cut
/// (falling back to Rune's sans slot where Segoe isn't installed).
pub fn font(ts: TextStyle) -> RStyle {
    let family = match ts.optical_family() {
        f if f == fonts::SMALL => SMALL_FAMILY,
        f if f == fonts::DISPLAY => DISPLAY_FAMILY,
        _ => TEXT_FAMILY,
    };
    RStyle { line_height: ts.line_height, ..RStyle::new(ts.size, ts.weight as f32).family(Family::Named(family)) }
}

/// `ControlContentThemeFontSize` 14 / Body.
pub fn body(theme: &Theme) -> RStyle {
    font(theme.type_ramp().body)
}

pub(crate) fn effective(enabled: bool, state: VisualState) -> VisualState {
    if enabled { state } else { VisualState::Disabled }
}

/// Fill with live hover/press brushes at Rest, or the forced state's brush.
pub(crate) fn fill(e: El, b: &Brushes, s: VisualState) -> El {
    match s {
        VisualState::Rest => e.fill(rc(b.rest)).hover_fill(rc(b.over)).press_fill(rc(b.pressed)),
        s => e.fill(rc(b.get(s))),
    }
}

/// Foreground for text inside a pressable `e`: at Rest the pressed brush follows the press.
pub(crate) fn press_ink(e: El, b: &Brushes, s: VisualState) -> El {
    if s == VisualState::Rest && b.pressed != b.rest { e.press_color(rc(b.pressed)) } else { e }
}

/// Border for a control whose Rest/PointerOver brush is an elevation gradient and whose
/// Pressed/Disabled brush is solid (`ButtonBorderBrush*`, `AccentButtonBorderBrush*`).
pub(crate) fn elevation_border(e: El, g: &LinearGradient, pressed: Color, disabled: Color, h: f32, s: VisualState) -> El {
    let w = sizes::CONTROL_BORDER;
    match s {
        VisualState::Rest => e.border_gradient(w, rc(g.top(h)), rc(g.bottom(h))).press_border(rc(pressed)),
        VisualState::PointerOver => e.border_gradient(w, rc(g.top(h)), rc(g.bottom(h))),
        VisualState::Pressed => e.border(w, rc(pressed)),
        VisualState::Disabled => e.border(w, rc(disabled)),
    }
}

/// WinUI's `UseSystemFocusVisuals` rectangle: a 2px `FocusStrokeColorOuter` stroke outside a
/// 1px `FocusStrokeColorInner` stroke, `FocusVisualMargin` -3 around the control. Drawn on
/// keyboard focus; `forced` draws it now (scenes, snapshots).
pub(crate) fn focus_visual(e: El, t: &Tokens, radius: f32, forced: bool) -> El {
    let outer = rc(t.focus_stroke_color_outer);
    let inner = rc(t.focus_stroke_color_inner);
    let off = sizes::FOCUS_STROKE_INNER;
    let e = e.focus_ring(sizes::FOCUS_STROKE_OUTER, off, outer).focus_ring_radius(radius + off + sizes::FOCUS_STROKE_OUTER);
    if forced {
        e.outline(sizes::FOCUS_STROKE_OUTER, off, outer).box_shadow(Shadow {
            x: 0.0,
            y: 0.0,
            blur: 0.0,
            spread: off,
            color: inner,
            inset: false,
        })
    } else {
        e
    }
}

/// `AcrylicBackgroundFillColorDefaultBrush` `FallbackColor` (the flyout/tooltip surface where
/// no backdrop is sampled): `#F9F9F9` light, `#2C2C2C` dark.
pub fn acrylic_fallback(theme: &Theme) -> Color {
    match theme.mode() {
        ThemeMode::Light => Color::rgb(0xF9, 0xF9, 0xF9),
        ThemeMode::Dark => Color::rgb(0x2C, 0x2C, 0x2C),
    }
}

/// The flyout `ThemeShadow` (elevation 32) approximated as one key shadow: 14%/26% black
/// light/dark, the opacities of WinUI's flyout drop shadow.
pub(crate) fn flyout_shadow(e: El, theme: &Theme) -> El {
    let a = match theme.mode() {
        ThemeMode::Light => 0.14,
        ThemeMode::Dark => 0.26,
    };
    e.shadow(8.0, 32.0, RColor::BLACK.alpha(a))
}

type Act = Box<dyn FnOnce(El) -> El>;

/// Deferred event handlers: generic over the app's state type at the call site, applied to the
/// interactive element at build time, dropped entirely when the control is disabled.
#[derive(Default)]
pub(crate) struct Acts(Vec<Act>);

impl Acts {
    pub fn push(&mut self, f: impl FnOnce(El) -> El + 'static) {
        self.0.push(Box::new(f));
    }
    pub fn apply(self, mut e: El, enabled: bool) -> El {
        if enabled {
            for f in self.0 {
                e = f(e);
            }
        }
        e
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Builder fields every control shares.
#[derive(Default)]
pub(crate) struct Common {
    pub enabled: bool,
    pub state: VisualState,
    pub focused: bool,
    pub key: Option<String>,
}

impl Common {
    pub fn new() -> Self {
        Self { enabled: true, ..Default::default() }
    }
    pub fn state(&self) -> VisualState {
        effective(self.enabled, self.state)
    }
    pub fn keyed(&self, e: El) -> El {
        match &self.key {
            Some(k) => e.key(k.clone()),
            None => e,
        }
    }
}

/// Implements the common builder setters (`enabled`, `state`, `focused`, `key`) for a control
/// with a `common: Common` field.
macro_rules! common_setters {
    ($t:ty) => {
        impl $t {
            /// `IsEnabled`. A disabled control draws its `*Disabled` brushes and drops handlers.
            pub fn enabled(mut self, on: bool) -> Self {
                self.common.enabled = on;
                self
            }
            /// Force a visual state (see [`crate::VisualState`]).
            pub fn state(mut self, s: $crate::VisualState) -> Self {
                self.common.state = s;
                self
            }
            /// Draw the keyboard focus visual now.
            pub fn focused(mut self, on: bool) -> Self {
                self.common.focused = on;
                self
            }
            /// Rune element key (for tests, anchoring flyouts and focus).
            pub fn key(mut self, k: impl Into<String>) -> Self {
                self.common.key = Some(k.into());
                self
            }
        }
    };
}
pub(crate) use common_setters;
