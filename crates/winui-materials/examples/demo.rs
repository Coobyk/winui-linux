//! Material scenes: `--scene mica|acrylic|reveal|shadow [--theme light|dark] --snap out.png`.
use rune::{AppSpec, Color, Cx, El, col, el, row, text};
use winui_materials as mat;
use winui_theme::{Theme, ThemeMode};

struct State {
    scene: &'static str,
}

const SCENES: &[(&str, fn(&mut State))] = &[
    ("mica", |s| s.scene = "mica"),
    ("acrylic", |s| s.scene = "acrylic"),
    ("reveal", |s| s.scene = "reveal"),
    ("shadow", |s| s.scene = "shadow"),
];

fn main() {
    rune::run(AppSpec {
        scenes: SCENES,
        ..AppSpec::new("winui-materials", (720.0, 480.0), || State { scene: "mica" }, view)
    });
}

fn theme(cx: &mut Cx<State>) -> Theme {
    if cx.system_dark() {
        Theme::dark()
    } else {
        Theme::light()
    }
}

fn chip(label: &str, th: &Theme, body: El) -> El {
    col().gap(8.0)
        .child(text(label).font(rune::tok::type_::LABEL).color(mat::rc(th.tokens().text_fill_color_secondary)))
        .child(el().size(280.0, 160.0).clip_rounded(8.0).child(body))
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let th = theme(cx);
    let t = th.tokens();
    let body = match s.scene {
        "mica" => col().gap(16.0).padding(24.0)
            .child(chip("mica", &th, mat::mica(&th)))
            .child(chip("mica alt", &th, mat::mica_alt(&th))),
        "acrylic" => col().gap(16.0).padding(24.0)
            .child(chip("acrylic 60%", &th, mat::acrylic(&th, t.accent_fill_color_default, 0.6, 0.8)))
            .child(chip("nav acrylic", &th, mat::nav_acrylic(&th))),
        "reveal" => {
            let card = |hover: bool| {
                el().size(280.0, 72.0)
                    .radius(4.0)
                    .fill(mat::rc(t.card_background_fill_color_default))
                    .border(1.0, mat::rc(t.card_stroke_color_default))
                    .center()
                    .child(text(if hover { "reveal hover" } else { "rest" }).color(mat::rc(t.text_fill_color_primary)))
            };
            col().gap(16.0).padding(24.0)
                .child(card(false))
                .child(el().size(280.0, 72.0).child(card(true).fill(mat::reveal_fill(&th))).child(mat::reveal_border(&th, 4.0)))
        }
        _ => {
            let card = |depth: f32, label: &'static str| {
                col().gap(8.0)
                    .child(
                        el().size(96.0, 64.0)
                            .box_shadow(mat::ambient(&th, depth))
                            .child(
                                el().w_full().h_full()
                                    .box_shadow(mat::elevation(&th, depth))
                                    .radius(8.0)
                                    .fill(mat::rc(t.card_background_fill_color_default))
                                    .center()
                                    .child(text(label).font(rune::tok::type_::LABEL).color(mat::rc(t.text_fill_color_secondary))),
                            ),
                    )
            };
            col().gap(24.0).padding(32.0)
                .child(row().gap(24.0).children([card(2.0, "2"), card(4.0, "4"), card(8.0, "8")]))
                .child(row().gap(24.0).children([card(16.0, "16"), card(32.0, "32"), card(64.0, "64")]))
        }
    };
    let _ = ThemeMode::Light;
    let _ = Color::BLACK;
    el().w_full().h_full().fill(mat::rc(t.solid_background_fill_color_base_alt)).child(body)
}
