//! Materials showcase. Headless: `cargo run -p winui-materials --example demo -- --scene acrylic --snap out.png`
//! (add `--theme dark` for dark theme). Scenes: mica, acrylic, reveal, shadow.

use rune::{AppSpec, Color, Cx, El, TextStyle, col, el, row, text};
use winui_materials::acrylic::Acrylic;
use winui_materials::color::to_rune;
use winui_materials::mica::mica_alt;
use winui_materials::shadow::DEPTHS;
use winui_materials::theme::{Color as TColor, Theme};
use winui_materials::{ElevationExt, RevealLight, mica, reveal};

#[derive(Clone, Copy, Default, PartialEq)]
enum Scene {
    #[default]
    Mica,
    Acrylic,
    Reveal,
    Shadow,
}

struct State {
    scene: Scene,
    rows: Vec<RevealLight>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            scene: Scene::Mica,
            rows: (0..5)
                .map(|_| RevealLight::with_size([320.0, 40.0]))
                .collect(),
        }
    }
}

const BODY: TextStyle = TextStyle::new(14.0, 400.0);
const SUBTITLE: TextStyle = TextStyle::new(20.0, 600.0);

fn label(theme: &Theme, s: &str, style: TextStyle) -> El {
    text(s)
        .font(style)
        .color(to_rune(theme.tokens().text_fill_color_primary))
}

fn caption(theme: &Theme, s: &str) -> El {
    text(s)
        .font(TextStyle::new(12.0, 400.0))
        .color(to_rune(theme.tokens().text_fill_color_secondary))
}

fn card(theme: &Theme) -> El {
    let t = theme.tokens();
    el().radius(8.0)
        .fill(to_rune(t.card_background_fill_color_default))
        .border(1.0, to_rune(t.card_stroke_color_default))
}

fn mica_scene(theme: &Theme) -> El {
    let title_bar = mica_alt(theme)
        .w_full()
        .h(48.0)
        .px(16.0)
        .center_y()
        .child(label(theme, "Mica Alt title bar", BODY));
    let body = col()
        .gap(12.0)
        .padding(24.0)
        .child(label(theme, "Mica", SUBTITLE))
        .child(card(theme).w(360.0).padding(16.0).child(caption(
            theme,
            "Opaque base colour + faint procedural noise.",
        )));
    mica(theme)
        .w_full()
        .h_full()
        .child(col().w_full().h_full().child(title_bar).child(body))
}

fn blob(x: f32, y: f32, r: f32, c: Color) -> El {
    el().absolute(x - r, y - r)
        .size(2.0 * r, 2.0 * r)
        .radius(r)
        .fill(c)
}

fn menu(theme: &Theme, acrylic: Acrylic, title: &str) -> El {
    let items = ["Cut", "Copy", "Paste", "Select all"];
    let t = theme.tokens();
    let mut list = col()
        .gap(2.0)
        .padding(6.0)
        .child(caption(theme, title).px(10.0).py(6.0));
    for item in items {
        list = list.child(
            row()
                .h(32.0)
                .px(10.0)
                .center_y()
                .radius(4.0)
                .child(label(theme, item, BODY)),
        );
    }
    acrylic
        .radius(8.0)
        .el()
        .w(200.0)
        .border(1.0, to_rune(t.control_stroke_color_default))
        .elevation(32.0, theme.mode())
        .child(list)
}

fn acrylic_scene(theme: &Theme) -> El {
    let bg = el()
        .w_full()
        .h_full()
        .fill(to_rune(theme.tokens().solid_background_fill_color_base))
        .child(blob(
            110.0,
            120.0,
            90.0,
            to_rune(TColor::rgb(0x00, 0x78, 0xD4)),
        ))
        .child(blob(
            300.0,
            250.0,
            110.0,
            to_rune(TColor::rgb(0xE3, 0x00, 0x8C)),
        ))
        .child(blob(
            470.0,
            110.0,
            80.0,
            to_rune(TColor::rgb(0xFF, 0xB9, 0x00)),
        ))
        .child(blob(
            520.0,
            320.0,
            70.0,
            to_rune(TColor::rgb(0x00, 0xB2, 0x94)),
        ));
    let panels = row()
        .absolute(40.0, 60.0)
        .gap(32.0)
        .child(menu(
            theme,
            Acrylic::in_app_default(theme),
            "In-app default",
        ))
        .child(menu(theme, Acrylic::in_app_base(theme), "In-app base"))
        .child(menu(
            theme,
            Acrylic::in_app_default(theme).fallback(true),
            "Fallback",
        ));
    bg.child(panels)
}

fn reveal_scene(theme: &Theme, s: &State) -> El {
    let names = ["Home", "Documents", "Pictures", "Music", "Settings"];
    let mut list = col().w(320.0).gap(4.0);
    for (light, name) in s.rows.iter().zip(names) {
        list = list.child(
            reveal(theme, light)
                .w_full()
                .h(40.0)
                .px(16.0)
                .center_y()
                .child(label(theme, name, BODY)),
        );
    }
    mica(theme).w_full().h_full().child(
        col()
            .gap(12.0)
            .padding(24.0)
            .child(label(theme, "Reveal", SUBTITLE))
            .child(list),
    )
}

fn shadow_scene(theme: &Theme) -> El {
    let fill = to_rune(Acrylic::in_app_default(theme).fallback);
    let mut grid = col().gap(36.0);
    for depths in DEPTHS.chunks(4) {
        let mut line = row().gap(36.0);
        for &d in depths {
            let tile = el()
                .size(112.0, 80.0)
                .radius(8.0)
                .fill(fill)
                .elevation(d, theme.mode())
                .padding(12.0)
                .child(label(theme, &format!("Depth {d}"), BODY));
            line = line.child(tile);
        }
        grid = grid.child(line);
    }
    mica(theme).w_full().h_full().child(
        col()
            .gap(24.0)
            .padding(32.0)
            .child(label(theme, "Elevation", SUBTITLE))
            .child(grid),
    )
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let theme = if cx.system_dark() {
        Theme::dark()
    } else {
        Theme::light()
    };
    match s.scene {
        Scene::Mica => mica_scene(&theme),
        Scene::Acrylic => acrylic_scene(&theme),
        Scene::Reveal => reveal_scene(&theme, s),
        Scene::Shadow => shadow_scene(&theme),
    }
}

fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[
            ("mica", |s| s.scene = Scene::Mica),
            ("acrylic", |s| s.scene = Scene::Acrylic),
            ("reveal", |s| {
                s.scene = Scene::Reveal;
                s.rows[1].point_at(150.0, 20.0);
            }),
            ("shadow", |s| s.scene = Scene::Shadow),
        ],
        ..AppSpec::new("winui-materials", (720.0, 480.0), State::default, view)
    });
}
