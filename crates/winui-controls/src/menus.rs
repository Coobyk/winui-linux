//! Menus and flyouts: `MenuFlyout`, `Flyout`, `ComboBox`, `CommandBar`-lite (WinUI Gallery ›
//! Menus & toolbars, Dialogs & flyouts › Flyout, Basic input › ComboBox).
use std::rc::Rc;

use rune::{col, row, El};

use crate::impl_control;
use crate::selection::{checkmark, header};
use crate::shim::icons::Symbol;
use crate::shim::theme::{sizes, Theme};
use crate::style::{self, flyout_surface, focus_rect, glyph, rc, Hook, R};

/// A subtle 32px list row inset 4px inside its popup (menu items, ComboBox items,
/// suggestions): `SubtleFillColorSecondary` on hover, `Tertiary` pressed, a fill when selected.
pub fn list_row(t: &Theme, enabled: bool, selected: bool) -> El {
    let k = t.tokens();
    let mut r = row().min_h(sizes::CONTROL_HEIGHT).mx(4.0).my(2.0).px(12.0).gap(12.0).center_y().radius(R);
    r = r.fill(rc(if selected { k.subtle_fill_color_secondary } else { k.subtle_fill_color_transparent }));
    if enabled {
        r = r
            .hover_fill(rc(if selected { k.subtle_fill_color_tertiary } else { k.subtle_fill_color_secondary }))
            .press_fill(rc(k.subtle_fill_color_tertiary))
            .press_color(rc(k.text_fill_color_secondary));
        r = focus_rect(r, t, R);
    }
    r.disabled(!enabled)
}

/// The 3×16 accent pill that marks a selected list item / nav item.
pub fn selection_pill(t: &Theme, x: f32, y: f32) -> El {
    rune::el().absolute(x, y).size(3.0, 16.0).radius(1.5).fill(rc(t.tokens().accent_fill_color_default))
}

#[derive(Clone, Debug)]
pub enum MenuItem {
    Item { text: String, icon: Option<Symbol>, accelerator: Option<String>, enabled: bool },
    Toggle { text: String, checked: bool },
    Separator,
}

impl MenuItem {
    pub fn item(text: impl Into<String>) -> Self {
        MenuItem::Item { text: text.into(), icon: None, accelerator: None, enabled: true }
    }
    pub fn icon(self, s: Symbol) -> Self {
        match self {
            MenuItem::Item { text, accelerator, enabled, .. } => MenuItem::Item { text, icon: Some(s), accelerator, enabled },
            m => m,
        }
    }
    pub fn accelerator(self, a: impl Into<String>) -> Self {
        match self {
            MenuItem::Item { text, icon, enabled, .. } => MenuItem::Item { text, icon, accelerator: Some(a.into()), enabled },
            m => m,
        }
    }
    pub fn enabled(self, on: bool) -> Self {
        match self {
            MenuItem::Item { text, icon, accelerator, .. } => MenuItem::Item { text, icon, accelerator, enabled: on },
            m => m,
        }
    }
    pub fn toggle(text: impl Into<String>, checked: bool) -> Self {
        MenuItem::Toggle { text: text.into(), checked }
    }
}

/// `MenuFlyout` — Gallery "MenuFlyout": an 8px-radius flyout of 32px items with optional icons,
/// toggle checks and right-aligned accelerators.
#[derive(Clone, Default)]
pub struct MenuFlyout {
    items: Vec<MenuItem>,
    min_width: f32,
    key: Option<String>,
    select: Hook<usize>,
}

