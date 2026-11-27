//! `winui-apps`: replicas of real Windows 11 app windows built on the winui-*
//! crates, for side-by-side comparison with the shots in `docs/reference/`.
//!
//! `cargo run -p winui-gallery --bin winui-apps -- --scene calc --theme dark --snap out.png`

#![forbid(unsafe_code)]

#[path = "../../fluent.rs"]
mod fluent;
#[path = "../../controls.rs"]
#[allow(dead_code)]
mod controls;

mod calc;
mod chrome;
mod clock;
mod settings;
mod snip;
mod store;
mod terminal;

use rune::{AppSpec, Cx, El};
use winui_theme::{AccentRamp, Theme, ThemeMode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum App {
    Calc,
    ClockTimer,
    ClockSettings,
    Settings,
    Store,
    Snip,
    Terminal,
}

pub struct State {
    app: App,
}

impl Default for State {
    fn default() -> Self {
        State { app: App::Calc }
    }
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let _ = cx;
    let t = Theme::new(ThemeMode::Dark, AccentRamp::DEFAULT);
    match s.app {
        App::Calc => calc::view(&t),
        App::ClockTimer => clock::timer(&t),
        App::ClockSettings => clock::settings(&t),
        App::Settings => settings::view(&t),
        App::Store => store::view(&t),
        App::Snip => snip::view(&t),
        App::Terminal => terminal::view(&t),
    }
}

macro_rules! scenes {
    ($($id:literal => $a:expr),* $(,)?) => { &[$(($id, |s: &mut State| s.app = $a)),*] };
}

pub fn spec() -> AppSpec<State> {
    AppSpec {
        scenes: scenes![
            "calc" => App::Calc,
            "clock-timer" => App::ClockTimer,
            "clock-settings" => App::ClockSettings,
            "settings" => App::Settings,
            "store" => App::Store,
            "snip" => App::Snip,
            "terminal" => App::Terminal,
        ],
        families: (fluent::families(), "ui-monospace"),
        min_size: Some((320.0, 200.0)),
        ..AppSpec::new("Windows 11", (1201.0, 933.0), State::default, view)
    }
}

fn main() {
    rune::run(spec());
}
