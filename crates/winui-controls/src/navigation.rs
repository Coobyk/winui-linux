//! Navigation: NavigationView (left expanded + compact), TabView,
//! BreadcrumbBar-lite. Matches "Navigation" Gallery samples.

use rune::{El, col, el, row, text};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, sizes};

use crate::{body, caption, glyph};

/// One NavigationView item: Fluent glyph + label.
#[derive(Clone, Debug)]
pub struct NavItem {
    pub label: String,
    pub symbol: Symbol,
    pub selected: bool,
    /// Small header-style item (uppercase caption, not clickable).
    pub header: bool,
}

impl NavItem {
    pub fn new(label: impl Into<String>, symbol: Symbol) -> Self {
        Self { label: label.into(), symbol, selected: false, header: false }
    }
    pub fn selected(mut self, on: bool) -> Self {
        self.selected = on;
        self
    }
    /// A group header ("Apps", "Styles") — caption text, no icon row.
    pub fn header(label: impl Into<String>) -> Self {
        Self { label: label.into(), symbol: Symbol::Circle, selected: false, header: true }
    }

    pub fn view(&self, theme: &Theme, expanded: bool) -> El {
        let t = theme.tokens();
        if self.header {
            return el().w_full().h(28.0).center_y().pl(12.0).child(
                text(self.label.clone())
                    .font(caption())
                    .color(rc(t.text_fill_color_tertiary))
                    .nowrap(),
            );
        }
        let mut e = el().w_full()
            .h(sizes::NAV_ITEM_HEIGHT)
            .mx(4.0)
            .radius(4.0)
            .role("link")
            .label(self.label.clone())
            .child(
                row().items_center()
                    .child(
                        el().w(sizes::NAV_ICON_BOX_WIDTH)
                            .h_full()
                            .center()
                            .child(glyph(self.symbol, sizes::NAV_ICON_SIZE, rc(t.text_fill_color_primary))),
                    )
                    .children(if expanded {
                        Some(text(self.label.clone()).font(body()).color(rc(t.text_fill_color_primary)).nowrap())
                    } else {
                        None
                    }),
            );
        if self.selected {
            e = e.fill(rc(t.subtle_fill_color_secondary)).child(
                el().absolute(0.0, (sizes::NAV_ITEM_HEIGHT - sizes::NAV_SELECTION_INDICATOR_HEIGHT) / 2.0)
                    .size(sizes::NAV_SELECTION_INDICATOR_WIDTH, sizes::NAV_SELECTION_INDICATOR_HEIGHT)
                    .radius(2.0)
                    .fill(rc(t.accent_fill_color_default)),
            );
        } else {
            e = e.hover_fill(winui_materials::reveal_fill(theme))
                .press_fill(rc(t.subtle_fill_color_tertiary))
                .cursor(rune::Cursor::Pointer);
        }
        e
    }
}

/// The NavigationView shell: acrylic pane with hamburger, header, items and a
/// pinned Settings footer. `expanded` = open 320px pane; `false` = 48px rail.
#[derive(Clone, Debug, Default)]
pub struct NavigationView {
    pub header: String,
    pub items: Vec<NavItem>,
    /// Footer items pinned to the bottom (Settings by convention).
    pub footer: Vec<NavItem>,
    pub expanded: bool,
    /// Show the AutoSuggestBox row under the hamburger.
    pub search: bool,
}

impl NavigationView {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            items: Vec::new(),
            footer: Vec::new(),
            expanded: true,
            search: true,
        }
    }
    pub fn items(mut self, items: Vec<NavItem>) -> Self {
        self.items = items;
        self
    }
    pub fn footer(mut self, items: Vec<NavItem>) -> Self {
        self.footer = items;
        self
    }
    pub fn expanded(mut self, on: bool) -> Self {
        self.expanded = on;
        self
    }

    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let pane_w = if self.expanded {
            sizes::NAV_OPEN_PANE_LENGTH
        } else {
            sizes::NAV_COMPACT_PANE_LENGTH
        };
        let mut pane = col().w(pane_w).h_full().py(4.0);

        // Hamburger + header.
        pane = pane.child(
            row().items_center().h(sizes::NAV_PANE_HEADER_MIN_HEIGHT).child(
                el().size(sizes::PANE_TOGGLE_BUTTON_WIDTH, sizes::PANE_TOGGLE_BUTTON_HEIGHT)
                    .center()
                    .radius(4.0)
                    .mx(4.0)
                    .role("button")
                    .label("Toggle navigation")
                    .hover_fill(winui_materials::reveal_fill(theme))
                    .cursor(rune::Cursor::Pointer)
                    .child(glyph(Symbol::GlobalNavigation, 16.0, rc(t.text_fill_color_primary))),
            ),
        );

        if self.search {
            let box_w = pane_w - 16.0;
            pane = pane.child(
                el().w(box_w)
                    .h(32.0)
                    .mx(8.0)
                    .mb(4.0)
                    .radius(4.0)
                    .fill(rc(t.control_fill_color_input_active))
                    .border(1.0, rc(t.control_stroke_color_secondary))
                    .clip_rounded(4.0)
                    .child(
                        row().items_center().w_full().pl(10.0)
                            .children(if self.expanded {
                                Some(
                                    text("Search")
                                        .font(body())
                                        .color(rc(t.text_fill_color_tertiary))
                                        .grow(1.0),
                                )
                            } else {
                                None
                            })
                            .child(
                                el().w(30.0).h_full().center().child(
                                    glyph(Symbol::Find, 12.0, rc(t.text_fill_color_secondary)),
                                ),
                            ),
                    )
                    .child(
                        el().absolute(0.0, 31.0)
                            .w_full()
                            .h(1.0)
                            .fill(rc(t.control_strong_stroke_color_default)),
                    ),
            );
        }

        let mut items = col().w_full().grow(1.0);
        for item in &self.items {
            items = items.child(item.view(theme, self.expanded));
        }
        pane = pane.child(items);

        for item in &self.footer {
            pane = pane.child(item.view(theme, self.expanded));
        }

        el().key("navview")
            .h_full()
            .child(
                el().h_full()
                    .w(pane_w + 8.0)
                    .child(winui_materials::nav_acrylic(theme))
                    .child(
                        el().w_full().h_full().absolute(0.0, 0.0).child(pane),
                    ),
            )
    }
}

