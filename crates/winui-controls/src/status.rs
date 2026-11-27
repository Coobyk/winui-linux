//! Status and info: `InfoBar`, `ProgressBar`, `ProgressRing`, `ToolTip`, `TeachingTip`-lite
//! (WinUI Gallery › Status & info, Dialogs & flyouts › TeachingTip).
use std::f32::consts::{PI, TAU};

use rune::{col, row, El, Shape};

use crate::buttons::Button;
use crate::impl_control;
use crate::shim::icons::Symbol;
use crate::shim::theme::{self, Theme};
use crate::style::{self, flyout_surface, focus_rect, glyph, rc, Hook, R};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Severity {
    #[default]
    Informational,
    Success,
    Warning,
    Error,
}

impl Severity {
    /// (icon colour, background, glyph) — `InfoBar*SeverityIconBackground` & friends.
    fn look(self, t: &Theme) -> (theme::Color, theme::Color, Symbol) {
        let k = t.tokens();
        match self {
            Severity::Informational => (k.system_fill_color_attention, k.system_fill_color_attention_background, Symbol::Info),
            Severity::Success => (k.system_fill_color_success, k.system_fill_color_success_background, Symbol::CheckmarkCircle),
            Severity::Warning => (k.system_fill_color_caution, k.system_fill_color_caution_background, Symbol::Warning),
            Severity::Error => (k.system_fill_color_critical, k.system_fill_color_critical_background, Symbol::ErrorCircle),
        }
    }
}

/// A subtle square close button (`InfoBar`/`TeachingTip` close).
fn close_button(t: &Theme, size: f32, key: String) -> El {
    let k = t.tokens();
    let b = rune::el()
        .size(size, size)
        .center()
        .radius(R)
        .fill(rc(k.subtle_fill_color_transparent))
        .hover_fill(rc(k.subtle_fill_color_secondary))
        .press_fill(rc(k.subtle_fill_color_tertiary))
        .press_color(rc(k.text_fill_color_tertiary))
        .child(glyph(Symbol::Dismiss, 16.0, k.text_fill_color_primary))
        .role("button")
        .label("Close")
        .key(key);
    focus_rect(b, t, R)
}

/// `InfoBar` — Gallery "InfoBar": severity-tinted card, 16px glyph, bold title + message on
/// one line, optional action button and a 38px close button.
#[derive(Clone, Default)]
pub struct InfoBar {
    severity: Severity,
    title: String,
    message: String,
    closable: bool,
    action: Option<Button>,
    key: Option<String>,
    close: Hook<()>,
}

impl InfoBar {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        InfoBar { title: title.into(), message: message.into(), closable: true, ..Default::default() }
    }
    pub fn severity(mut self, s: Severity) -> Self {
        self.severity = s;
        self
    }
    pub fn closable(mut self, on: bool) -> Self {
        self.closable = on;
        self
    }
    pub fn action(mut self, b: Button) -> Self {
        self.action = Some(b);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_close<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.close.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let (icon_c, bg, sym) = self.severity.look(t);
        let key = self.key.clone().unwrap_or_else(|| format!("infobar:{}", self.title));
        let mut bar = row()
            .w_full()
            .min_h(48.0)
            .pl(16.0)
            .pr(if self.closable { 4.0 } else { 16.0 })
            .gap(16.0)
            .center_y()
            .radius(R)
            .fill(rc(bg))
            .border(1.0, rc(k.card_stroke_color_default))
            .child(glyph(sym, 16.0, icon_c))
            .child(
                row().gap(12.0).py(14.0).grow(1.0).flex_wrap()
                    .child(rune::text(self.title.clone()).font(style::body_strong(t)).color(rc(k.text_fill_color_primary)))
                    .child(rune::text(self.message.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)).wrap()),
            )
            .role(if self.severity == Severity::Error { "alert" } else { "status" })
            .label(format!("{}: {}", self.title, self.message))
            .key(key.clone());
        if let Some(a) = &self.action {
            bar = bar.child(a.view(t));
        }
        if self.closable {
            bar = bar.child(self.close.attach(close_button(t, 38.0, format!("{key}:close")), ()));
        }
        bar
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressState {
    #[default]
    Running,
    Paused,
    Error,
}

