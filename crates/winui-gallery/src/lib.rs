//! WinUI 3 Gallery-style showcase on Rune. See docs/ARCHITECTURE.md and the crate README.

pub mod fallback;
pub mod pages;
pub mod shell;
pub mod state;

pub use rune;
pub use state::{ACCENTS, Page, State};

use rune::AppSpec;

/// One scene per page id (`--scene <page-id>`), plus `search` (a filtered results page).
pub const SCENES: &[(&str, fn(&mut State))] = &[
    ("home", |s| s.page = Page::Home),
    ("buttons", |s| s.page = Page::Buttons),
    ("input", |s| s.page = Page::Input),
    ("collections", |s| s.page = Page::Collections),
    ("menus", |s| s.page = Page::Menus),
    ("navigation", |s| s.page = Page::Navigation),
    ("status", |s| s.page = Page::Status),
    ("media", |s| s.page = Page::Media),
    ("styles-icons", |s| s.page = Page::StylesIcons),
    ("styles-colors", |s| s.page = Page::StylesColors),
    ("styles-type", |s| s.page = Page::StylesType),
    ("settings", |s| s.page = Page::Settings),
    ("search", |s| s.set_search("button")),
];

pub fn spec() -> AppSpec<State> {
    AppSpec {
        scenes: SCENES,
        min_size: Some((640.0, 480.0)),
        families: ("Segoe UI Variable Text, Segoe UI, Inter", "Cascadia Mono, ui-monospace"),
        ..AppSpec::new("WinUI 3 Gallery", (1280.0, 800.0), State::from_args, shell::view)
    }
}

/// argv for Rune: everything after a bare `--` is passed through verbatim.
/// Rune's argv from the process's: drops the program name and the first bare `--`.
pub fn rune_args(argv: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = false;
    argv.into_iter()
        .skip(1)
        .filter(|a| {
            let sep = !seen && a == "--";
            seen |= sep;
            !sep
        })
        .collect()
}