impl MenuFlyout {
    pub fn new(items: impl IntoIterator<Item = MenuItem>) -> Self {
        MenuFlyout { items: items.into_iter().collect(), min_width: 120.0, ..Default::default() }
    }
    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = w;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    /// `f(state, item_index)`.
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let has_icons = self.items.iter().any(|m| matches!(m, MenuItem::Item { icon: Some(_), .. } | MenuItem::Toggle { .. }));
        let mut menu = flyout_surface(t).min_w(self.min_width).py(2.0).role("menu");
        for (i, m) in self.items.iter().enumerate() {
            let e = match m {
                MenuItem::Separator => rune::el().h(1.0).w_full().my(2.0).fill(rc(k.divider_stroke_color_default)),
                MenuItem::Item { text, icon, accelerator, enabled } => {
                    let fg = if *enabled { k.text_fill_color_primary } else { k.text_fill_color_disabled };
                    let mut r = list_row(t, *enabled, false).role("menuitem").label(text.clone()).key(format!("menu:{text}"));
                    if let Some(s) = icon {
                        r = r.child(glyph(*s, 16.0, fg));
                    } else if has_icons {
                        r = r.child(rune::el().size(16.0, 16.0));
                    }
                    r = r.child(rune::text(text.clone()).font(style::body(t)).color(rc(fg)).grow(1.0));
                    if let Some(a) = accelerator {
                        r = r.child(rune::text(a.clone()).font(style::caption(t)).color(rc(k.text_fill_color_secondary)).pl(12.0));
                    }
                    if *enabled { self.select.attach(r, i) } else { r }
                }
                MenuItem::Toggle { text, checked } => {
                    let mark = if *checked { checkmark(k.text_fill_color_primary).size(16.0, 16.0) } else { rune::el().size(16.0, 16.0) };
                    let r = list_row(t, true, false)
                        .role("menuitemcheckbox")
                        .checked(*checked)
                        .label(text.clone())
                        .key(format!("menu:{text}"))
                        .child(mark)
                        .child(rune::text(text.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)));
                    self.select.attach(r, i)
                }
            };
            menu = menu.child(e);
        }
        match &self.key {
            Some(key) => menu.key(key.clone()),
            None => menu,
        }
    }
}

/// `Flyout` — Gallery "Flyout": arbitrary content on an 8px-radius flyout surface, 16px
/// padding, anchored below an element keyed `anchor`.
#[derive(Clone)]
pub struct Flyout {
    content: Rc<dyn Fn(&Theme) -> El>,
    anchor: Option<String>,
}

impl Flyout {
    pub fn new(content: impl Fn(&Theme) -> El + 'static) -> Self {
        Flyout { content: Rc::new(content), anchor: None }
    }
    /// Position below the element keyed `key` (out of flow).
    pub fn anchor(mut self, key: impl Into<String>) -> Self {
        self.anchor = Some(key.into());
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let f = flyout_surface(t).padding(16.0).gap(12.0).child((self.content)(t)).role("dialog");
        match &self.anchor {
            Some(k) => f.anchored(k.clone(), rune::Side::Bottom, rune::Align::Start, 4.0),
            None => f,
        }
    }
}

/// `ComboBox` — Gallery "ComboBox": a 32px standard-button face with the selection and a
/// chevron; when `open`, a list flyout whose selected item carries the accent pill.
#[derive(Clone)]
pub struct ComboBox {
    items: Vec<String>,
    selected: Option<usize>,
    placeholder: String,
    header: Option<String>,
    width: f32,
    open: bool,
    enabled: bool,
    key: String,
    select: Hook<usize>,
    toggle: Hook<bool>,
}

impl ComboBox {
    pub fn new<I: Into<String>>(items: impl IntoIterator<Item = I>) -> Self {
        ComboBox {
            items: items.into_iter().map(Into::into).collect(),
            selected: None,
            placeholder: String::new(),
            header: None,
            width: 200.0,
            open: false,
            enabled: true,
            key: "combobox".into(),
            select: Hook::default(),
            toggle: Hook::default(),
        }
    }
    pub fn selected(mut self, i: Option<usize>) -> Self {
        self.selected = i;
        self
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn open(mut self, on: bool) -> Self {
        self.open = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let recipe = style::Recipe::standard(k);
        let (text, color) = match self.selected.and_then(|i| self.items.get(i)) {
            Some(s) => (s.clone(), recipe.fg(en)),
            None => (self.placeholder.clone(), if en { k.text_fill_color_secondary } else { k.text_fill_color_disabled }),
        };
        let mut face = row()
            .w(self.width)
            .h(sizes::CONTROL_HEIGHT)
            .pl(12.0)
            .pr(10.0)
            .pb(1.0)
            .center_y()
            .radius(R)
            .child(rune::text(text).font(style::body(t)).color(rc(color)).grow(1.0))
            .child(glyph(Symbol::ChevronDown, 12.0, if en { k.text_fill_color_secondary } else { k.text_fill_color_disabled }))
            .role("combobox")
            .label(self.header.clone().unwrap_or_else(|| self.placeholder.clone()))
            .expanded(self.open)
            .key(self.key.clone());
        face = recipe.paint(face, en);
        if en {
            face = self.toggle.attach(focus_rect(face, t, R), !self.open);
        }
        let mut out = col();
        if let Some(h) = &self.header {
            out = out.child(header(h, t, en));
        }
        out = out.child(face);
        if self.open && en {
            let mut list = flyout_surface(t).w(self.width).py(2.0).role("listbox");
            for (i, item) in self.items.iter().enumerate() {
                let sel = self.selected == Some(i);
                let mut r = list_row(t, true, sel)
                    .child(rune::text(item.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)))
                    .role("option")
                    .checked(sel)
                    .label(item.clone())
                    .key(format!("{}:item{i}", self.key));
                if sel {
                    r = r.child(selection_pill(t, 0.0, (sizes::CONTROL_HEIGHT - 16.0) / 2.0));
                }
                list = list.child(self.select.attach(r, i));
            }
            out = out.child(list.anchored(self.key.clone(), rune::Side::Bottom, rune::Align::Start, 4.0));
        }
        out
    }
}

