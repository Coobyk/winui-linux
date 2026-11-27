//! Styles pages: icons grid, colour swatches, type ramp.

use super::{Sample, c};
use crate::fallback::controls::{body, caption, icon, strong};
use crate::fallback::icons::Symbol;
use crate::fallback::theme::{Color, TYPE_RAMP, TextStyle, Theme, radii};
use crate::state::{Page, State};
use rune::{El, col, el, row, text};

/// Icons whose name contains the query.
pub fn icons_named(q: &str) -> Vec<Symbol> {
    let q = q.trim().to_lowercase();
    Symbol::ALL.iter().copied().filter(|s| s.name().to_lowercase().contains(&q)).collect()
}

/// The icons the grid shows: name matches, or all when nothing matches by name (empty query, or
/// the Icons sample was found by its own title or keywords).
pub fn icons_matching(q: &str) -> Vec<Symbol> {
    let hits = icons_named(q);
    if hits.is_empty() { Symbol::ALL.to_vec() } else { hits }
}

fn icons_grid(s: &State, t: &Theme) -> El {
    let tk = t.tokens();
    let mut grid = row().key("icons-grid").w_full().gap(8.0).flex_wrap();
    for sym in icons_matching(s.query()) {
        grid = grid.child(
            col().key(format!("icon-{}", sym.name())).role("img").label(sym.name()).size(112.0, 96.0).radius(radii::CONTROL).center().gap(12.0)
                .fill(c(tk.control_fill_color_secondary)).border(1.0, c(tk.card_stroke_color_default)).hover_fill(c(tk.control_fill_color_tertiary))
                .child(icon(sym, 24.0, tk.text_fill_color_primary))
                .child(caption(sym.name(), tk.text_fill_color_secondary)),
        );
    }
    grid
}

fn swatch(t: &Theme, name: &str, color: Color) -> El {
    let tk = t.tokens();
    // Translucent brushes are shown over the solid base so they read as they do in use.
    let chip = el().w_full().h(56.0).corners([radii::CONTROL, radii::CONTROL, 0.0, 0.0]).fill(c(tk.solid_background_fill_color_base))
        .child(el().absolute(0.0, 0.0).w_full().h_full().corners([radii::CONTROL, radii::CONTROL, 0.0, 0.0]).fill(c(color)));
    col().key(format!("swatch-{name}")).label(name).w(200.0).radius(radii::CONTROL).border(1.0, c(tk.card_stroke_color_default)).clip_rounded(radii::CONTROL)
        .child(chip)
        .child(col().w_full().px(8.0).py(6.0).fill(c(tk.solid_background_fill_color_tertiary))
            .child(caption(name, tk.text_fill_color_primary))
            .child(caption(format!("{} · {:.0}%", color.to_hex(), color.a * 100.0), tk.text_fill_color_secondary)))
}

fn swatch_group(t: &Theme, title: &str, items: &[(&str, Color)]) -> El {
    let mut g = row().w_full().gap(8.0).flex_wrap();
    for (n, col_) in items {
        g = g.child(swatch(t, n, *col_));
    }
    col().w_full().gap(8.0).child(strong(title, t.tokens().text_fill_color_primary)).child(g)
}

fn colors(_: &State, t: &Theme) -> El {
    let k = t.tokens();
    let a = t.accent();
    col().key("swatches").w_full().gap(20.0)
        .child(swatch_group(t, "Text", &[("TextFillColorPrimary", k.text_fill_color_primary), ("TextFillColorSecondary", k.text_fill_color_secondary), ("TextFillColorTertiary", k.text_fill_color_tertiary), ("TextFillColorDisabled", k.text_fill_color_disabled), ("AccentTextFillColorPrimary", k.accent_text_fill_color_primary)]))
        .child(swatch_group(t, "Fill", &[("ControlFillColorDefault", k.control_fill_color_default), ("ControlFillColorSecondary", k.control_fill_color_secondary), ("ControlFillColorTertiary", k.control_fill_color_tertiary), ("ControlStrongFillColorDefault", k.control_strong_fill_color_default), ("SubtleFillColorSecondary", k.subtle_fill_color_secondary), ("AccentFillColorDefault", k.accent_fill_color_default)]))
        .child(swatch_group(t, "Stroke", &[("ControlStrokeColorDefault", k.control_stroke_color_default), ("ControlStrokeColorSecondary", k.control_stroke_color_secondary), ("ControlStrongStrokeColorDefault", k.control_strong_stroke_color_default), ("CardStrokeColorDefault", k.card_stroke_color_default), ("DividerStrokeColorDefault", k.divider_stroke_color_default)]))
        .child(swatch_group(t, "Background", &[("SolidBackgroundFillColorBase", k.solid_background_fill_color_base), ("SolidBackgroundFillColorSecondary", k.solid_background_fill_color_secondary), ("LayerFillColorDefault", k.layer_fill_color_default), ("CardBackgroundFillColorDefault", k.card_background_fill_color_default), ("SmokeFillColorDefault", k.smoke_fill_color_default)]))
        .child(swatch_group(t, "Signal", &[("SystemFillColorSuccess", k.system_fill_color_success), ("SystemFillColorCaution", k.system_fill_color_caution), ("SystemFillColorCritical", k.system_fill_color_critical), ("SystemFillColorAttention", k.system_fill_color_attention), ("SystemFillColorSolidNeutral", k.system_fill_color_solid_neutral)]))
        .child(swatch_group(t, "Accent ramp", &[("SystemAccentColorDark3", a.dark3), ("SystemAccentColorDark2", a.dark2), ("SystemAccentColorDark1", a.dark1), ("SystemAccentColor", a.base), ("SystemAccentColorLight1", a.light1), ("SystemAccentColorLight2", a.light2), ("SystemAccentColorLight3", a.light3)]))
}

fn type_ramp(_: &State, t: &Theme) -> El {
    let tk = t.tokens();
    let ramp: [(&str, TextStyle); 8] = [
        ("Caption", TYPE_RAMP.caption), ("Body", TYPE_RAMP.body), ("Body Strong", TYPE_RAMP.body_strong), ("Body Large", TYPE_RAMP.body_large),
        ("Subtitle", TYPE_RAMP.subtitle), ("Title", TYPE_RAMP.title), ("Title Large", TYPE_RAMP.title_large), ("Display", TYPE_RAMP.display),
    ];
    let mut colm = col().key("type-ramp").w_full();
    for (i, (name, st)) in ramp.into_iter().enumerate() {
        let weight = if st.weight >= 600 { "Semibold" } else { "Regular" };
        let r = row().key(format!("type-{}", name.to_lowercase().replace(' ', "-"))).w_full().py(12.0).gap(24.0).items_center()
            .child(text(name).font(st.rune()).color(c(tk.text_fill_color_primary)).grow(1.0))
            .child(body(format!("{weight} {}/{}", st.size, st.line_height), tk.text_fill_color_secondary).w(200.0));
        if i > 0 {
            colm = colm.child(el().w_full().h(1.0).fill(c(tk.divider_stroke_color_default)));
        }
        colm = colm.child(r);
    }
    colm
}

pub static SAMPLES: &[Sample] = &[
    Sample { id: "icons", page: Page::StylesIcons, title: "Fluent System Icons", keywords: "icon icons symbol glyph iconography", view: icons_grid },
    Sample { id: "colors", page: Page::StylesColors, title: "Theme colour brushes", keywords: "color colour brush swatch palette accent", view: colors },
    Sample { id: "type-ramp", page: Page::StylesType, title: "Type ramp", keywords: "typography type font text ramp", view: type_ramp },
];
