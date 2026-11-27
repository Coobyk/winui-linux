use winui_materials::*;
use winui_theme::{Color, Theme, ThemeMode};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn noise_tile_is_deterministic_and_matches_winui_histogram() {
    let tile = noise_tile();
    assert_eq!(tile.len(), (NOISE_SIZE * NOISE_SIZE) as usize);
    assert_eq!(tile, noise_tile());
    let n = tile.len() as f32;
    for &(level, share) in &NOISE_LEVELS {
        let count = tile.iter().filter(|&&v| v == level).count() as f32;
        assert!((count / n - share).abs() <= 1.0 / n, "level {level}");
    }
    assert!(
        tile.iter()
            .all(|v| NOISE_LEVELS.iter().any(|&(l, _)| l == *v))
    );
    // Neighbouring pixels decorrelated: not a gradient or stripes.
    let same = tile.windows(2).filter(|w| w[0] == w[1]).count() as f32 / n;
    assert!(same < 0.35, "{same}");
    let px = noise_pixels();
    assert_eq!(px.size(), (NOISE_SIZE, NOISE_SIZE));
    assert_eq!(NOISE_OPACITY, 0.02);
}

#[test]
fn tint_opacity_modifier_matches_acrylicbrush_cpp() {
    assert!(close(
        tint_opacity_modifier(Color::rgb(255, 255, 255)),
        0.45
    ));
    assert!(close(tint_opacity_modifier(Color::rgb(0, 0, 0)), 0.85));
    assert!(close(tint_opacity_modifier(Color::rgb(255, 0, 0)), 0.90));
}

#[test]
fn acrylic_effective_colours() {
    let dark = Theme::dark();
    let base = AcrylicBrush::background_base(&dark);
    assert_eq!(base.tint_color, Color::rgb(0x20, 0x20, 0x20));
    assert_eq!(base.fallback_color, Color::rgb(0x1C, 0x1C, 0x1C));
    assert!(close(base.effective_tint_color().a, 128.0 / 255.0));
    assert!(close(base.effective_luminosity_color().a, 0.96));

    // No luminosity opacity: derived from tint with the modifier and HSV clamps.
    let b = AcrylicBrush::new(
        Color::rgb(255, 255, 255),
        0.8,
        None,
        Color::rgb(255, 255, 255),
    );
    assert!(close(
        b.effective_tint_color().a,
        (255.0f32 * 0.8 * 0.45).round() / 255.0
    ));
    let l = b.effective_luminosity_color();
    assert!(close(l.r, 0.965) && close(l.a, 204.0 / 255.0 * 0.88 + 0.15));

    let light = AcrylicBrush::background_default(&Theme::light());
    assert_eq!(light.tint_color, Color::rgb(0xFC, 0xFC, 0xFC));
    assert_eq!(light.tint_luminosity_opacity, Some(0.85));
    assert!(close(light.effective_tint_color().a, 0.0));
}

#[test]
fn acrylic_composite_is_opaque_and_tracks_theme() {
    let wall = Color::rgb(0x00, 0x5F, 0xB8);
    let d = AcrylicBrush::background_base(&Theme::dark()).composite_over(wall);
    let l = AcrylicBrush::background_base(&Theme::light()).composite_over(wall);
    assert!(close(d.a, 1.0) && close(l.a, 1.0));
    assert!(lum(d) < 0.2 && lum(l) > 0.8, "{d:?} {l:?}");
}

#[test]
fn accent_acrylic_tracks_accent() {
    let t = Theme::light();
    let red = t.with_accent(Color::rgb(0xE8, 0x11, 0x23));
    assert_eq!(
        AcrylicBrush::accent_default(&t).tint_color,
        t.accent().light3
    );
    assert_eq!(
        AcrylicBrush::accent_default(&red).tint_color,
        red.accent().light3
    );
    assert_ne!(
        AcrylicBrush::accent_default(&t).tint_color,
        AcrylicBrush::accent_default(&red).tint_color
    );
    assert_eq!(
        AcrylicBrush::accent_base(&Theme::dark()).tint_color,
        Theme::dark().accent().dark2
    );
}