#[derive(Clone, Debug)]
pub enum AppBarItem {
    Button { icon: Symbol, label: String, enabled: bool },
    Toggle { icon: Symbol, label: String, checked: bool },
    Separator,
}

impl AppBarItem {
    pub fn button(icon: Symbol, label: impl Into<String>) -> Self {
        AppBarItem::Button { icon, label: label.into(), enabled: true }
    }
    pub fn toggle(icon: Symbol, label: impl Into<String>, checked: bool) -> Self {
        AppBarItem::Toggle { icon, label: label.into(), checked }
    }
}

/// `CommandBar`-lite — Gallery "CommandBar" (`DefaultLabelPosition="Right"`): subtle 40px
/// app-bar buttons, separators and a "More" overflow button.
#[derive(Clone, Default)]
pub struct CommandBar {
    items: Vec<AppBarItem>,
    overflow: bool,
    press: Hook<usize>,
    more: Hook<()>,
}

impl CommandBar {
    pub fn new(items: impl IntoIterator<Item = AppBarItem>) -> Self {
        CommandBar { items: items.into_iter().collect(), overflow: true, ..Default::default() }
    }
    pub fn overflow(mut self, on: bool) -> Self {
        self.overflow = on;
        self
    }
    /// `f(state, item_index)`.
    pub fn on_press<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.press.press(f);
        self
    }
    pub fn on_more<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.more.press(move |s: &mut S, ()| f(s));
        self
    }
    fn app_button(t: &Theme, icon: Symbol, label: Option<&str>, enabled: bool, checked: bool) -> El {
        let k = t.tokens();
        let recipe = if checked { style::Recipe::accent(k) } else { style::Recipe::subtle(k) };
        let fg = recipe.fg(enabled);
        let mut b = row().h(40.0).px(12.0).gap(8.0).center().radius(R).child(glyph(icon, 16.0, fg));
        if let Some(l) = label {
            b = b.child(rune::text(l.to_owned()).font(style::body(t)).color(rc(fg)));
        }
        let mut b = recipe.paint(b, enabled).role("button").label(label.unwrap_or("More").to_owned()).disabled(!enabled);
        if checked {
            b = b.checked(true);
        }
        if enabled { focus_rect(b, t, R) } else { b }
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let mut bar = row().gap(4.0).center_y().padding(4.0).radius(R_BAR).role("toolbar");
        for (i, item) in self.items.iter().enumerate() {
            bar = bar.child(match item {
                AppBarItem::Separator => rune::el().size(1.0, 24.0).mx(4.0).fill(rc(k.divider_stroke_color_default)),
                AppBarItem::Button { icon, label, enabled } => {
                    let b = Self::app_button(t, *icon, Some(label), *enabled, false).key(format!("appbar:{label}"));
                    if *enabled { self.press.attach(b, i) } else { b }
                }
                AppBarItem::Toggle { icon, label, checked } => {
                    self.press.attach(Self::app_button(t, *icon, Some(label), true, *checked).key(format!("appbar:{label}")), i)
                }
            });
        }
        if self.overflow {
            bar = bar.child(self.more.attach(Self::app_button(t, Symbol::MoreHorizontal, None, true, false).key("appbar:more"), ()));
        }
        bar
    }
}

const R_BAR: f32 = style::R_OVERLAY;

impl_control!(MenuFlyout, Flyout, ComboBox, CommandBar);
