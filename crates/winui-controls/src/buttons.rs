//! Buttons: `Button`, `HyperlinkButton`, `ToggleButton`, `RepeatButton`, `SplitButton`,
//! `DropDownButton` (WinUI Gallery › Basic input › Button … DropDownButton).
use rune::{col, row, El, Phase};

use crate::shim::icons::Symbol;
use crate::shim::theme::{sizes, Theme};
use crate::style::{self, focus_rect, glyph, rc, Hook, Recipe, R};
use crate::{impl_control, menus::MenuFlyout};

/// Face of a button: 32px tall, 11px side padding, 8px icon gap, 4px corners, focus rect.
fn face(t: &Theme, recipe: Recipe, enabled: bool, icon: Option<Symbol>, label: &str, trailing: Option<El>) -> El {
    let fg = recipe.fg(enabled);
    let mut r = row().h(sizes::CONTROL_HEIGHT).px(11.0).pb(1.0).gap(8.0).center().radius(R);
    if let Some(i) = icon {
        r = r.child(glyph(i, 16.0, fg));
    }
    if !label.is_empty() {
        r = r.child(rune::text(label.to_owned()).font(style::body(t)).color(rc(fg)));
    }
    if let Some(tr) = trailing {
        r = r.child(tr);
    }
    let r = recipe.paint(r, enabled).role("button").label(label.to_owned()).disabled(!enabled);
    if enabled { focus_rect(r, t, R) } else { r }
}

fn keyed(e: El, key: &Option<String>) -> El {
    match key {
        Some(k) => e.key(k.clone()),
        None => e,
    }
}

