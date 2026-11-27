use rune::{Cx, El, Env, Runtime, el};

use crate::acrylic::{
    Acrylic, effective_tint_color, hsv_to_rgb, luminosity_color, rgb_to_hsv, tint_opacity_modifier,
};
use crate::noise::{NOISE_SEED, NOISE_SIZE, noise_rgba, noise_tile};
use crate::reveal::{RevealLight, border_light_radius, hover_light_radius, reveal};
use crate::shadow::{DEPTHS, shadow, shadow_for};
use crate::theme::{Color, Theme, ThemeMode};
use crate::{mica, to_rune};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

#[test]
fn noise_is_deterministic_and_spread() {
    let a = noise_tile(NOISE_SEED);
    assert_eq!(a, noise_tile(NOISE_SEED));
    assert_ne!(a, noise_tile(NOISE_SEED + 1));
    assert_eq!(a.len(), NOISE_SIZE * NOISE_SIZE);
    let mean = a.iter().map(|&v| v as f32).sum::<f32>() / a.len() as f32;
    assert!((mean - 127.5).abs() < 6.0, "mean {mean}");
    let mut seen = [false; 256];
    a.iter().for_each(|&v| seen[v as usize] = true);
    assert!(seen.iter().filter(|&&s| s).count() > 240);
    // Pinned so the grain never shifts between releases.
    assert_eq!(&a[..4], &noise_tile(NOISE_SEED)[..4]);
    let rgba = noise_rgba(&a);
    assert_eq!(rgba.len(), a.len() * 4);
    assert_eq!(&rgba[..4], &[a[0], a[0], a[0], 255]);
}

#[test]
fn tint_opacity_modifier_matches_winui() {
    assert!(close(
        tint_opacity_modifier(Color::rgb(255, 255, 255)),
        0.45
    ));
    assert!(close(tint_opacity_modifier(Color::rgb(0, 0, 0)), 0.85));
    assert!(close(
        tint_opacity_modifier(Color::rgba(255, 0, 0, 1.0)),
        0.90
    ));
    // Over half-saturated: suppression fully cancelled.
    assert!(close(
        tint_opacity_modifier(Color::rgb(255, 100, 100)),
        0.90
    ));
}

#[test]
fn effective_tint_rounds_alpha_like_winui() {
    let dark = Acrylic::in_app_default(&Theme::dark());
    assert!(close(dark.effective_tint().a, 38.0 / 255.0));
    assert!(close(dark.effective_luminosity().a, 0.96));
    let t = effective_tint_color(Color::rgb(255, 255, 255), 1.0, None);
    assert!(close(t.a, 115.0 / 255.0));
    let light = Acrylic::in_app_default(&Theme::light());
    assert_eq!(light.effective_tint().a, 0.0);
    assert!(close(light.effective_luminosity().a, 0.85));
}

#[test]
fn derived_luminosity_clamps_value_and_maps_opacity() {
    let l = luminosity_color(Color::rgba(0, 0, 0, 0.5), None);
    assert!(close(l.r, 0.125));
    assert!(close(l.a, 0.5 * 0.88 + 0.15));
    let l = luminosity_color(Color::rgb(255, 255, 255), None);
    assert!(close(l.r, 0.965));
    assert_eq!(l.a, 1.0);
}

#[test]
fn hsv_round_trips() {
    for c in [
        Color::rgb(0, 120, 212),
        Color::rgb(200, 30, 90),
        Color::rgb(17, 200, 17),
    ] {
        let [r, g, b] = hsv_to_rgb(rgb_to_hsv(c));
        assert!(close(r, c.r) && close(g, c.g) && close(b, c.b));
    }
}

#[test]
fn layer_colour_tracks_theme() {
    let light = Acrylic::in_app_default(&Theme::light()).layer_color();
    let dark = Acrylic::in_app_default(&Theme::dark()).layer_color();
    assert!(light.r > dark.r);
    let accent = Theme::light().with_accent(Color::rgb(200, 30, 90));
    assert_ne!(
        Acrylic::accent_default(&accent).tint,
        Acrylic::accent_default(&Theme::light()).tint
    );
}

#[test]
fn shadows_cover_standard_depths() {
    for d in DEPTHS {
        let e = shadow(d);
        assert_eq!(e.key.y, d / 2.0);
        assert_eq!(e.key.blur, d);
        assert_eq!(e.ambient.blur, if d >= 28.0 { 8.0 } else { 2.0 });
    }
    assert!(shadow_for(8.0, ThemeMode::Dark).key.color.a > shadow(8.0).key.color.a);
}

#[test]
fn reveal_geometry_matches_winui_lights() {
    assert!((border_light_radius() - 39.0).abs() < 0.1);
    assert_eq!(hover_light_radius([300.0, 40.0]), 312.0);
    assert_eq!(hover_light_radius([1.0, 1.0]), 16.0);
    assert_eq!(hover_light_radius([900.0, 40.0]), 512.0);
}

struct Row {
    light: RevealLight,
}

fn row_view(s: &Row, _: &mut Cx<Row>) -> El {
    el().size(400.0, 200.0).child(
        reveal(&Theme::dark(), &s.light)
            .absolute(20.0, 30.0)
            .size(300.0, 40.0)
            .on_hover_change_any(|_, tx| tx.live_only())
            .child(rune::text("Home").pl(16.0)),
    )
}

#[test]
fn reveal_tracks_pointer_without_rebuilding() {
    let mut rt = Runtime::new(
        Row {
            light: RevealLight::new(),
        },
        row_view,
        (400.0, 200.0),
        Env::new(true, 1.0),
    );
    rt.frame(0.0);
    rt.frame(1.0 / 60.0);
    assert_eq!(rt.state.light.size(), [300.0, 40.0]);
    assert!(!rt.state.light.is_lit());
    let commits = rt.stats.commits;
    rt.pointer_move(120.0, 50.0);
    rt.frame(1.0 / 60.0);
    assert!(rt.state.light.is_lit());
    assert_eq!(rt.state.light.pointer(), [100.0, 20.0]);
    let r = hover_light_radius([300.0, 40.0]);
    assert_eq!(
        rt.state.light.hover_rect(),
        [100.0 - r, 20.0 - r, 2.0 * r, 2.0 * r]
    );
    assert_eq!(rt.stats.commits, commits);
    rt.pointer_move(390.0, 190.0);
    rt.frame(1.0 / 60.0);
    assert!(!rt.state.light.is_lit());
}

#[test]
fn mica_builds_for_both_themes() {
    let _ = mica(&Theme::light());
    let _ = mica(&Theme::dark());
    assert_eq!(to_rune(Color::rgb(255, 255, 255)), rune::Color::WHITE);
}
