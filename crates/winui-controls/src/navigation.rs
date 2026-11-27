//! Navigation: `NavigationView`, `TabView`, `BreadcrumbBar`-lite (WinUI Gallery ›
//! Navigation).
use std::rc::Rc;

use rune::{col, row, El};

use crate::content::InfoBadge;
use crate::impl_control;
use crate::menus::selection_pill;
use crate::shim::icons::Symbol;
use crate::shim::theme::{sizes, Theme};
use crate::style::{self, focus_rect, glyph, rc, Hook, R, R_OVERLAY};

/// One `NavigationViewItem`.
#[derive(Clone, Debug)]
pub struct NavItem {
    pub icon: Symbol,
    pub label: String,
    pub badge: Option<u32>,
}

impl NavItem {
    pub fn new(icon: Symbol, label: impl Into<String>) -> Self {
        NavItem { icon, label: label.into(), badge: None }
    }
    pub fn badge(mut self, n: u32) -> Self {
        self.badge = Some(n);
        self
    }
}

/// `NavigationView` — Gallery "NavigationView" (`PaneDisplayMode="Left"`): a 320px pane (48px
/// compact) of 36px subtle items with the accent selection pill, a hamburger toggle, a
/// Settings footer item and the content on a `LayerFillColorDefault` surface with an 8px
/// top-left corner.
#[derive(Clone)]
pub struct NavigationView {
    items: Vec<NavItem>,
    footer: Vec<NavItem>,
    selected: usize,
    pane_open: bool,
    header: Option<String>,
    settings: bool,
    content: Rc<dyn Fn(&Theme) -> El>,
    select: Hook<usize>,
    toggle: Hook<bool>,
}

