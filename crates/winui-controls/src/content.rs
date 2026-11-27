//! Content controls: Card, Expander, ListRow (reveal rows), PersonPicture,
//! IconElement, InfoBadge. Matches "Collections" / "Layout" Gallery samples.

use rune::{Color, El, col, el, row, text};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, radii, sizes};

use crate::{body, caption, glyph};

/// The standard WinUI card: `CardBackgroundFillColorDefault` +
/// `CardStrokeColorDefault` + control radius, with a hover lift.
pub struct Card {
    pub child: Option<El>,
    pub hover: bool,
}

impl Card {
    pub fn new() -> Self {
        Self { child: None, hover: false }
    }
    pub fn child(mut self, e: El) -> Self {
        self.child = Some(e);
        self
    }
    /// The clickable-card look (hover subtle-fill).
    pub fn interactive(mut self, on: bool) -> Self {
        self.hover = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let mut e = el().radius(radii::CONTROL)
            .fill(rc(t.card_background_fill_color_default))
            .border(1.0, rc(t.card_stroke_color_default));
        if self.hover {
            e = e.hover_fill(rc(t.card_background_fill_color_secondary)).cursor(rune::Cursor::Pointer);
        }
        if let Some(c) = self.child.clone() {
            e = e.child(c);
        }
        e
    }
}

/// An Expander: card header row (title + chevron) — expanded shows the
/// content area below a divider, collapsed is the header alone.
#[derive(Clone, Debug)]
pub struct Expander {
    pub title: String,
    pub expanded: bool,
    pub content: Option<String>,
}

impl Expander {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), expanded: false, content: None }
    }
    pub fn expanded(mut self, on: bool) -> Self {
        self.expanded = on;
        self
    }
    pub fn content(mut self, c: impl Into<String>) -> Self {
        self.content = Some(c.into());
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let header = row().items_center()
            .h(sizes::EXPANDER_MIN_HEIGHT)
            .px(16.0)
            .child(
                text(self.title.clone())
                    .font(body())
                    .color(rc(t.text_fill_color_primary))
                    .grow(1.0),
            )
            .child(
                el().size(sizes::EXPANDER_CHEVRON_BUTTON_SIZE, sizes::EXPANDER_CHEVRON_BUTTON_SIZE)
                    .center()
                    .radius(4.0)
                    .role("button")
                    .label(if self.expanded { "Collapse" } else { "Expand" })
                    .hover_fill(rc(t.subtle_fill_color_secondary))
                    .child(glyph(
                        if self.expanded { Symbol::ChevronUp } else { Symbol::ChevronDown },
                        12.0,
                        rc(t.text_fill_color_secondary),
                    )),
            );
        let mut card = col().w_full().child(header);
        if self.expanded {
            card = card
                .child(el().w_full().h(1.0).fill(rc(t.divider_stroke_color_default)))
                .child(
                    el().w_full()
                        .padding(sizes::EXPANDER_CONTENT_PADDING)
                        .child(
                            text(self.content.clone().unwrap_or_default())
                                .font(body())
                                .color(rc(t.text_fill_color_secondary)),
                        ),
                );
        }
        el().w_full()
            .radius(radii::CONTROL)
            .fill(rc(t.card_background_fill_color_default))
            .border(1.0, rc(t.card_stroke_color_default))
            .clip_rounded(radii::CONTROL)
            .child(card)
    }
}

/// A ListView row: icon/avatar slot + two text lines, with reveal hover and
/// the selected pill — the look of Gallery ListView items.
#[derive(Clone, Debug)]
pub struct ListRow {
    pub title: String,
    pub subtitle: Option<String>,
    pub symbol: Option<Symbol>,
    pub selected: bool,
}

impl ListRow {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), subtitle: None, symbol: None, selected: false }
    }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
        self
    }
    pub fn symbol(mut self, s: Symbol) -> Self {
        self.symbol = Some(s);
        self
    }
    pub fn selected(mut self, on: bool) -> Self {
        self.selected = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let mut content = row().items_center().h(sizes::LIST_VIEW_ITEM_MIN_HEIGHT).px(12.0).gap(12.0);
        if let Some(sym) = self.symbol {
            content = content.child(
                el().size(32.0, 32.0)
                    .radius(4.0)
                    .center()
                    .fill(rc(t.subtle_fill_color_secondary))
                    .child(glyph(sym, 16.0, rc(t.text_fill_color_primary))),
            );
        }
        let mut texts = col().gap(1.0).child(
            text(self.title.clone()).font(body()).color(rc(t.text_fill_color_primary)).nowrap(),
        );
        if let Some(sub) = &self.subtitle {
            texts = texts.child(
                text(sub.clone()).font(caption()).color(rc(t.text_fill_color_secondary)).nowrap(),
            );
        }
        content = content.child(texts);
        let mut e = el().w_full()
            .mx(4.0)
            .radius(4.0)
            .role("listitem")
            .label(self.title.clone())
            .child(content);
        if self.selected {
            e = e.fill(rc(t.subtle_fill_color_secondary)).child(
                el().absolute(0.0, 12.0)
                    .size(3.0, 16.0)
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

/// A PersonPicture: initials on a themed disc (or the person glyph when the
/// name is empty).
#[derive(Clone, Debug)]
pub struct PersonPicture {
    pub initials: String,
    pub size: f32,
}

impl PersonPicture {
    pub fn new(initials: impl Into<String>) -> Self {
        Self { initials: initials.into(), size: 40.0 }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let centre = if self.initials.is_empty() {
            glyph(Symbol::Person, self.size * 0.5, rc(t.text_fill_color_secondary))
        } else {
            text(self.initials.clone())
                .font(rune::TextStyle::new(self.size * 0.36, 600.0))
                .color(rc(t.text_fill_color_primary))
        };
        el().size(self.size, self.size)
            .radius(999.0)
            .fill(rc(t.subtle_fill_color_tertiary))
            .center()
            .child(centre)
    }
}

/// IconElement: a Fluent glyph at standard control sizes.
#[derive(Clone, Debug)]
pub struct IconElement {
    pub symbol: Symbol,
    pub size: f32,
    pub color: Option<Color>,
}

impl IconElement {
    pub fn new(symbol: Symbol) -> Self {
        Self { symbol, size: 16.0, color: None }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        glyph(
            self.symbol,
            self.size,
            self.color.unwrap_or_else(|| rc(theme.tokens().text_fill_color_primary)),
        )
    }
}

/// An InfoBadge: the small accent pill/dot pinned on nav items and icons.
/// `None` value = dot only.
#[derive(Clone, Debug)]
pub struct InfoBadge {
    pub value: Option<u32>,
}

impl InfoBadge {
    pub fn dot() -> Self {
        Self { value: None }
    }
    pub fn count(n: u32) -> Self {
        Self { value: Some(n) }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        match self.value {
            None => el().size(8.0, 8.0).radius(999.0).fill(rc(t.system_fill_color_critical)),
            Some(n) => el().min_w(16.0)
                .h(16.0)
                .px(4.0)
                .radius(999.0)
                .center()
                .fill(rc(t.system_fill_color_critical))
                .child(
                    text(n.to_string())
                        .font(rune::TextStyle::new(11.0, 400.0))
                        .color(rc(t.text_on_accent_fill_color_primary)),
                ),
        }
    }
}
