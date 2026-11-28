//! Content: `Expander`, `ListView`-lite, `PersonPicture`, `IconElement`, `InfoBadge`
//! (WinUI Gallery › Layout › Expander, Collections › ListView, Media/People › PersonPicture,
//! Iconography, Status & info › InfoBadge).
use std::rc::Rc;

use rune::{col, row, El};

use crate::impl_control;
use crate::menus::selection_pill;
use crate::shim::icons::Symbol;
use crate::shim::theme::{self, Theme};
use crate::style::{self, focus_rect, glyph, rc, Hook, R};

/// `Expander` — Gallery "Expander": a 48px card header with a 32px chevron button; the
/// content card below uses `CardBackgroundFillColorSecondary` and shares the border.
#[derive(Clone)]
pub struct Expander {
    header: String,
    description: Option<String>,
    icon: Option<Symbol>,
    expanded: bool,
    width: Option<f32>,
    key: String,
    content: Rc<dyn Fn(&Theme) -> El>,
    toggle: Hook<bool>,
}

impl Expander {
    pub fn new(header: impl Into<String>, content: impl Fn(&Theme) -> El + 'static) -> Self {
        let header = header.into();
        Expander {
            key: format!("expander:{header}"),
            header,
            description: None,
            icon: None,
            expanded: false,
            width: None,
            content: Rc::new(content),
            toggle: Hook::default(),
        }
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn expanded(mut self, on: bool) -> Self {
        self.expanded = on;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let mut text = col().grow(1.0).child(rune::text(self.header.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)));
        if let Some(d) = &self.description {
            text = text.child(rune::text(d.clone()).font(style::caption(t)).color(rc(k.text_fill_color_secondary)));
        }
        let chevron = rune::el()
            .size(32.0, 32.0)
            .center()
            .radius(R)
            .group_opacity(1.0, 1.0)
            .child(glyph(if self.expanded { Symbol::ChevronUp } else { Symbol::ChevronDown }, 12.0, k.text_fill_color_primary));
        let chevron_bg = crate::style::layers((32.0, 32.0), R, (k.subtle_fill_color_transparent, None), (k.subtle_fill_color_secondary, None))
            .center()
            .child(chevron.absolute(0.0, 0.0));
        let mut head = row()
            .min_h(48.0)
            .pl(16.0)
            .pr(8.0)
            .gap(16.0)
            .center_y()
            .hover_group();
        head = style::surface(head, k.card_background_fill_color_default, k.card_stroke_color_default, if self.expanded { [R, R, 0.0, 0.0] } else { [R; 4] });
        if let Some(i) = self.icon {
            head = head.child(glyph(i, 16.0, k.text_fill_color_primary));
        }
        head = head.child(text).child(chevron_bg).role("button").label(self.header.clone()).expanded(self.expanded).key(self.key.clone());
        head = self.toggle.attach(focus_rect(head.press_color(rc(k.text_fill_color_secondary)), t, R), !self.expanded);
        let mut e = col().child(head);
        if self.expanded {
            e = e.child(
                col()
                    .padding(16.0)
                    .fill(rc(k.card_background_fill_color_secondary))
                    .border_sides([1.0, 0.0, 1.0, 1.0], rc(k.card_stroke_color_default))
                    .corners([0.0, 0.0, R, R])
                    .child((self.content)(t)),
            );
        }
        match self.width {
            Some(w) => e.w(w),
            None => e,
        }
    }
}

/// One `ListView` row.
#[derive(Clone, Debug)]
pub struct ListItem {
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<Symbol>,
}

impl ListItem {
    pub fn new(title: impl Into<String>) -> Self {
        ListItem { title: title.into(), subtitle: None, icon: None }
    }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
}

/// `ListView`-lite — Gallery "ListView" (single selection): 40px rows (56px with a
/// subtitle) inset 4px, hover/press subtle fills, selected row filled with the accent pill.
#[derive(Clone, Default)]
pub struct ListView {
    items: Vec<ListItem>,
    selected: Option<usize>,
    width: Option<f32>,
    key: Option<String>,
    select: Hook<usize>,
}

impl ListView {
    pub fn new(items: impl IntoIterator<Item = ListItem>) -> Self {
        ListView { items: items.into_iter().collect(), ..Default::default() }
    }
    pub fn selected(mut self, i: Option<usize>) -> Self {
        self.selected = i;
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
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let key = self.key.clone().unwrap_or_else(|| "listview".into());
        let mut list = col().role("listbox").key(key.clone());
        for (i, item) in self.items.iter().enumerate() {
            let sel = self.selected == Some(i);
            let h = if item.subtitle.is_some() { 56.0 } else { 40.0 };
            let mut r = crate::menus::list_row(t, true, sel).min_h(h);
            if let Some(ic) = item.icon {
                r = r.child(glyph(ic, 16.0, k.text_fill_color_primary));
            }
            let mut text = col().child(rune::text(item.title.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)));
            if let Some(s) = &item.subtitle {
                text = text.child(rune::text(s.clone()).font(style::caption(t)).color(rc(k.text_fill_color_secondary)));
            }
            r = r.child(text).role("option").checked(sel).label(item.title.clone()).key(format!("{key}:{i}"));
            if sel {
                r = r.child(selection_pill(t, 0.0, (h - 16.0) / 2.0));
            }
            list = list.child(self.select.attach(r, i));
        }
        match self.width {
            Some(w) => list.w(w),
            None => list,
        }
    }
}