/// `ProgressBar` — Gallery "ProgressBar": 3px rounded accent indicator over a 1px track;
/// paused draws caution, error draws critical; indeterminate shows a 40% segment.
#[derive(Clone)]
pub struct ProgressBar {
    value: Option<f32>,
    state: ProgressState,
    width: f32,
}

impl Default for ProgressBar {
    fn default() -> Self {
        ProgressBar { value: None, state: ProgressState::Running, width: 200.0 }
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self::default()
    }
    /// Determinate progress, 0–100. Unset is indeterminate.
    pub fn value(mut self, v: f32) -> Self {
        self.value = Some(v.clamp(0.0, 100.0));
        self
    }
    pub fn state(mut self, s: ProgressState) -> Self {
        self.state = s;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let color = match self.state {
            ProgressState::Running => k.accent_fill_color_default,
            ProgressState::Paused => k.system_fill_color_caution,
            ProgressState::Error => k.system_fill_color_critical,
        };
        let w = self.width;
        let (x, len) = match self.value {
            Some(v) => (0.0, w * v / 100.0),
            None if self.state == ProgressState::Running => (w * 0.3, w * 0.4),
            None => (0.0, w),
        };
        let track = rune::el().absolute(0.0, 1.0).size(w, 1.0).fill(rc(k.control_strong_stroke_color_default));
        let bar = rune::el().absolute(x, 0.0).size(len, 3.0).radius(1.5).fill(rc(color));
        let mut e = rune::el().size(w, 3.0).child(track).child(bar).role("progressbar").label("Progress");
        if let Some(v) = self.value {
            e = e.range_value(v as f64, 0.0, 100.0);
        }
        e
    }
}

/// `ProgressRing` — Gallery "ProgressRing": a 4px accent arc, 32px by default; determinate
/// sweeps from 12 o'clock, indeterminate shows a quarter arc.
#[derive(Clone)]
pub struct ProgressRing {
    value: Option<f32>,
    size: f32,
    state: ProgressState,
}

impl Default for ProgressRing {
    fn default() -> Self {
        ProgressRing { value: None, size: 32.0, state: ProgressState::Running }
    }
}

