//! Menus and flyouts: ComboBox, Flyout, MenuFlyout, CommandBar-lite.
//! Matches "Menus & toolbars" Gallery samples.

use rune::{El, col, el, row, text};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, radii, sizes};

use crate::{body, caption, glyph};

/// A closed ComboBox: text-control chrome, the current value, chevron right.
/// For the open look, wrap it with [`ComboBox::popup`].
#[derive(Clone, Debug)]
pub struct ComboBox {
    pub value: String,
    pub enabled: bool,
    /// Rendered selected index inside [`popup`](Self::popup)'s list.
    pub selected: usize,
}

impl ComboBox {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), enabled: true, selected: 0 }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        el().role("combobox")
            .label(self.value.clone())
            .min_h(sizes::CONTROL_HEIGHT)
            .min_w(sizes::COMBO_BOX_MIN_WIDTH)
            .radius(4.0)
            .fill(rc(if self.enabled {
                t.control_fill_color_input_active
            } else {
                t.control_fill_color_disabled
            }))
            .border(1.0, rc(t.control_stroke_color_secondary))
            .clip_rounded(4.0)
            .child(
                row().items_center().w_full()
                    .pl(sizes::COMBO_BOX_PADDING.left)
                    .child(
                        text(self.value.clone())
                            .font(body())
                            .color(rc(if self.enabled {
                                t.text_fill_color_primary
                            } else {
                                t.text_fill_color_disabled
                            }))
                            .grow(1.0)
                            .nowrap(),
                    )
                    .child(
                        el().w(30.0).h_full().center().child(
                            glyph(Symbol::ChevronDown, 12.0, rc(t.text_fill_color_secondary)),
                        ),
                    ),
            )
            .child(
                el().absolute(0.0, sizes::CONTROL_HEIGHT - 1.0)
                    .w_full()
                    .h(1.0)
                    .fill(rc(t.control_strong_stroke_color_default)),
            )
    }

    /// The open dropdown list: `items` with the selected row pill-highlighted.
    /// Position it under the closed box in your layout.
    pub fn popup(&self, theme: &Theme, items: &[String], width: f32) -> El {
        let t = theme.tokens();
        let mut list = col().py(4.0);
        for (i, item) in items.iter().enumerate() {
            let mut e = el().w_full()
                .h(28.0)
                .center_y()
                .px(sizes::COMBO_BOX_ITEM_PADDING.left)
                .radius(4.0)
                .mx(4.0)
                .child(text(item.clone()).font(body()).color(rc(t.text_fill_color_primary)).nowrap());
            if i == self.selected {
                e = e.fill(rc(t.subtle_fill_color_secondary))
                    .child(
                        el().absolute(0.0, 6.0)
                            .size(sizes::COMBO_BOX_ITEM_PILL_WIDTH, sizes::COMBO_BOX_ITEM_PILL_HEIGHT)
                            .radius(2.0)
                            .fill(rc(t.accent_fill_color_default)),
                    );
            } else {
                e = e.hover_fill(rc(t.subtle_fill_color_secondary));
            }
            list = list.child(e);
        }
        el().w(width)
            .radius(radii::OVERLAY)
            .fill(rc(t.layer_on_acrylic_fill_color_default))
            .border(1.0, rc(t.surface_stroke_color_flyout))
            .box_shadow(winui_materials::elevation(theme, 16.0))
            .child(list)
    }
}

/// One menu row: optional icon, label, optional shortcut hint, optional
/// submenu chevron.
#[derive(Clone, Debug)]
pub struct MenuItem {
    pub label: String,
    pub symbol: Option<Symbol>,
    pub shortcut: Option<String>,
    pub submenu: bool,
    pub enabled: bool,
    pub selected: bool,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            symbol: None,
            shortcut: None,
            submenu: false,
            enabled: true,
            selected: false,
        }
    }
    pub fn symbol(mut self, s: Symbol) -> Self {
        self.symbol = Some(s);
        self
    }
    pub fn shortcut(mut self, s: impl Into<String>) -> Self {
        self.shortcut = Some(s.into());
        self
    }
    pub fn submenu(mut self, on: bool) -> Self {
        self.submenu = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let text_c = rc(if self.enabled {
            t.text_fill_color_primary
        } else {
            t.text_fill_color_disabled
        });
        let mut e = row().items_center()
            .h(28.0)
            .px(12.0)
            .gap(8.0)
            .children(self.symbol.map(|s| glyph(s, 16.0, text_c)))
            .child(text(self.label.clone()).font(body()).color(text_c).grow(1.0).nowrap());
        if let Some(sc) = &self.shortcut {
            e = e.child(text(sc.clone()).font(caption()).color(rc(t.text_fill_color_tertiary)).nowrap());
        }
        if self.submenu {
            e = e.child(glyph(Symbol::ChevronRight, 12.0, rc(t.text_fill_color_tertiary)));
        }
        let mut e = el().w_full().mx(4.0).radius(4.0).child(e);
        if self.selected {
            e = e.fill(rc(t.subtle_fill_color_secondary));
        } else if self.enabled {
            e = e.hover_fill(rc(t.subtle_fill_color_secondary));
        }
        e.role("menuitem").label(self.label.clone())
    }
}

