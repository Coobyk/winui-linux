//! Pages: a registry of labelled samples per control family, plus Home, Styles and Settings.

mod app;
mod basic;
mod more;
mod styles;

pub use more::MAX_TABS;

use crate::fallback::controls::{body, caption, strong};
use crate::fallback::theme::{Color, TYPE_RAMP, Theme, radii};
use crate::state::{Page, State};
use rune::{El, col, row, text};

pub(crate) fn c(x: Color) -> rune::Color {
    x.into()
}

/// One labelled sample: a WinUI 3 Gallery `ControlExample`.
pub struct Sample {
    pub id: &'static str,
    pub page: Page,
    pub title: &'static str,
    /// Extra search terms (control names).
    pub keywords: &'static str,
    pub view: fn(&State, &Theme) -> El,
}

impl Sample {
    pub fn matches(&self, q: &str) -> bool {
        let q = q.to_lowercase();
        q.split_whitespace().all(|w| {
            self.title.to_lowercase().contains(w) || self.keywords.to_lowercase().contains(w) || self.page.nav_label().to_lowercase().contains(w)
        })
    }
}

pub fn samples() -> impl Iterator<Item = &'static Sample> {
    basic::SAMPLES.iter().chain(more::SAMPLES).chain(styles::SAMPLES).chain(app::SAMPLES)
}

pub fn samples_for(page: Page) -> impl Iterator<Item = &'static Sample> {
    samples().filter(move |s| s.page == page)
}

/// Samples matching the search query.
pub fn search(q: &str) -> Vec<&'static Sample> {
    if q.trim().is_empty() {
        return Vec::new();
    }
    samples().filter(|s| s.matches(q) || (s.id == "icons" && !styles::icons_named(q).is_empty())).collect()
}

pub fn page_header(theme: &Theme, title: &str, description: &str) -> El {
    let t = theme.tokens();
    col().key("page-header").gap(8.0).child(text(title).font(TYPE_RAMP.title.rune()).color(c(t.text_fill_color_primary))).child(body(description, t.text_fill_color_secondary))
}

/// A sample card: header text above a rounded card holding the sample.
pub fn sample_card(s: &State, theme: &Theme, sample: &Sample) -> El {
    let t = theme.tokens();
    let card = col()
        .w_full()
        .padding(12.0)
        .radius(radii::OVERLAY)
        .fill(c(t.card_background_fill_color_default))
        .border(1.0, c(t.card_stroke_color_default))
        .child((sample.view)(s, theme));
    col().key(format!("sample-{}", sample.id)).label(sample.title).w_full().gap(12.0).child(strong(sample.title, t.text_fill_color_primary)).child(card)
}

fn page_body(theme: &Theme) -> El {
    let _ = theme;
    col().w_full().max_w(1100.0).px(36.0).pt(24.0).pb(36.0).gap(24.0)
}

pub fn page_view(s: &State, theme: &Theme, page: Page) -> El {
    let mut body_el = page_body(theme).key(format!("page-{}", page.id())).child(page_header(theme, page.title(), page.description()));
    if page == Page::Home {
        body_el = page_body(theme).key("page-home");
    }
    for sample in samples_for(page) {
        body_el = body_el.child(if page == Page::Home { (sample.view)(s, theme) } else { sample_card(s, theme, sample) });
    }
    body_el
}

pub fn search_results(s: &State, theme: &Theme) -> El {
    let t = theme.tokens();
    let q = s.query();
    let hits = search(q);
    let mut body_el = page_body(theme).key("page-search").child(page_header(theme, &format!("Results for \u{201C}{q}\u{201D}"), &format!("{} matching samples", hits.len())));
    if hits.is_empty() {
        body_el = body_el.child(row().key("no-results").child(body("No results found. Try another control name.", t.text_fill_color_secondary)));
    }
    for sample in hits {
        body_el = body_el.child(col().w_full().gap(4.0).child(caption(sample.page.nav_label(), t.text_fill_color_tertiary)).child(sample_card(s, theme, sample)));
    }
    body_el
}
