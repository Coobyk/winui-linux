//! winui-materials demo.
//!
//! ```text
//! cargo run -p winui-materials --example demo -- --scene acrylic --snap out.png
//! ```
//! Scenes: `mica`, `acrylic`, `reveal`, `shadow` (dark theme) and the same names with a
//! `-light` suffix. Without `--snap` it opens a window; click the tabs to switch scenes.

use rune::{App, AppSpec, Cx, El, col, el, row, text};
use winui_materials::{
    AcrylicBrush, MaterialExt, MicaBrush, MicaKind, Reveal, THEME_SHADOW_DEPTHS, acrylic, mica, rc,
    reveal,
};
use winui_theme::{Color, Theme};

#[derive(Clone, Copy, PartialEq)]
enum Scene {
    Mica,
    Acrylic,
    Reveal,
    Shadow,
}

struct State {
    scene: Scene,
    theme: Theme,
    reveal: Reveal,
}

const W: f32 = 760.0;
const H: f32 = 520.0;
const CARD: (f32, f32) = (300.0, 64.0);

fn init() -> State {
    State {
        scene: Scene::Mica,
        theme: Theme::dark(),
        reveal: Reveal::default(),
    }
}

fn ink(theme: &Theme) -> rune::Color {
    rc(theme.tokens().text_fill_color_primary)
}

fn label(theme: &Theme, s: &str) -> El {
    text(s.to_string()).color(ink(theme))
}

/// A colourful backdrop so blur, tint and luminosity are visible.
fn wallpaper() -> El {
    let c = |r, g, b| rc(Color::rgb(r, g, b));
    el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .linear_gradient(
            135f32.to_radians(),
            [
                (0.0, c(0x00, 0x5F, 0xB8)),
                (0.5, c(0x8E, 0x3C, 0xC7)),
                (1.0, c(0xF2, 0x8C, 0x28)),
            ],
        )
        .child(
            el().absolute(80.0, 120.0)
                .size(160.0, 160.0)
                .radius(80.0)
                .fill(c(0xFF, 0xD3, 0x00)),
        )
        .child(
            el().absolute(470.0, 260.0)
                .size(200.0, 120.0)
                .radius(12.0)
                .fill(c(0x10, 0x7C, 0x10)),
        )
        .child(
            el().absolute(330.0, 80.0)
                .size(60.0, 300.0)
                .fill(c(0xFF, 0xFF, 0xFF)),
        )
}

fn mica_scene(t: &Theme) -> El {
    let wall = Color::rgb(0x00, 0x5F, 0xB8);
    let swatch = |name: &str, e: El| {
        col().gap(8.0).child(label(t, name)).child(
            e.size(200.0, 140.0)
                .radius(8.0)
                .clip_rounded(8.0)
                .border(1.0, rc(t.tokens().card_stroke_color_default)),
        )
    };
    mica(t)
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .padding(32.0)
        .child(
            col()
                .gap(24.0)
                .child(label(t, "Mica (MicaKind.Base) — window surface"))
                .child(
                    row()
                        .gap(24.0)
                        .child(swatch("Base", MicaBrush::new(t, MicaKind::Base).el()))
                        .child(swatch("BaseAlt", MicaBrush::new(t, MicaKind::BaseAlt).el()))
                        .child(swatch(
                            "Base over #005FB8",
                            MicaBrush::new(t, MicaKind::Base).el_over(wall),
                        )),
                )
                .child(
                    el().size(640.0, 120.0)
                        .radius(8.0)
                        .fill(rc(t.tokens().card_background_fill_color_default))
                        .border(1.0, rc(t.tokens().card_stroke_color_default))
                        .padding(16.0)
                        .child(label(t, "CardBackgroundFillColorDefault on Mica")),
                ),
        )
}

fn acrylic_scene(t: &Theme) -> El {
    let panel = |name: &str, e: El| {
        col()
            .gap(8.0)
            .child(
                e.size(220.0, 180.0)
                    .border(1.0, rc(t.tokens().surface_stroke_color_default)),
            )
            .child(label(t, name))
    };
    el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .child(wallpaper())
        .child(
            row()
                .absolute(24.0, 140.0)
                .gap(24.0)
                .child(panel(
                    "AcrylicBackgroundFillColorDefault",
                    AcrylicBrush::background_default(t)
                        .el_rounded(8.0)
                        .radius(8.0),
                ))
                .child(panel(
                    "AcrylicBackgroundFillColorBase",
                    AcrylicBrush::background_base(t).el_rounded(8.0).radius(8.0),
                ))
                .child(panel(
                    "AccentAcrylicBackgroundFillColorDefault",
                    AcrylicBrush::accent_default(t).el_rounded(8.0).radius(8.0),
                ))
                .child(
                    el().absolute(0.0, 240.0).child(
                        acrylic(t, t.tokens().solid_background_fill_color_base, 0.5, None)
                            .size(712.0, 64.0),
                    ),
                ),
        )
}