/// A MenuFlyout: the elevated acrylic menu, `MenuItem` rows plus separators
/// (`MenuItem::new("---")` renders a divider).
#[derive(Clone, Debug, Default)]
pub struct MenuFlyout {
    pub items: Vec<MenuItem>,
}

impl MenuFlyout {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self { items }
    }
    pub fn view(&self, theme: &Theme, width: f32) -> El {
        let t = theme.tokens();
        let mut list = col().py(4.0);
        for item in &self.items {
            if item.label == "---" {
                list = list.child(
                    el().w_full().h(1.0).my(4.0).mx(12.0).fill(rc(t.divider_stroke_color_default)),
                );
            } else {
                list = list.child(item.view(theme));
            }
        }
        el().w(width)
            .radius(radii::OVERLAY)
            .fill(rc(t.layer_on_acrylic_fill_color_default))
            .border(1.0, rc(t.surface_stroke_color_flyout))
            .box_shadow(winui_materials::elevation(theme, 16.0))
            .child(list)
    }
}

/// A plain Flyout surface — the elevated acrylic card any content can sit in.
#[derive(Clone, Debug)]
pub struct Flyout {
    pub title: Option<String>,
    pub body: String,
}

impl Flyout {
    pub fn new(body: impl Into<String>) -> Self {
        Self { title: None, body: body.into() }
    }
    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }
    pub fn view(&self, theme: &Theme, width: f32) -> El {
        let t = theme.tokens();
        let pad = sizes::FLYOUT_CONTENT_PADDING;
        let mut c = col().gap(8.0).pl(pad.left).pr(pad.right).pt(pad.top).pb(pad.bottom);
        if let Some(title) = &self.title {
            c = c.child(
                text(title.clone())
                    .font(rune::TextStyle::new(14.0, 600.0))
                    .color(rc(t.text_fill_color_primary)),
            );
        }
        c = c.child(text(self.body.clone()).font(body()).color(rc(t.text_fill_color_secondary)));
        el().w(width)
            .radius(radii::OVERLAY)
            .fill(rc(t.layer_on_acrylic_fill_color_default))
            .border(1.0, rc(t.surface_stroke_color_flyout))
            .box_shadow(winui_materials::elevation(theme, 32.0))
            .child(c)
    }
}

/// CommandBar-lite: a row of icon buttons on a card surface, ending in a
/// "more" (…) button.
#[derive(Clone, Debug)]
pub struct CommandBar {
    pub commands: Vec<(Symbol, String)>,
}

impl CommandBar {
    pub fn new(commands: Vec<(Symbol, String)>) -> Self {
        Self { commands }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let mut bar = row().items_center().gap(2.0).px(8.0).h(40.0);
        for (sym, name) in &self.commands {
            bar = bar.child(
                el().size(32.0, 32.0)
                    .center()
                    .radius(4.0)
                    .role("button")
                    .label(name.clone())
                    .hover_fill(rc(t.subtle_fill_color_secondary))
                    .press_fill(rc(t.subtle_fill_color_tertiary))
                    .cursor(rune::Cursor::Pointer)
                    .child(glyph(*sym, 16.0, rc(t.text_fill_color_primary))),
            );
        }
        bar = bar.child(
            el().size(32.0, 32.0)
                .center()
                .radius(4.0)
                .role("button")
                .label("More commands")
                .hover_fill(rc(t.subtle_fill_color_secondary))
                .child(glyph(Symbol::MoreHorizontal, 16.0, rc(t.text_fill_color_primary))),
        );
        el().radius(radii::OVERLAY)
            .fill(rc(t.layer_fill_color_default))
            .border(1.0, rc(t.card_stroke_color_default))
            .box_shadow(winui_materials::elevation(theme, 8.0))
            .child(bar)
    }
}
