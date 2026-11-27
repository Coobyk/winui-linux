//! WinUI 3 Gallery-style showcase app. See docs/ARCHITECTURE.md.
//!
//! `--scene <page>` snaps that page; `--theme light|dark` picks the mode.
fn main() {
    rune::run(winui_gallery::spec());
}
