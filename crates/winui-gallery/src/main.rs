//! WinUI 3 Gallery-style showcase app. See docs/ARCHITECTURE.md.
//!
//! `winui-gallery [--scene <page-id>] [--theme light|dark] [--snap out.png] [--at <secs>]
//!  [--accent #RRGGBB] [-- <rune args>]`

fn main() {
    let argv = winui_gallery::rune_args(std::env::args());
    winui_gallery::rune::App::new(winui_gallery::spec()).args(argv).run();
}
