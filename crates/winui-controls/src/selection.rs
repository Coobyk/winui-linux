//! Selection: `CheckBox`, `RadioButton`, `ToggleSwitch`, `RatingControl` (WinUI Gallery ›
//! Basic input). Slider and ColorPicker live in `range`.
use rune::{col, row, El};

use crate::impl_control;
use crate::shim::icons::Symbol;
use crate::shim::theme;
use crate::shim::theme::{sizes, Theme};
use crate::style::{self, focus_rect, glyph, hover_layer, layers, layers3, pinned, rc, Hook, VisualState, R};

/// Tri-state value (`IsThreeState`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

/// `CheckBox` — Gallery "CheckBox": 20px box, 4px corners, 8px label gap, 32px row.
#[derive(Clone, Default)]
pub struct CheckBox {
    label: String,
    state: CheckState,
    enabled: bool,
    three_state: bool,
    key: Option<String>,
    toggle: Hook<CheckState>,
}

impl CheckBox {
    pub fn new(label: impl Into<String>) -> Self {
        CheckBox { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn checked(mut self, on: bool) -> Self {
        self.state = if on { CheckState::Checked } else { CheckState::Unchecked };
        self
    }
    pub fn state(mut self, s: CheckState) -> Self {
        self.state = s;
        self
    }
    pub fn three_state(mut self, on: bool) -> Self {
        self.three_state = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    /// `f(state, next)`: Unchecked → Checked → (Indeterminate →) Unchecked.
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, CheckState) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    fn next(&self) -> CheckState {
        match self.state {
            CheckState::Unchecked => CheckState::Checked,
            CheckState::Checked if self.three_state => CheckState::Indeterminate,
            _ => CheckState::Unchecked,
        }
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let on = self.state != CheckState::Unchecked;
        let en = self.enabled;
        let (rest, hover, pressed) = selector_fills(k, on, en);
        let mark_c = match (en, pinned()) {
            (false, _) => k.text_on_accent_fill_color_disabled,
            (true, Some(VisualState::Pressed)) => k.text_on_accent_fill_color_secondary,
            (true, _) => k.text_on_accent_fill_color_primary,
        };
        let mut bx = layers3((20.0, 20.0), R, rest, hover, pressed).center();
        bx = match self.state {
            CheckState::Checked => bx.child(checkmark(mark_c)),
            CheckState::Indeterminate => bx.child(rune::el().size(8.0, 1.5).radius(1.0).fill(rc(mark_c))),
            CheckState::Unchecked => bx,
        };
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let mut e = row()
            .min_h(sizes::CONTROL_HEIGHT)
            .gap(8.0)
            .center_y()
            .radius(R)
            .hover_group()
            .child(bx)
            .child(rune::text(self.label.clone()).font(style::body(t)).color(rc(fg)))
            .role("checkbox")
            .label(self.label.clone())
            .disabled(!en);
        e = match self.state {
            CheckState::Indeterminate => e.checked_mixed(),
            s => e.checked(s == CheckState::Checked),
        };
        if en {
            e = focus_rect(e, t, R);
            e = self.toggle.attach(e, self.next());
        }
        match &self.key {
            Some(key) => e.key(key.clone()),
            None => e,
        }
    }
}

/// `RadioButton` — Gallery "RadioButton": 20px ring, accent fill with a 12px dot when checked.
#[derive(Clone, Default)]
pub struct RadioButton {
    label: String,
    checked: bool,
    enabled: bool,
    key: Option<String>,
    select: Hook<()>,
}

impl RadioButton {
    pub fn new(label: impl Into<String>) -> Self {
        RadioButton { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn checked(mut self, on: bool) -> Self {
        self.checked = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.select.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let c = 10.0;
        let (rest, hover, pressed) = selector_fills(k, self.checked, en);
        let mut ring = layers3((20.0, 20.0), c, rest, hover, pressed).center();
        // RadioButtonCheckGlyphSize 12 / PointerOverSize 14 / PressedOverSize 10, stroked with
        // CircleElevationBorderBrush; an unchecked radio shows the 10px glyph while pressed.
        let dot = if en { k.text_on_accent_fill_color_primary } else { k.text_on_accent_fill_color_disabled };
        let d = |size: f32| {
            let g = &k.circle_elevation_border;
            rune::el()
                .absolute((20.0 - size) / 2.0, (20.0 - size) / 2.0)
                .size(size, size)
                .radius(size / 2.0)
                .fill(rc(dot))
                .border_gradient(1.0, rc(g.top(size)), rc(g.bottom(size)))
        };
        match (self.checked, en && pinned() == Some(VisualState::Pressed)) {
            (_, true) => ring = ring.child(d(10.0)),
            (true, false) => ring = ring.child(hover_layer(d(12.0), 1.0, 0.0)).child(hover_layer(d(14.0), 0.0, 1.0)),
            (false, false) => {}
        }
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let mut e = row()
            .min_h(sizes::CONTROL_HEIGHT)
            .gap(8.0)
            .center_y()
            .radius(R)
            .hover_group()
            .child(ring)
            .child(rune::text(self.label.clone()).font(style::body(t)).color(rc(fg)))
            .role("radio")
            .label(self.label.clone())
            .checked(self.checked)
            .disabled(!en);
        if en {
            e = focus_rect(e, t, R);
            e = self.select.attach(e, ());
        }
        match &self.key {
            Some(key) => e.key(key.clone()),
            None => e,
        }
    }
}

/// A vertical group of `RadioButton`s with a header — Gallery "RadioButtons".
#[derive(Clone, Default)]
pub struct RadioButtons {
    header: Option<String>,
    items: Vec<String>,
    selected: Option<usize>,
    enabled: bool,
    select: Hook<usize>,
}

impl RadioButtons {
    pub fn new<I: Into<String>>(items: impl IntoIterator<Item = I>) -> Self {
        RadioButtons { items: items.into_iter().map(Into::into).collect(), enabled: true, ..Default::default() }
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn selected(mut self, i: Option<usize>) -> Self {
        self.selected = i;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let mut c = col().role("radiogroup");
        if let Some(h) = &self.header {
            c = c.child(header(h, t, self.enabled));
        }
        for (i, item) in self.items.iter().enumerate() {
            let r = RadioButton::new(item.clone()).checked(self.selected == Some(i)).enabled(self.enabled).key(format!("radio:{item}"));
            c = c.child(if self.enabled { self.select.attach(r.view(t), i) } else { r.view(t) });
        }
        c
    }
}

/// A control header (`ControlHeaderThemeStyle`): body text, 8px above the control.
pub fn header(s: &str, t: &Theme, enabled: bool) -> El {
    let k = t.tokens();
    rune::text(s.to_owned()).font(style::body(t)).color(rc(if enabled { k.text_fill_color_primary } else { k.text_fill_color_disabled })).pb(8.0)
}

/// `ToggleSwitch` — Gallery "ToggleSwitch": 40×20 pill track, 12px knob (14px on hover),
/// On/Off content 12px to the right.
#[derive(Clone)]
pub struct ToggleSwitch {
    header: Option<String>,
    on: bool,
    enabled: bool,
    on_content: String,
    off_content: String,
    key: Option<String>,
    toggle: Hook<bool>,
}

impl Default for ToggleSwitch {
    fn default() -> Self {
        ToggleSwitch {
            header: None,
            on: false,
            enabled: true,
            on_content: "On".into(),
            off_content: "Off".into(),
            key: None,
            toggle: Hook::default(),
        }
    }
}

impl ToggleSwitch {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn content(mut self, on: impl Into<String>, off: impl Into<String>) -> Self {
        self.on_content = on.into();
        self.off_content = off.into();
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        // ToggleSwitchFill/Stroke{Off,On}{,PointerOver,Pressed,Disabled}.
        let (rest, hover, mut pressed) = selector_fills(k, self.on, en);
        if !self.on && en {
            pressed.1 = Some(k.control_strong_stroke_color_default);
        }
        let knob = match (self.on, en) {
            (false, true) => k.text_fill_color_secondary,
            (false, false) => k.text_fill_color_disabled,
            (true, true) => k.text_on_accent_fill_color_primary,
            (true, false) => k.text_on_accent_fill_color_disabled,
        };
        // SwitchKnob: 12x12 at rest, 14x14 PointerOver, 17x14 Pressed (stretching away
        // from the edge it is parked at).
        let knob_at = |w: f32, h: f32| {
            let inset = 4.0 - (h - 12.0) / 2.0;
            let x = if self.on { 40.0 - inset - w } else { inset };
            let e = rune::el().absolute(x, (20.0 - h) / 2.0).size(w, h).radius(h / 2.0).fill(rc(knob));
            if self.on && en {
                let g = &k.circle_elevation_border;
                e.border_gradient(1.0, rc(g.top(h)), rc(g.bottom(h)))
            } else {
                e
            }
        };
        let track = if en && pinned() == Some(VisualState::Pressed) {
            layers3((40.0, 20.0), 10.0, rest, hover, pressed).child(knob_at(17.0, 14.0))
        } else {
            layers((40.0, 20.0), 10.0, rest, hover)
                .child(hover_layer(knob_at(12.0, 12.0), 1.0, 0.0))
                .child(hover_layer(knob_at(14.0, 14.0), 0.0, 1.0))
        };
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let content = if self.on { &self.on_content } else { &self.off_content };
        let mut sw = row()
            .min_h(sizes::CONTROL_HEIGHT)
            .min_w(154.0)
            .gap(12.0)
            .center_y()
            .radius(R)
            .hover_group()
            .child(track)
            .child(rune::text(content.clone()).font(style::body(t)).color(rc(fg)))
            .role("switch")
            .label(self.header.clone().unwrap_or_else(|| content.clone()))
            .checked(self.on)
            .disabled(!en);
        if en {
            sw = focus_rect(sw, t, R);
            sw = self.toggle.attach(sw, !self.on);
        }
        if let Some(key) = &self.key {
            sw = sw.key(key.clone());
        }
        match &self.header {
            Some(h) => col().child(header(h, t, en)).child(sw),
            None => sw,
        }
    }
}

/// `RatingControl` — Gallery "RatingControl": five 20px stars, 8px apart, accent when rated.
#[derive(Clone)]
pub struct RatingControl {
    value: f32,
    max: u32,
    enabled: bool,
    caption: Option<String>,
    key: String,
    rate: Hook<u32>,
}

impl Default for RatingControl {
    fn default() -> Self {
        RatingControl { value: 0.0, max: 5, enabled: true, caption: None, key: "rating".into(), rate: Hook::default() }
    }
}

impl RatingControl {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn value(mut self, v: f32) -> Self {
        self.value = v;
        self
    }
    pub fn max(mut self, n: u32) -> Self {
        self.max = n;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn caption(mut self, s: impl Into<String>) -> Self {
        self.caption = Some(s.into());
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    pub fn on_rate<S: 'static>(mut self, f: impl Fn(&mut S, u32) + 'static) -> Self {
        self.rate.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let filled = if en { k.accent_fill_color_default } else { k.accent_fill_color_disabled };
        let empty = if en { k.control_strong_stroke_color_default } else { k.control_strong_stroke_color_disabled };
        let mut r = row().gap(8.0).center_y().min_h(sizes::CONTROL_HEIGHT).role("slider").label("Rating").range_value(self.value as f64, 0.0, self.max as f64);
        for i in 0..self.max {
            let on = self.value >= i as f32 + 0.5;
            let star = if on { glyph(Symbol::StarFilled, 20.0, filled) } else { glyph(Symbol::Star, 20.0, empty) };
            let mut cell = rune::el().size(20.0, 20.0).radius(R).child(star).key(format!("{}:{}", self.key, i + 1));
            if en {
                cell = self.rate.attach(cell.hover(rc(k.subtle_fill_color_secondary)), i + 1);
            }
            r = r.child(cell);
        }
        if let Some(c) = &self.caption {
            r = r.child(rune::text(c.clone()).font(style::caption(t)).color(rc(k.text_fill_color_secondary)).pl(4.0));
        }
        if en { focus_rect(r.key(self.key.clone()), t, R) } else { r.key(self.key.clone()) }
    }
}


/// The CheckBox glyph (`CheckBoxCheckedGlyph`, E73E at 12px), stroked.
pub(crate) fn checkmark(c: theme::Color) -> El {
    rune::canvas(rune::Shape::new().stroke(vec![[1.5, 6.5], [4.5, 9.5], [10.5, 2.5]], 1.4, rc(c)).view_box(12.0, 12.0)).size(12.0, 12.0)
}

impl_control!(CheckBox, RadioButton, RadioButtons, ToggleSwitch, RatingControl);

/// Box/ring/track fill + stroke for (Rest, PointerOver, Pressed) of CheckBox, RadioButton
/// and ToggleSwitch, which share `ControlAltFillColor*` + `ControlStrongStroke*` off and
/// `AccentFillColor*` on.
type Layer = (theme::Color, Option<theme::Color>);
fn selector_fills(k: &crate::shim::theme::Tokens, on: bool, en: bool) -> (Layer, Layer, Layer) {
    match (on, en) {
        (false, true) => (
            (k.control_alt_fill_color_secondary, Some(k.control_strong_stroke_color_default)),
            (k.control_alt_fill_color_tertiary, Some(k.control_strong_stroke_color_default)),
            (k.control_alt_fill_color_quarternary, Some(k.control_strong_stroke_color_disabled)),
        ),
        (false, false) => {
            let d = (k.control_alt_fill_color_disabled, Some(k.control_strong_stroke_color_disabled));
            (d, d, d)
        }
        (true, true) => (
            (k.accent_fill_color_default, None),
            (k.accent_fill_color_secondary, None),
            (k.accent_fill_color_tertiary, None),
        ),
        (true, false) => {
            let d = (k.accent_fill_color_disabled, None);
            (d, d, d)
        }
    }
}