impl ProgressRing {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn value(mut self, v: f32) -> Self {
        self.value = Some(v.clamp(0.0, 100.0));
        self
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn state(mut self, s: ProgressState) -> Self {
        self.state = s;
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let color = match self.state {
            ProgressState::Running => k.accent_fill_color_default,
            ProgressState::Paused => k.system_fill_color_caution,
            ProgressState::Error => k.system_fill_color_critical,
        };
        let s = self.size;
        let stroke = (s / 8.0).max(2.0);
        let (start, sweep) = match self.value {
            Some(v) => (-PI / 2.0, TAU * v / 100.0),
            None => (-PI / 2.0 + PI / 4.0, PI / 2.0),
        };
        let mut shape = Shape::new();
        if self.value.is_some() {
            shape = shape.stroke(arc(s, stroke, 0.0, TAU), stroke, rc(k.control_alt_fill_color_quarternary));
        }
        if sweep > 0.0 {
            shape = shape.stroke(arc(s, stroke, start, sweep), stroke, rc(color));
        }
        let mut e = rune::canvas(shape).size(s, s).role("progressbar").label("Progress");
        if let Some(v) = self.value {
            e = e.range_value(v as f64, 0.0, 100.0);
        }
        e
    }
}

fn arc(size: f32, stroke: f32, start: f32, sweep: f32) -> Vec<[f32; 2]> {
    let r = (size - stroke) / 2.0;
    let c = size / 2.0;
    let n = ((sweep.abs() / TAU) * 64.0).ceil().max(2.0) as usize;
    (0..=n).map(|i| {
        let a = start + sweep * i as f32 / n as f32;
        [c + r * a.cos(), c + r * a.sin()]
    }).collect()
}

/// `ToolTip` — Gallery "ToolTip": 12px text on an acrylic surface, 4px corners, 9/6px padding,
/// 320px max width; shown above the element keyed `anchor`.
#[derive(Clone)]
pub struct ToolTip {
    text: String,
    anchor: Option<String>,
}

impl ToolTip {
    pub fn new(text: impl Into<String>) -> Self {
        ToolTip { text: text.into(), anchor: None }
    }
    pub fn anchor(mut self, key: impl Into<String>) -> Self {
        self.anchor = Some(key.into());
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let tip = rune::el()
            .max_w(320.0)
            .pl(9.0)
            .pr(9.0)
            .pt(6.0)
            .pb(8.0)
            .radius(R)
            .fill(rc(style::acrylic(t)))
            .border(1.0, rc(k.surface_stroke_color_flyout))
            .shadow(4.0, 8.0, rune::Color::BLACK.alpha(if t.is_dark() { 0.26 } else { 0.14 }))
            .child(rune::text(self.text.clone()).font(style::caption(t)).color(rc(k.text_fill_color_primary)).wrap())
            .role("tooltip");
        match &self.anchor {
            Some(a) => tip.anchored(a.clone(), rune::Side::Top, rune::Align::Center, 4.0).raise(),
            None => tip,
        }
    }
}

/// `TeachingTip`-lite — Gallery "TeachingTip": a 360px flyout with title, subtitle, close
/// button and up to two actions, anchored below the element keyed `anchor`. No beak (see
/// README gaps).
#[derive(Clone, Default)]
pub struct TeachingTip {
    title: String,
    subtitle: String,
    icon: Option<Symbol>,
    anchor: Option<String>,
    action: Option<Button>,
    close_text: Option<String>,
    close: Hook<()>,
}

impl TeachingTip {
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        TeachingTip { title: title.into(), subtitle: subtitle.into(), ..Default::default() }
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn anchor(mut self, key: impl Into<String>) -> Self {
        self.anchor = Some(key.into());
        self
    }
    /// The accent `ActionButton`.
    pub fn action(mut self, b: Button) -> Self {
        self.action = Some(b.accent(true));
        self
    }
    /// A text `CloseButton` instead of the corner ✕.
    pub fn close_text(mut self, s: impl Into<String>) -> Self {
        self.close_text = Some(s.into());
        self
    }
    pub fn on_close<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.close.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let mut head = row().gap(12.0).items_start();
        if let Some(i) = self.icon {
            head = head.child(glyph(i, 20.0, k.accent_text_fill_color_primary));
        }
        head = head.child(
            col().grow(1.0).gap(4.0)
                .child(rune::text(self.title.clone()).font(style::body_strong(t)).color(rc(k.text_fill_color_primary)).wrap())
                .child(rune::text(self.subtitle.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)).wrap()),
        );
        if self.close_text.is_none() {
            head = head.child(self.close.attach(close_button(t, 28.0, "teachingtip:close".into()), ()));
        }
        let mut tip = flyout_surface(t).w(360.0).padding(12.0).gap(16.0).child(head).role("dialog").label(self.title.clone());
        if self.action.is_some() || self.close_text.is_some() {
            let mut actions = row().gap(8.0).justify_end();
            if let Some(a) = &self.action {
                actions = actions.child(a.view(t).grow(1.0));
            }
            if let Some(c) = &self.close_text {
                actions = actions.child(self.close.attach(Button::new(c.clone()).key("teachingtip:close").view(t), ()).grow(1.0));
            }
            tip = tip.child(actions);
        }
        match &self.anchor {
            Some(a) => tip.anchored(a.clone(), rune::Side::Bottom, rune::Align::Center, 8.0).raise(),
            None => tip,
        }
    }
}

impl_control!(InfoBar, ProgressBar, ProgressRing, ToolTip, TeachingTip);
