//! Home and Settings.

use super::{Sample, c};
use crate::fallback::controls::{self as k, body, caption, icon, strong};
use crate::fallback::icons::Symbol;
use crate::fallback::materials;
use crate::fallback::theme::{TYPE_RAMP, Theme, ThemeMode, radii};
use crate::state::{ACCENTS, NAV_GROUPS, Page, State};
use rune::{El, col, el, row, text};

fn home(_: &State, t: &Theme) -> El {
    let tk = t.tokens();
    let a = t.accent();
    let hero = col().key("hero").w_full().h(260.0).px(36.0).pt(48.0).gap(8.0).radius(radii::OVERLAY)
        .linear_gradient(120.0, [(0.0, c(a.light3).alpha(0.55)), (0.5, c(a.light1).alpha(0.35)), (1.0, c(a.dark2).alpha(0.55))])
        .child(text("WinUI 3 Gallery").font(TYPE_RAMP.title_large.rune()).color(c(tk.text_fill_color_primary)))
        .child(body("Fluent Design on Linux, drawn by Rune.", tk.text_fill_color_secondary));
    let mut tiles = row().key("home-tiles").w_full().gap(12.0).flex_wrap();
    for (_, group) in NAV_GROUPS.iter().skip(1) {
        for &p in group.iter() {
            let tile = col().key(format!("tile-{}", p.id())).role("link").label(p.nav_label()).focusable(true).w(250.0).h(160.0).padding(24.0).gap(12.0).radius(radii::OVERLAY)
                .fill(c(tk.card_background_fill_color_default)).border(1.0, c(tk.card_stroke_color_default))
                .hover_fill(c(tk.control_fill_color_secondary)).press_fill(c(tk.control_fill_color_tertiary))
                .child(icon(p.icon(), 32.0, tk.accent_text_fill_color_primary))
                .child(strong(p.nav_label(), tk.text_fill_color_primary))
                .child(caption(p.description(), tk.text_fill_color_secondary).wrap())
                .on_press(move |s: &mut State| s.navigate(p));
            tiles = tiles.child(materials::elevate(tile, t, 2.0));
        }
    }
    col().w_full().gap(24.0).child(hero).child(strong("Recently added samples", tk.text_fill_color_primary)).child(tiles)
}

fn settings_card(t: &Theme, key: &str, sym: Symbol, title: &str, desc: &str, control: El) -> El {
    let tk = t.tokens();
    row().key(key).w_full().min_h(68.0).px(16.0).py(12.0).gap(16.0).items_center().radius(radii::CONTROL)
        .fill(c(tk.card_background_fill_color_default)).border(1.0, c(tk.card_stroke_color_default))
        .child(icon(sym, 20.0, tk.text_fill_color_primary))
        .child(col().grow(1.0).child(body(title, tk.text_fill_color_primary)).child(caption(desc, tk.text_fill_color_secondary)))
        .child(control)
}

fn theme_settings(s: &State, t: &Theme) -> El {
    let dark = t.mode() == ThemeMode::Dark;
    let radios = row().key("theme-radios").gap(16.0)
        .child(k::radio(t, "theme-light", "Light", s.theme_override == Some(ThemeMode::Light), |s: &mut State| s.theme_override = Some(ThemeMode::Light)))
        .child(k::radio(t, "theme-dark", "Dark", s.theme_override == Some(ThemeMode::Dark), |s: &mut State| s.theme_override = Some(ThemeMode::Dark)))
        .child(k::radio(t, "theme-system", "Use system setting", s.theme_override.is_none(), |s: &mut State| s.theme_override = None));
    col().w_full().gap(4.0)
        .child(settings_card(t, "setting-dark", Symbol::WeatherMoon, "Dark mode", "Switch between the light and dark app theme.",
            k::toggle_switch(t, "theme-toggle", None, dark, move |s: &mut State| s.theme_override = Some(if dark { ThemeMode::Light } else { ThemeMode::Dark }))))
        .child(settings_card(t, "setting-theme", Symbol::PaintBrush, "App theme", "Select which app theme to display.", radios))
}

fn accent_picker(s: &State, t: &Theme) -> El {
    let tk = t.tokens();
    let mut grid = row().key("accent-picker").role("radiogroup").label("Accent colour").gap(4.0).flex_wrap().max_w(400.0);
    for (i, (name, col_)) in ACCENTS.iter().enumerate() {
        let sel = s.accent == *col_;
        let mut chip = el().key(format!("accent-{i}")).role("radio").label(*name).checked(sel).size(44.0, 44.0).radius(radii::CONTROL).center()
            .fill(c(*col_)).on_press(move |s: &mut State| s.accent = ACCENTS[i].1);
        chip = if sel {
            chip.border(2.0, c(tk.text_fill_color_primary)).child(icon(Symbol::Checkmark, 16.0, crate::fallback::theme::Color::rgb(0xFF, 0xFF, 0xFF)))
        } else {
            chip.border(1.0, c(tk.control_stroke_color_default))
        };
        grid = grid.child(chip.focusable(true));
    }
    let current = ACCENTS.iter().find(|(_, col_)| *col_ == s.accent).map(|(n, _)| *n).unwrap_or("Custom");
    col().w_full().gap(4.0)
        .child(settings_card(t, "setting-accent", Symbol::Color, "Accent color", &format!("{current} · {}", s.accent.to_hex()),
            k::button(t, "accent-reset", "Reset", k::ButtonStyle::Standard, s.accent != ACCENTS[0].1, |s: &mut State| s.accent = ACCENTS[0].1)))
        .child(col().w_full().padding(16.0).gap(12.0).radius(radii::CONTROL).fill(c(tk.card_background_fill_color_secondary)).border(1.0, c(tk.card_stroke_color_default))
            .child(grid)
            .child(row().gap(12.0).child(k::button(t, "accent-preview", "Accent button", k::ButtonStyle::Accent, true, |_: &mut State| {}))
                .child(k::checkbox(t, "accent-check", "Checked", Some(true), true, |_: &mut State| {}))
                .child(k::toggle_switch(t, "accent-switch", None, true, |_: &mut State| {}))))
}

fn about(_: &State, t: &Theme) -> El {
    let tk = t.tokens();
    settings_card(t, "setting-about", Symbol::Info, "About", "winui-linux · WinUI 3 Fluent Design ported to Rune. Icons: Fluent System Icons (MIT).", caption(env!("CARGO_PKG_VERSION"), tk.text_fill_color_secondary))
}

pub static SAMPLES: &[Sample] = &[
    Sample { id: "home", page: Page::Home, title: "Home", keywords: "home gallery start", view: home },
    Sample { id: "theme", page: Page::Settings, title: "Theme mode", keywords: "theme dark light mode settings appearance", view: theme_settings },
    Sample { id: "accent", page: Page::Settings, title: "Accent color", keywords: "accent color colour picker settings personalize", view: accent_picker },
    Sample { id: "about", page: Page::Settings, title: "About", keywords: "about version license", view: about },
];