fn reveal_scene(s: &State) -> El {
    let t = &s.theme;
    let card = || {
        el().size(CARD.0, CARD.1)
            .radius(4.0)
            .fill(rc(t.tokens().control_fill_color_default))
            .border(1.0, rc(t.tokens().control_stroke_color_default))
            .padding(16.0)
            .child(label(t, "Hover me"))
    };
    let lit = Reveal::track(
        card().child(reveal(t, &s.reveal, CARD.0, CARD.1, 4.0)),
        |s: &mut State| &mut s.reveal,
    );
    let fixed = Reveal {
        pointer: Some([60.0, 4.0]),
        pressed: false,
    };
    mica(t)
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .padding(32.0)
        .child(
            col()
                .gap(16.0)
                .child(label(
                    t,
                    "Reveal: hover light + border light follow the pointer",
                ))
                .child(lit)
                .child(label(t, "Pointer pinned at (60, 4):"))
                .child(card().child(reveal(t, &fixed, CARD.0, CARD.1, 4.0)))
                .child(label(t, "Pressed at (240, 40):"))
                .child(card().child(reveal(
                    t,
                    &Reveal {
                        pointer: Some([240.0, 40.0]),
                        pressed: true,
                    },
                    CARD.0,
                    CARD.1,
                    4.0,
                ))),
        )
}

fn shadow_scene(t: &Theme) -> El {
    let tile = |d: f32| {
        col()
            .gap(8.0)
            .items_center()
            .child(
                el().size(72.0, 72.0)
                    .radius(8.0)
                    .fill(rc(t.tokens().solid_background_fill_color_quarternary))
                    .theme_shadow(t, d),
            )
            .child(label(t, &format!("Z = {d}")))
    };
    mica(t)
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .padding(40.0)
        .child(
            col()
                .gap(48.0)
                .child(label(t, "ThemeShadow depth ramp"))
                .child(
                    row()
                        .gap(28.0)
                        .children(THEME_SHADOW_DEPTHS.iter().map(|&d| tile(d))),
                ),
        )
}

fn tab(s: &State, name: &'static str, scene: Scene) -> El {
    let t = &s.theme;
    let on = s.scene == scene;
    let fill = if on {
        t.tokens().accent_fill_color_default
    } else {
        t.tokens().control_fill_color_default
    };
    let fg = if on {
        t.tokens().text_on_accent_fill_color_primary
    } else {
        t.tokens().text_fill_color_primary
    };
    el().padding(8.0)
        .radius(4.0)
        .fill(rc(fill))
        .child(text(name).color(rc(fg)))
        .on_press(move |s: &mut State| s.scene = scene)
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let body = match s.scene {
        Scene::Mica => mica_scene(&s.theme),
        Scene::Acrylic => acrylic_scene(&s.theme),
        Scene::Reveal => reveal_scene(s),
        Scene::Shadow => shadow_scene(&s.theme),
    };
    el().key("root").size(W, H).child(body).child(
        row()
            .absolute(16.0, H - 48.0)
            .gap(8.0)
            .child(tab(s, "Mica", Scene::Mica))
            .child(tab(s, "Acrylic", Scene::Acrylic))
            .child(tab(s, "Reveal", Scene::Reveal))
            .child(tab(s, "Shadow", Scene::Shadow))
            .child(
                el().padding(8.0)
                    .radius(4.0)
                    .fill(rc(s.theme.tokens().control_fill_color_default))
                    .child(label(&s.theme, "Light/Dark"))
                    .on_press(|s: &mut State| s.theme = s.theme.toggled()),
            ),
    )
}

fn main() {
    App::new(AppSpec::<State> {
        scenes: &[
            ("mica", |s| s.scene = Scene::Mica),
            ("acrylic", |s| s.scene = Scene::Acrylic),
            ("reveal", |s| {
                s.scene = Scene::Reveal;
                s.reveal = Reveal {
                    pointer: Some([150.0, 30.0]),
                    pressed: false,
                };
            }),
            ("shadow", |s| s.scene = Scene::Shadow),
            ("mica-light", |s| {
                (s.scene, s.theme) = (Scene::Mica, Theme::light())
            }),
            ("acrylic-light", |s| {
                (s.scene, s.theme) = (Scene::Acrylic, Theme::light())
            }),
            ("reveal-light", |s| {
                (s.scene, s.theme) = (Scene::Reveal, Theme::light());
                s.reveal = Reveal {
                    pointer: Some([150.0, 30.0]),
                    pressed: false,
                };
            }),
            ("shadow-light", |s| {
                (s.scene, s.theme) = (Scene::Shadow, Theme::light())
            }),
        ],
        ..AppSpec::new("winui-materials demo", (W, H), init, view)
    })
    .run();
}