/// `Button` — Gallery "Button" (standard and `AccentButtonStyle`).
#[derive(Clone, Default)]
pub struct Button {
    label: String,
    icon: Option<Symbol>,
    accent: bool,
    enabled: bool,
    width: Option<f32>,
    key: Option<String>,
    press: Hook<()>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Button { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn accent(mut self, on: bool) -> Self {
        self.accent = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_press<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.press.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let recipe = if self.accent { Recipe::accent(k) } else { Recipe::standard(k) };
        let mut e = face(t, recipe, self.enabled, self.icon, &self.label, None);
        if let Some(w) = self.width {
            e = e.w(w);
        }
        if self.enabled {
            e = self.press.attach(e, ());
        }
        keyed(e, &self.key)
    }
}

/// `HyperlinkButton` — Gallery "HyperlinkButton": accent text on a subtle surface.
#[derive(Clone, Default)]
pub struct HyperlinkButton {
    label: String,
    enabled: bool,
    key: Option<String>,
    press: Hook<()>,
}

impl HyperlinkButton {
    pub fn new(label: impl Into<String>) -> Self {
        HyperlinkButton { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_press<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.press.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let fg = if self.enabled { k.accent_text_fill_color_primary } else { k.accent_text_fill_color_disabled };
        let text = rune::text(self.label.clone())
            .font(style::body(t))
            .color(rc(fg))
            .group_color(rc(k.accent_text_fill_color_secondary));
        let mut e = row().h(sizes::CONTROL_HEIGHT).px(11.0).pb(1.0).center().radius(R).child(text);
        e = Recipe::subtle(k).paint(e, self.enabled).role("link").label(self.label.clone()).disabled(!self.enabled);
        if self.enabled {
            e = focus_rect(e.hover_group().press_color(rc(k.accent_text_fill_color_tertiary)), t, R);
            e = self.press.attach(e, ());
        }
        keyed(e, &self.key)
    }
}

/// `ToggleButton` — Gallery "ToggleButton": standard at rest, accent when checked.
#[derive(Clone, Default)]
pub struct ToggleButton {
    label: String,
    icon: Option<Symbol>,
    checked: bool,
    enabled: bool,
    key: Option<String>,
    toggle: Hook<bool>,
}

impl ToggleButton {
    pub fn new(label: impl Into<String>) -> Self {
        ToggleButton { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn checked(mut self, on: bool) -> Self {
        self.checked = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    /// `f(state, new_checked)`.
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let recipe = if self.checked { Recipe::accent(k) } else { Recipe::standard(k) };
        let mut e = face(t, recipe, self.enabled, self.icon, &self.label, None).checked(self.checked);
        if self.enabled {
            e = self.toggle.attach(e, !self.checked);
        }
        keyed(e, &self.key)
    }
}

/// `RepeatButton` — Gallery "RepeatButton": fires on pointer down (`ClickMode.Press`) and
/// again on every `interval` while held, driven by the app's clock (see README gaps).
#[derive(Clone, Default)]
pub struct RepeatButton {
    label: String,
    enabled: bool,
    key: Option<String>,
    repeat: Hook<()>,
}

impl RepeatButton {
    /// WinUI's `Delay` (ms) before repeating starts.
    pub const DELAY_MS: u32 = 250;
    /// WinUI's `Interval` (ms) between repeats.
    pub const INTERVAL_MS: u32 = 250;

    pub fn new(label: impl Into<String>) -> Self {
        RepeatButton { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    /// `f(state, held)`: `held` is true on pointer down and false on release, so the app can
    /// arm/disarm its repeat timer; `f` also fires once per down.
    pub fn on_repeat<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        let f = std::rc::Rc::new(f);
        self.repeat.raw(move |e, ()| {
            let f = f.clone();
            e.on_drag(move |s: &mut S, d: &mut rune::Drag, _tx: &mut rune::Tx| match d.phase {
                Phase::Down => f(s, true),
                Phase::Up => f(s, false),
                Phase::Move => {}
            })
        });
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let mut e = face(t, Recipe::standard(t.tokens()), self.enabled, None, &self.label, None);
        if self.enabled {
            e = self.repeat.attach(e, ());
        }
        keyed(e, &self.key)
    }
}

/// `DropDownButton` — Gallery "DropDownButton": a button with a trailing chevron that opens a
/// `MenuFlyout` below it.
#[derive(Clone)]
pub struct DropDownButton {
    label: String,
    icon: Option<Symbol>,
    enabled: bool,
    key: String,
    open: bool,
    menu: Option<MenuFlyout>,
    toggle: Hook<bool>,
}

impl DropDownButton {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        DropDownButton { key: format!("dropdown:{label}"), label, icon: None, enabled: true, open: false, menu: None, toggle: Hook::default() }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    /// Whether the flyout is showing (app-owned state).
    pub fn open(mut self, on: bool) -> Self {
        self.open = on;
        self
    }
    pub fn flyout(mut self, menu: MenuFlyout) -> Self {
        self.menu = Some(menu);
        self
    }
    /// `f(state, new_open)`.
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let recipe = Recipe::standard(t.tokens());
        let chevron = glyph(Symbol::ChevronDown, 12.0, recipe.fg(self.enabled));
        let mut e = face(t, recipe, self.enabled, self.icon, &self.label, Some(chevron)).pr(8.0).expanded(self.open).key(self.key.clone());
        if self.enabled {
            e = self.toggle.attach(e, !self.open);
        }
        with_flyout(e, t, self.open, self.menu.as_ref(), &self.key)
    }
}

/// Mounts `menu` below `anchor` when `open`.
pub(crate) fn with_flyout(anchor: El, t: &Theme, open: bool, menu: Option<&MenuFlyout>, key: &str) -> El {
    match (open, menu) {
        (true, Some(m)) => col().child(m.view(t).anchored(key.to_owned(), rune::Side::Bottom, rune::Align::Start, 4.0).raise()).child(anchor),
        _ => anchor,
    }
}

/// `SplitButton` — Gallery "SplitButton": a primary action and a chevron part that opens a
/// flyout, each with its own hover/press, sharing one elevation border.
#[derive(Clone)]
pub struct SplitButton {
    label: String,
    icon: Option<Symbol>,
    enabled: bool,
    accent: bool,
    key: String,
    open: bool,
    menu: Option<MenuFlyout>,
    press: Hook<()>,
    toggle: Hook<bool>,
}

impl SplitButton {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        SplitButton {
            key: format!("split:{label}"),
            label,
            icon: None,
            enabled: true,
            accent: false,
            open: false,
            menu: None,
            press: Hook::default(),
            toggle: Hook::default(),
        }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn accent(mut self, on: bool) -> Self {
        self.accent = on;
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    pub fn open(mut self, on: bool) -> Self {
        self.open = on;
        self
    }
    pub fn flyout(mut self, menu: MenuFlyout) -> Self {
        self.menu = Some(menu);
        self
    }
    pub fn on_press<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.press.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let recipe = if self.accent { Recipe::accent(k) } else { Recipe::standard(k) };
        let fg = recipe.fg(self.enabled);
        let part = |e: El| {
            let e = e.h(sizes::CONTROL_HEIGHT).center().fill(rc(recipe.fill.pick(self.enabled)));
            if self.enabled {
                let e = e.hover_fill(rc(recipe.fill.hover)).press_fill(rc(recipe.fill.pressed)).press_color(rc(recipe.fg.pressed));
                focus_rect(e, t, R)
            } else {
                e
            }
        };
        let mut primary = row().px(11.0).pb(1.0).gap(8.0);
        if let Some(i) = self.icon {
            primary = primary.child(glyph(i, 16.0, fg));
        }
        primary = primary.child(rune::text(self.label.clone()).font(style::body(t)).color(rc(fg)));
        let mut primary = part(primary).corners([R, 0.0, 0.0, R]).role("button").label(self.label.clone()).key(format!("{}:primary", self.key));
        let mut secondary = part(row().w(34.0).child(glyph(Symbol::ChevronDown, 12.0, fg)))
            .corners([0.0, R, R, 0.0])
            .role("button")
            .label("More options")
            .expanded(self.open)
            .key(format!("{}:secondary", self.key));
        if self.enabled {
            primary = self.press.attach(primary, ());
            secondary = self.toggle.attach(secondary, !self.open);
        }
        let divider = rune::el().w(1.0).h(sizes::CONTROL_HEIGHT).fill(rc(if self.accent {
            k.control_stroke_color_on_accent_default
        } else {
            k.control_stroke_color_default
        }));
        let border = recipe.border.expect("button recipes have borders");
        let group = row().radius(R).key(self.key.clone()).child(primary).child(divider).child(secondary);
        let group = if self.enabled {
            group.border_gradient(1.0, rc(border.top), rc(border.bottom))
        } else {
            group.border(1.0, rc(border.flat))
        };
        with_flyout(group.disabled(!self.enabled), t, self.open, self.menu.as_ref(), &self.key)
    }
}

/// Which caption button: minimize, maximize/restore or close.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptionKind {
    Minimize,
    Maximize,
    Close,
}

/// The Windows 11 title-bar caption buttons (46x32): subtle fill on hover, `#C42B1C` with a
/// white glyph for Close (`Theme::caption_buttons`).
#[derive(Clone, Default)]
pub struct CaptionButtons {
    maximized: bool,
    inactive: bool,
    press: Hook<CaptionKind>,
}

impl CaptionButtons {
    pub fn new() -> Self {
        Self::default()
    }
    /// Shows the restore glyph on the middle button.
    pub fn maximized(mut self, on: bool) -> Self {
        self.maximized = on;
        self
    }
    /// Window inactive: glyphs drop to `TextFillColorDisabled`.
    pub fn inactive(mut self, on: bool) -> Self {
        self.inactive = on;
        self
    }
    pub fn on_press<S: 'static>(mut self, f: impl Fn(&mut S, CaptionKind) + 'static) -> Self {
        self.press.press(f);
        self
    }