/// `InfoBadge` — Gallery "InfoBadge": a dot, a number pill or an icon circle.
#[derive(Clone, Copy, Debug)]
pub enum BadgeKind {
    Dot,
    Value(u32),
    Icon(Symbol),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeSeverity {
    /// `AccentFillColorDefault` (the default style).
    #[default]
    Attention,
    Informational,
    Success,
    Caution,
    Critical,
}

#[derive(Clone, Debug)]
pub struct InfoBadge {
    kind: BadgeKind,
    severity: BadgeSeverity,
}

impl InfoBadge {
    pub fn dot() -> Self {
        InfoBadge { kind: BadgeKind::Dot, severity: BadgeSeverity::Attention }
    }
    pub fn value(v: u32) -> Self {
        InfoBadge { kind: BadgeKind::Value(v), severity: BadgeSeverity::Attention }
    }
    pub fn icon(s: Symbol) -> Self {
        InfoBadge { kind: BadgeKind::Icon(s), severity: BadgeSeverity::Attention }
    }
    pub fn severity(mut self, s: BadgeSeverity) -> Self {
        self.severity = s;
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let bg = match self.severity {
            BadgeSeverity::Attention => k.accent_fill_color_default,
            BadgeSeverity::Informational => k.system_fill_color_neutral,
            BadgeSeverity::Success => k.system_fill_color_success,
            BadgeSeverity::Caution => k.system_fill_color_caution,
            BadgeSeverity::Critical => k.system_fill_color_critical,
        };
        let fg = k.text_on_accent_fill_color_primary;
        let e = match self.kind {
            BadgeKind::Dot => rune::el().size(6.0, 6.0).radius(3.0).fill(rc(bg)),
            BadgeKind::Value(v) => row()
                .h(16.0)
                .min_w(16.0)
                .px(4.0)
                .center()
                .radius(8.0)
                .fill(rc(bg))
                .child(rune::text(v.to_string()).font(rune::TextStyle::new(11.0, 400.0).lh(16.0)).color(rc(fg))),
            BadgeKind::Icon(s) => rune::el().size(16.0, 16.0).center().radius(8.0).fill(rc(bg)).child(glyph(s, 10.0, fg)),
        };
        e.role("status").label(match self.kind {
            BadgeKind::Value(v) => format!("{v} notifications"),
            _ => "Notification".into(),
        })
    }
}

/// `PersonPicture` — Gallery "PersonPicture": a circle with initials (or the contact glyph),
/// 96px by default, optional badge number top-right.
#[derive(Clone, Debug)]
pub struct PersonPicture {
    name: Option<String>,
    initials: Option<String>,
    size: f32,
    badge: Option<u32>,
}

impl Default for PersonPicture {
    fn default() -> Self {
        PersonPicture { name: None, initials: None, size: 96.0, badge: None }
    }
}

impl PersonPicture {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn display_name(mut self, n: impl Into<String>) -> Self {
        self.name = Some(n.into());
        self
    }
    pub fn initials(mut self, i: impl Into<String>) -> Self {
        self.initials = Some(i.into());
        self
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn badge(mut self, n: u32) -> Self {
        self.badge = Some(n);
        self
    }
    /// WinUI's initials rule: first letters of the first and last words.
    pub fn initials_of(name: &str) -> String {
        let words: Vec<&str> = name.split_whitespace().filter(|w| w.chars().next().is_some_and(char::is_alphanumeric)).collect();
        let first = |w: &&str| w.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
        match words.as_slice() {
            [] => String::new(),
            [one] => first(one),
            [a, .., b] => first(a) + &first(b),
        }
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let s = self.size;
        let initials = self.initials.clone().or_else(|| self.name.as_deref().map(Self::initials_of)).filter(|i| !i.is_empty());
        let mut circle = rune::el()
            .size(s, s)
            .center()
            .radius(s / 2.0)
            .fill(rc(k.control_alt_fill_color_quarternary))
            .border(1.0, rc(k.card_stroke_color_default))
            .role("img")
            .label(self.name.clone().unwrap_or_else(|| "Person".into()));
        circle = match initials {
            Some(i) => circle.child(rune::text(i).font(rune::TextStyle::new((s * 0.42).round(), 600.0).lh(s * 0.5)).color(rc(k.text_fill_color_primary))),
            None => circle.child(glyph(Symbol::Person, s * 0.42, k.text_fill_color_primary)),
        };
        match self.badge {
            Some(n) => {
                let b = (s * 0.25).max(16.0);
                let badge = row()
                    .absolute(s - b * 0.9, 0.0)
                    .h(b)
                    .min_w(b)
                    .px(4.0)
                    .center()
                    .radius(b / 2.0)
                    .fill(rc(k.control_solid_fill_color_default))
                    .border(1.0, rc(k.card_stroke_color_default))
                    .child(rune::text(n.to_string()).font(rune::TextStyle::new((b * 0.55).round(), 600.0).lh(b)).color(rc(k.text_fill_color_primary)));
                rune::el().size(s, s).child(circle).child(badge)
            }
            None => circle,
        }
    }
}

/// `IconElement` — Gallery "Iconography" (`SymbolIcon`/`FontIcon`/`PathIcon`): a Fluent glyph
/// at 16px in `TextFillColorPrimary` unless told otherwise.
#[derive(Clone, Debug)]
pub struct IconElement {
    symbol: Symbol,
    size: f32,
    color: Option<theme::Color>,
}

impl IconElement {
    pub fn new(symbol: Symbol) -> Self {
        IconElement { symbol, size: 16.0, color: None }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn foreground(mut self, c: theme::Color) -> Self {
        self.color = Some(c);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        glyph(self.symbol, self.size, self.color.unwrap_or(t.tokens().text_fill_color_primary)).role("img").label(format!("{:?}", self.symbol))
    }
}

impl_control!(Expander, ListView, InfoBadge, PersonPicture, IconElement);