#[test]
fn mica_uses_solid_background_base() {
    for t in [Theme::light(), Theme::dark()] {
        let m = mica_brush(&t);
        assert_eq!(m.tint_color, t.tokens().solid_background_fill_color_base);
        assert_eq!(m.luminosity_opacity, 1.0);
        assert!(close(m.composite_over(Color::rgb(0xF2, 0x8C, 0x28)).a, 1.0));
    }
    assert_eq!(mica_brush(&Theme::dark()).tint_opacity, 0.8);
    assert_eq!(mica_brush(&Theme::light()).tint_opacity, 0.5);
    assert_eq!(
        MicaBrush::new(&Theme::dark(), MicaKind::BaseAlt).tint_color,
        Color::rgb(0x0A, 0x0A, 0x0A)
    );
    let _ = (mica(&Theme::dark()), mica_alt(&Theme::light()));
}

#[test]
fn theme_shadow_recipes_match_dropshadowrecipe_h() {
    let r = |d, m| shadow(d).recipe(m);
    // Z 2 → elevation 1: no shadow.
    assert!(r(2.0, ThemeMode::Light).shadows().is_empty());
    // Z 16 → elevation 8: directional only.
    let s = r(16.0, ThemeMode::Light);
    assert!(
        close(s.elevation, 8.0)
            && close(s.directional_opacity, 0.14)
            && close(s.directional_blur_radius, 8.0)
    );
    assert!(close(s.directional_y_offset, 4.0) && s.ambient_opacity == 0.0);
    assert!(close(r(8.0, ThemeMode::Light).directional_opacity, 0.10));
    assert!(close(r(32.0, ThemeMode::Dark).directional_opacity, 0.26));
    assert_eq!(r(32.0, ThemeMode::Dark).shadows().len(), 1);
    // Z 64 → elevation 32: ambient + directional.
    let s = r(64.0, ThemeMode::Light);
    assert!(close(s.ambient_blur_radius, 32.0 / 3.0) && close(s.ambient_y_offset, 2.0));
    assert!(close(s.ambient_opacity, 0.15) && close(s.directional_opacity, 0.19));
    let s = r(128.0, ThemeMode::Dark);
    assert!(
        close(s.elevation, 64.0)
            && close(s.ambient_opacity, 0.37)
            && close(s.directional_opacity, 0.37)
    );
    assert_eq!(s.shadows().len(), 2);
    assert!(close(r(1000.0, ThemeMode::Dark).elevation, 64.0));
    assert_eq!(
        THEME_SHADOW_DEPTHS,
        [2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0]
    );
}

#[test]
fn reveal_light_footprints() {
    assert!((RevealLight::BORDER.radius() - 39.0).abs() < 0.1);
    assert!((RevealLight::BORDER_WIDE.radius() - 77.0).abs() < 0.1);
    assert!(close(RevealLight::hover(100.0, 32.0).radius(), 112.0));
    assert!(close(RevealLight::hover(1.0, 1.0).radius(), 16.0));
    assert!(close(RevealLight::hover(900.0, 10.0).radius(), 512.0));
    assert!((RevealLight::PRESS.radius() - 47.25).abs() < 0.01);
    let dark = Theme::dark();
    assert_eq!(Reveal::border_color(&dark), Color::rgba(255, 255, 255, 0.2));
    assert_eq!(Reveal::accent_border_color(&dark), dark.accent().base);
    let _ = reveal(
        &dark,
        &Reveal {
            pointer: Some([10.0, 10.0]),
            pressed: false,
        },
        100.0,
        40.0,
        4.0,
    );
    let _ = reveal(&dark, &Reveal::default(), 100.0, 40.0, 4.0);
}

#[test]
fn colour_conversion_decodes_srgb() {
    let c = rc(Color::rgb(0x20, 0x20, 0x20));
    assert!((c.r - 0.014_444).abs() < 1e-4);
    assert_eq!(rc(Color::rgba(255, 255, 255, 0.5)).a, 0.5);
}

#[test]
fn blend_modes() {
    let grey = Color::rgb(128, 128, 128);
    let red = Color::rgb(255, 0, 0);
    let out = blend_luminosity(red, grey);
    assert!(close(lum(out), lum(grey)));
    assert!(out.r > out.g);
    let out = blend_color(grey, red);
    assert!(close(lum(out), lum(grey)) && out.r > out.g);
}
