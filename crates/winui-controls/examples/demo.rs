//! Every control in every common state.
//!
//! `cargo run -p winui-controls --example demo -- --scene button --snap out.png`
//! (`--theme dark` for dark). Scenes are control names, see `SCENES`.

use rune::{AppSpec, Cx, El, col, row, text};
use winui_controls::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Scene {
    #[default]
    Button,
}

#[derive(Default)]
struct State {
    scene: Scene,
    clicks: u32,
}

fn caption(theme: &Theme, s: &str) -> El {
    text(s.to_string()).font(font(theme.type_ramp().caption)).color(rc(theme.tokens().text_fill_color_secondary)).w(96.0)
}

/// One labelled row per common state plus a keyboard-focused one.
fn states(theme: &Theme, f: impl Fn(VisualState) -> El, focused: impl Fn() -> El) -> El {
    let mut c = col().gap(12.0);
    for s in VisualState::ALL {
        c = c.child(row().gap(16.0).center_y().child(caption(theme, s.name())).child(f(s)));
    }
    c.child(row().gap(16.0).center_y().child(caption(theme, "Focused")).child(focused()))
}

fn scene_view(s: &State, theme: &Theme) -> (&'static str, El) {
    match s.scene {
        Scene::Button => (
            "Button",
            states(
                theme,
                |st| {
                    row()
                        .gap(16.0)
                        .child(Button::new("Standard XAML button").state(st).view(theme))
                        .child(Button::new("Accent style button").accent().state(st).view(theme))
                },
                || row().gap(16.0).child(Button::new("Standard XAML button").focused(true).view(theme)).child(Button::new("Accent").accent().focused(true).view(theme)),
            ),
        ),
    }
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let theme = if cx.system_dark() { Theme::dark() } else { Theme::light() };
    let t = theme.tokens();
    let (title, body) = scene_view(s, &theme);
    col()
        .w_full()
        .h_full()
        .fill(rc(t.solid_background_fill_color_base))
        .padding(24.0)
        .gap(20.0)
        .child(text(title).font(font(theme.type_ramp().subtitle)).color(rc(t.text_fill_color_primary)))
        .child(body)
        .child(text(format!("clicks: {}", s.clicks)).font(font(theme.type_ramp().caption)).color(rc(t.text_fill_color_tertiary)))
}

fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[("button", |s| s.scene = Scene::Button)],
        ..AppSpec::new("winui-controls", (760.0, 420.0), State::default, view)
    });
}