    /// One caption button.
    pub fn button(&self, t: &Theme, kind: CaptionKind) -> El {
        let c = t.caption_buttons();
        let close = kind == CaptionKind::Close;
        let (hover, pressed) = if close { (c.close_fill_pointer_over, c.close_fill_pressed) } else { (c.fill_pointer_over, c.fill_pressed) };
        let rest_fg = if self.inactive { c.foreground_inactive } else { c.foreground };
        let (hover_fg, pressed_fg) = if close { (c.close_foreground_pointer_over, c.close_foreground_pressed) } else { (c.foreground, c.foreground_pressed) };
        let fg = style::by_state(rest_fg, hover_fg, pressed_fg);
        let icon = match kind {
            CaptionKind::Minimize => rune::el().size(10.0, 1.0).fill(rc(fg)),
            CaptionKind::Maximize if self.maximized => rune::el()
                .size(10.0, 10.0)
                .child(rune::el().absolute(2.0, 0.0).size(8.0, 8.0).radius(1.5).border(1.0, rc(fg)))
                .child(rune::el().absolute(0.0, 2.0).size(8.0, 8.0).radius(1.5).fill(rc(t.tokens().solid_background_fill_color_base)).border(1.0, rc(fg))),
            CaptionKind::Maximize => rune::el().size(10.0, 10.0).radius(1.5).border(1.0, rc(fg)),
            CaptionKind::Close if style::pinned().is_none() => rune::el()
                .size(12.0, 12.0)
                .child(style::hover_layer(glyph(Symbol::Dismiss, 12.0, rest_fg).absolute(0.0, 0.0), 1.0, 0.0))
                .child(style::hover_layer(glyph(Symbol::Dismiss, 12.0, hover_fg).absolute(0.0, 0.0), 0.0, 1.0)),
            CaptionKind::Close => glyph(Symbol::Dismiss, 12.0, fg),
        };
        let label = match kind {
            CaptionKind::Minimize => "Minimize",
            CaptionKind::Maximize if self.maximized => "Restore",
            CaptionKind::Maximize => "Maximize",
            CaptionKind::Close => "Close",
        };
        let base = style::by_state(t.tokens().subtle_fill_color_transparent, hover, pressed);
        let mut e = rune::el().size(46.0, 32.0).center().fill(rc(base)).child(icon).role("button").label(label);
        if style::pinned().is_none() {
            e = e.hover_fill(rc(hover)).press_fill(rc(pressed));
            if close {
                e = e.hover_group();
                e = e.press_color(rc(pressed_fg));
            }
            e = self.press.attach(e, kind);
        }
        e
    }

    pub fn view(&self, t: &Theme) -> El {
        row()
            .child(self.button(t, CaptionKind::Minimize))
            .child(self.button(t, CaptionKind::Maximize))
            .child(self.button(t, CaptionKind::Close))
    }
}

impl_control!(Button, HyperlinkButton, ToggleButton, RepeatButton, DropDownButton, SplitButton, CaptionButtons);