/// One TabView tab.
#[derive(Clone, Debug)]
pub struct Tab {
    pub title: String,
    pub symbol: Option<Symbol>,
    pub selected: bool,
    pub closable: bool,
}

impl Tab {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), symbol: None, selected: false, closable: true }
    }
    pub fn symbol(mut self, s: Symbol) -> Self {
        self.symbol = Some(s);
        self
    }
    pub fn selected(mut self, on: bool) -> Self {
        self.selected = on;
        self
    }

    fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let pad = sizes::TAB_ITEM_PADDING;
        let mut content = row().items_center().h_full().pl(10.0).pr(4.0).gap(6.0);
        if let Some(sym) = self.symbol {
            content = content.child(glyph(sym, 12.0, rc(t.text_fill_color_primary)));
        }
        content = content
            .child(
                text(self.title.clone())
                    .font(rune::TextStyle::new(sizes::TAB_HEADER_FONT_SIZE, 400.0))
                    .color(rc(if self.selected {
                        t.text_fill_color_primary
                    } else {
                        t.text_fill_color_secondary
                    }))
                    .nowrap(),
            )
            .child(
                el().size(20.0, 20.0)
                    .center()
                    .radius(4.0)
                    .role("button")
                    .label(format!("Close {}", self.title))
                    .hover_fill(rc(t.subtle_fill_color_tertiary))
                    .child(glyph(Symbol::Dismiss, 8.0, rc(t.text_fill_color_secondary))),
            );
        let _ = pad;
        let mut e = el().min_w(sizes::TAB_ITEM_MIN_WIDTH)
            .max_w(sizes::TAB_ITEM_MAX_WIDTH)
            .h(sizes::TAB_ITEM_MIN_HEIGHT)
            .child(content);
        if self.selected {
            e = e.fill(rc(t.layer_fill_color_default))
                .corners([4.0, 4.0, 0.0, 0.0])
                .child(
                    el().absolute(0.0, sizes::TAB_ITEM_MIN_HEIGHT - 1.0)
                        .w_full()
                        .h(1.0)
                        .fill(rc(t.layer_fill_color_default)),
                );
        } else {
            e = e.corners([4.0, 4.0, 0.0, 0.0])
                .hover_fill(rc(t.subtle_fill_color_secondary))
                .cursor(rune::Cursor::Pointer);
        }
        e
    }
}

/// A TabView strip on the mica-alt band: tabs + a trailing "add" button.
#[derive(Clone, Debug, Default)]
pub struct TabView {
    pub tabs: Vec<Tab>,
}

impl TabView {
    pub fn new(tabs: Vec<Tab>) -> Self {
        Self { tabs }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let mut strip = row().items_end().h(40.0).px(4.0);
        for (i, tab) in self.tabs.iter().enumerate() {
            strip = strip.child(tab.view(theme).key(format!("tab-{i}")));
        }
        strip = strip.child(
            el().size(28.0, 28.0)
                .mb(4.0)
                .center()
                .radius(4.0)
                .role("button")
                .label("New tab")
                .hover_fill(rc(t.subtle_fill_color_secondary))
                .cursor(rune::Cursor::Pointer)
                .child(glyph(Symbol::Add, 12.0, rc(t.text_fill_color_primary))),
        );
        el().w_full()
            .fill(rc(t.solid_background_fill_color_base_alt))
            .child(strip)
            .child(
                el().absolute(0.0, 39.0).w_full().h(1.0).fill(rc(t.divider_stroke_color_default)),
            )
    }
}

/// BreadcrumbBar-lite: `Home / Documents / Report` — last item is the current
/// page (primary text), the rest are secondary links with chevron separators.
#[derive(Clone, Debug)]
pub struct BreadcrumbBar {
    pub trail: Vec<String>,
}

impl BreadcrumbBar {
    pub fn new(trail: Vec<String>) -> Self {
        Self { trail }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let mut e = row().items_center().h(32.0);
        let n = self.trail.len();
        for (i, crumb) in self.trail.iter().enumerate() {
            let last = i == n - 1;
            let mut link = el().radius(4.0)
                .px(4.0)
                .h(24.0)
                .center_y()
                .role("link")
                .label(crumb.clone())
                .child(
                    text(crumb.clone())
                        .font(body())
                        .color(rc(if last { t.text_fill_color_primary } else { t.text_fill_color_secondary }))
                        .nowrap(),
                );
            if !last {
                link = link.hover_fill(rc(t.subtle_fill_color_secondary)).cursor(rune::Cursor::Pointer);
            }
            e = e.child(link);
            if !last {
                e = e.child(
                    el().w(20.0).h_full().center().child(
                        glyph(Symbol::ChevronRight, 8.0, rc(t.text_fill_color_tertiary)),
                    ),
                );
            }
        }
        e
    }
}