impl NavigationView {
    pub fn new(items: impl IntoIterator<Item = NavItem>) -> Self {
        NavigationView {
            items: items.into_iter().collect(),
            footer: Vec::new(),
            selected: 0,
            pane_open: true,
            header: None,
            settings: true,
            content: Rc::new(|_| rune::el()),
            select: Hook::default(),
            toggle: Hook::default(),
        }
    }
    pub fn footer(mut self, items: impl IntoIterator<Item = NavItem>) -> Self {
        self.footer = items.into_iter().collect();
        self
    }
    /// Index across `items`, then `footer`, then Settings.
    pub fn selected(mut self, i: usize) -> Self {
        self.selected = i;
        self
    }
    pub fn pane_open(mut self, on: bool) -> Self {
        self.pane_open = on;
        self
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn settings(mut self, on: bool) -> Self {
        self.settings = on;
        self
    }
    pub fn content(mut self, f: impl Fn(&Theme) -> El + 'static) -> Self {
        self.content = Rc::new(f);
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn on_toggle_pane<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    fn item(&self, t: &Theme, i: usize, it: &NavItem) -> El {
        let k = t.tokens();
        let sel = self.selected == i;
        let mut r = row()
            .h(36.0)
            .mx(4.0)
            .my(2.0)
            .pl(12.0)
            .pr(8.0)
            .gap(16.0)
            .center_y()
            .radius(R)
            .fill(rc(if sel { k.subtle_fill_color_secondary } else { k.subtle_fill_color_transparent }))
            .hover_fill(rc(if sel { k.subtle_fill_color_tertiary } else { k.subtle_fill_color_secondary }))
            .press_fill(rc(k.subtle_fill_color_tertiary))
            .press_color(rc(k.text_fill_color_secondary))
            .child(glyph(it.icon, 16.0, k.text_fill_color_primary));
        if self.pane_open {
            r = r.child(rune::text(it.label.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)).grow(1.0));
            if let Some(n) = it.badge {
                r = r.child(InfoBadge::value(n).view(t));
            }
        }
        if sel {
            r = r.child(selection_pill(t, 0.0, 10.0));
        }
        let r = focus_rect(r, t, R).role("tab").checked(sel).label(it.label.clone()).key(format!("nav:{}", it.label));
        self.select.attach(r, i)
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let width = if self.pane_open { sizes::NAV_PANE_OPEN } else { sizes::NAV_PANE_COMPACT };
        let toggle = row()
            .size(40.0, 36.0)
            .mx(4.0)
            .my(2.0)
            .center()
            .radius(R)
            .fill(rc(k.subtle_fill_color_transparent))
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .press_fill(rc(k.subtle_fill_color_tertiary))
            .child(glyph(Symbol::GlobalNavigation, 16.0, k.text_fill_color_primary))
            .role("button")
            .label(if self.pane_open { "Close Navigation" } else { "Open Navigation" })
            .expanded(self.pane_open)
            .key("nav:toggle");
        let toggle = self.toggle.attach(focus_rect(toggle, t, R), !self.pane_open);
        let mut top = col().pt(4.0).child(toggle);
        for (i, it) in self.items.iter().enumerate() {
            top = top.child(self.item(t, i, it));
        }
        let mut bottom = col().pb(4.0);
        let n = self.items.len();
        for (j, it) in self.footer.iter().enumerate() {
            bottom = bottom.child(self.item(t, n + j, it));
        }
        if self.settings {
            bottom = bottom.child(self.item(t, n + self.footer.len(), &NavItem::new(Symbol::Settings, "Settings")));
        }
        let pane = col().w(width).h_full().justify_between().child(top).child(bottom).role("navigation");
        let mut page = col()
            .grow(1.0)
            .h_full()
            .fill(rc(k.layer_fill_color_default))
            .border_sides([1.0, 1.0, 0.0, 0.0], rc(k.card_stroke_color_default))
            .corners([R_OVERLAY, 0.0, 0.0, 0.0])
            .padding(36.0)
            .pt(24.0)
            .gap(24.0)
            .key("nav:content");
        if let Some(h) = &self.header {
            page = page.child(rune::text(h.clone()).font(style::font(t.type_ramp().title)).color(rc(k.text_fill_color_primary)));
        }
        row().w_full().h_full().child(pane).child(page.child((self.content)(t)))
    }
}

/// One `TabViewItem`.
#[derive(Clone, Debug)]
pub struct Tab {
    pub header: String,
    pub icon: Option<Symbol>,
    pub closable: bool,
}

impl Tab {
    pub fn new(header: impl Into<String>) -> Self {
        Tab { header: header.into(), icon: None, closable: true }
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn closable(mut self, on: bool) -> Self {
        self.closable = on;
        self
    }
}

/// `TabView` — Gallery "TabView": 32px tabs (100–240px wide) whose selected tab joins the
/// content surface with 8px top corners, hover fills on the rest, 1px separators, close and
/// add buttons.
#[derive(Clone)]
pub struct TabView {
    tabs: Vec<Tab>,
    selected: usize,
    add: bool,
    content: Rc<dyn Fn(&Theme, usize) -> El>,
    select: Hook<usize>,
    close: Hook<usize>,
    add_tab: Hook<()>,
}

impl TabView {
    pub fn new(tabs: impl IntoIterator<Item = Tab>) -> Self {
        TabView {
            tabs: tabs.into_iter().collect(),
            selected: 0,
            add: true,
            content: Rc::new(|_, _| rune::el()),
            select: Hook::default(),
            close: Hook::default(),
            add_tab: Hook::default(),
        }
    }
    pub fn selected(mut self, i: usize) -> Self {
        self.selected = i;
        self
    }
    pub fn add_button(mut self, on: bool) -> Self {
        self.add = on;
        self
    }
    /// Content for the selected tab.
    pub fn content(mut self, f: impl Fn(&Theme, usize) -> El + 'static) -> Self {
        self.content = Rc::new(f);
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn on_close<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.close.press(f);
        self
    }
    pub fn on_add<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.add_tab.press(move |s: &mut S, ()| f(s));
        self
    }
    fn small_button(t: &Theme, s: Symbol, w: f32, h: f32, key: String, label: &str) -> El {
        let k = t.tokens();
        let b = rune::el()
            .size(w, h)
            .center()
            .radius(R)
            .fill(rc(k.subtle_fill_color_transparent))
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .press_fill(rc(k.subtle_fill_color_tertiary))
            .child(glyph(s, if h < 28.0 { 12.0 } else { 16.0 }, k.text_fill_color_secondary))
            .role("button")
            .label(label.to_owned())
            .key(key);
        focus_rect(b, t, R)
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let surface = k.solid_background_fill_color_tertiary;
        let mut strip = row().h(40.0).pl(8.0).pt(8.0).items_end().role("tablist");
        for (i, tab) in self.tabs.iter().enumerate() {
            let sel = i == self.selected;
            let mut r = row().h(32.0).min_w(100.0).max_w(240.0).pl(8.0).pr(4.0).gap(8.0).center_y().corners([R_OVERLAY, R_OVERLAY, 0.0, 0.0]);
            let fg = if sel { k.text_fill_color_primary } else { k.text_fill_color_secondary };
            if let Some(ic) = tab.icon {
                r = r.child(glyph(ic, 16.0, fg));
            }
            r = r.child(rune::text(tab.header.clone()).font(style::body(t)).color(rc(fg)).grow(1.0));
            if tab.closable {
                r = r.child(self.close.attach(Self::small_button(t, Symbol::Dismiss, 24.0, 24.0, format!("tab:{i}:close"), "Close tab"), i));
            }
            r = if sel {
                r.fill(rc(surface)).border_sides([1.0, 1.0, 1.0, 0.0], rc(k.card_stroke_color_default))
            } else {
                r.fill(rc(k.subtle_fill_color_transparent))
                    .hover_fill(rc(k.layer_on_mica_base_alt_fill_color_secondary))
                    .press_fill(rc(k.subtle_fill_color_tertiary))
            };
            let r = focus_rect(r, t, R).role("tab").checked(sel).label(tab.header.clone()).key(format!("tab:{i}"));
            strip = strip.child(self.select.attach(r, i));
            let next_sel = i + 1 == self.selected;
            if !sel && !next_sel && i + 1 < self.tabs.len() {
                strip = strip.child(rune::el().size(1.0, 16.0).mb(8.0).fill(rc(k.divider_stroke_color_default)));
            }
        }
        if self.add {
            strip = strip.child(row().h(32.0).pl(4.0).center_y().child(self.add_tab.attach(Self::small_button(t, Symbol::Add, 32.0, 24.0, "tab:add".into(), "Add tab"), ())));
        }
        let body = col()
            .w_full()
            .grow(1.0)
            .padding(16.0)
            .fill(rc(surface))
            .border_sides([0.0, 1.0, 0.0, 0.0], rc(k.card_stroke_color_default))
            .child((self.content)(t, self.selected));
        col().w_full().child(strip).child(body)
    }
}

/// `BreadcrumbBar`-lite — Gallery "BreadcrumbBar": items separated by chevrons; the last
/// (current) item is primary text, earlier ones are secondary and pressable.
#[derive(Clone, Default)]
pub struct BreadcrumbBar {
    items: Vec<String>,
    select: Hook<usize>,
}

impl BreadcrumbBar {
    pub fn new<I: Into<String>>(items: impl IntoIterator<Item = I>) -> Self {
        BreadcrumbBar { items: items.into_iter().map(Into::into).collect(), select: Hook::default() }
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let mut bar = row().gap(4.0).center_y().role("navigation").label("Breadcrumb");
        let last = self.items.len().saturating_sub(1);
        for (i, item) in self.items.iter().enumerate() {
            let current = i == last;
            let fg = if current { k.text_fill_color_primary } else { k.text_fill_color_secondary };
            let mut e = row()
                .h(sizes::CONTROL_HEIGHT)
                .px(4.0)
                .center_y()
                .radius(R)
                .child(rune::text(item.clone()).font(style::body(t)).color(rc(fg)).group_color(rc(k.text_fill_color_primary)))
                .key(format!("breadcrumb:{i}"))
                .label(item.clone());
            if !current {
                e = self.select.attach(focus_rect(e.hover_group().press_color(rc(k.text_fill_color_tertiary)).role("link"), t, R), i);
            }
            bar = bar.child(e);
            if !current {
                bar = bar.child(glyph(Symbol::ChevronRight, 12.0, k.text_fill_color_secondary));
            }
        }
        bar
    }
}

impl_control!(NavigationView, TabView, BreadcrumbBar);
