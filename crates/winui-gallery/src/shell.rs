//! The NavigationView shell: Mica window, title bar, acrylic nav pane with search, content frame.

use crate::fallback::controls::{self, body, caption, icon, list_row};
use crate::fallback::icons::Symbol;
use crate::fallback::materials;
use crate::fallback::theme::{Theme, TYPE_RAMP, radii, sizes};
use crate::pages;
use crate::state::{NAV_GROUPS, Page, State};
use rune::{Cx, El, ScrollSpec, col, el, row, text};

fn c(x: crate::fallback::theme::Color) -> rune::Color {
    x.into()
}

pub fn view(s: &State, cx: &mut Cx<State>) -> El {
    let theme = s.theme(cx.env().system_dark);
    let t = theme.tokens();
    let body = row().w_full().grow(1.0).child(nav_pane(s, &theme)).child(content_frame(s, &theme));
    materials::mica(&theme)
        .key("window")
        .child(col().absolute(0.0, 0.0).w_full().h_full().child(title_bar(s, &theme)).child(body))
        .color(c(t.text_fill_color_primary))
}

fn title_bar(s: &State, theme: &Theme) -> El {
    let t = theme.tokens();
    let subtle = |key: &str, sym: Symbol, label: &str, enabled: bool| {
        let fg = if enabled { t.text_fill_color_primary } else { t.text_fill_color_disabled };
        let mut b = el().key(key).role("button").label(label).size(40.0, 36.0).radius(radii::CONTROL).center().child(icon(sym, 16.0, fg));
        if enabled {
            b = materials::reveal(b, theme);
        }
        b
    };
    let back = subtle("back", Symbol::ArrowLeft, "Back", s.page != Page::Home).on_press(|s: &mut State| s.navigate(Page::Home));
    let toggle = subtle("pane-toggle", Symbol::LineHorizontal3, "Toggle navigation", true).on_press(|s: &mut State| s.pane_open = !s.pane_open);
    let logo = el().size(16.0, 16.0).radius(3.0).linear_gradient(135.0, [(0.0, c(theme.accent().light2)), (1.0, c(theme.accent().dark1))]);
    row()
        .key("title-bar")
        .w_full()
        .h(sizes::TITLE_BAR_HEIGHT)
        .px(4.0)
        .gap(4.0)
        .items_center()
        .child(back)
        .child(toggle)
        .child(row().ml(12.0).gap(16.0).items_center().child(logo).child(caption("WinUI 3 Gallery", t.text_fill_color_primary)))
        .child(el().ml(8.0).px(6.0).h(18.0).radius(radii::CONTROL).center().fill(c(t.subtle_fill_color_secondary)).child(caption("Rune · Linux", t.text_fill_color_secondary)))
}

fn search_box(s: &State, theme: &Theme) -> El {
    let t = theme.tokens();
    let field = controls::text_box(theme, "search", &s.search, "Search controls and samples", 288.0, true, |s: &mut State| &mut s.search).pr(36.0);
    let glyph = el().absolute(288.0 - 32.0, 0.0).size(32.0, 32.0).center().child(icon(Symbol::Search, 16.0, t.text_fill_color_secondary));
    el().size(288.0, 32.0).child(field).child(glyph)
}

fn nav_item(s: &State, theme: &Theme, page: Page, expanded: bool) -> El {
    let t = theme.tokens();
    let selected = s.page == page && s.query().is_empty();
    let mut content = row().gap(16.0).items_center().child(icon(page.icon(), 16.0, t.text_fill_color_primary));
    if expanded {
        content = content.child(body(page.nav_label(), t.text_fill_color_primary));
    }
    list_row(theme, &format!("nav-{}", page.id()), sizes::NAV_ITEM_HEIGHT, selected, content, move |s: &mut State| s.navigate(page)).label(page.nav_label())
}

fn nav_pane(s: &State, theme: &Theme) -> El {
    let t = theme.tokens();
    let expanded = s.pane_open;
    let width = if expanded { sizes::NAV_PANE_EXPANDED } else { sizes::NAV_PANE_COMPACT };
    let mut items = col().w_full().grow(1.0).scroll(ScrollSpec::default());
    for (header, group) in NAV_GROUPS {
        if let Some(h) = header {
            if expanded {
                items = items.child(row().h(40.0).pl(16.0).items_center().child(text(*h).font(TYPE_RAMP.body_strong.rune()).color(c(t.text_fill_color_primary))));
            } else {
                items = items.child(el().h(1.0).mx(4.0).my(4.0).fill(c(t.divider_stroke_color_default)));
            }
        }
        for &p in group.iter() {
            items = items.child(nav_item(s, theme, p, expanded));
        }
    }
    let footer = col().w_full().pb(4.0).child(el().h(1.0).mx(4.0).mb(4.0).fill(c(t.divider_stroke_color_default))).child(nav_item(s, theme, Page::Settings, expanded));
    let mut pane = col().key("nav-pane").role("navigation").label("Navigation").w(width).h_full();
    if expanded {
        pane = pane.child(row().w_full().h(48.0).px(16.0).items_center().child(search_box(s, theme)));
    } else {
        let find = el().key("search-compact").role("button").label("Search").size(40.0, 36.0).mx(4.0).radius(radii::CONTROL).center().child(icon(Symbol::Search, 16.0, t.text_fill_color_primary)).on_press(|s: &mut State| s.pane_open = true);
        pane = pane.child(materials::reveal(find, theme));
    }
    let pane = pane.child(items).child(footer);
    // Acrylic behind the pane (NavigationViewDefaultPaneBackground in Left mode over Mica).
    let tokens = theme.tokens();
    materials::acrylic(theme, tokens.acrylic_background_tint, tokens.acrylic_tint_opacity, 0.35).w(width).h_full().child(pane)
}

fn content_frame(s: &State, theme: &Theme) -> El {
    let t = theme.tokens();
    let page = if s.query().is_empty() { pages::page_view(s, theme, s.page) } else { pages::search_results(s, theme) };
    let scroll_key = if s.query().is_empty() { format!("scroll-{}", s.page.id()) } else { "scroll-search".to_string() };
    col()
        .key("content")
        .grow(1.0)
        .h_full()
        .fill(c(t.layer_fill_color_default))
        .border(1.0, c(t.card_stroke_color_default))
        .corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .clip_rounded(radii::OVERLAY)
        // Keyed per page so each page opens at its top.
        .child(col().key(scroll_key).w_full().grow(1.0).scroll(ScrollSpec::default()).child(page))
}
