use winui_theme::*;

fn hex(c: Color) -> String {
    c.to_string()
}

#[test]
fn dark_values_match_winui() {
    let t = Theme::dark();
    let k = t.tokens();
    assert_eq!(hex(k.text_fill_color_primary), "#FFFFFFFF");
    assert_eq!(hex(k.text_fill_color_secondary), "#C5FFFFFF");
    assert_eq!(hex(k.text_fill_color_tertiary), "#87FFFFFF");
    assert_eq!(hex(k.text_fill_color_disabled), "#5DFFFFFF");
    assert_eq!(hex(k.text_on_accent_fill_color_primary), "#FF000000");
    assert_eq!(hex(k.control_fill_color_default), "#0FFFFFFF");
    assert_eq!(hex(k.control_fill_color_secondary), "#15FFFFFF");
    assert_eq!(hex(k.control_stroke_color_default), "#12FFFFFF");
    assert_eq!(hex(k.card_background_fill_color_default), "#0DFFFFFF");
    assert_eq!(hex(k.layer_fill_color_default), "#4C3A3A3A");
    assert_eq!(hex(k.solid_background_fill_color_base), "#FF202020");
    assert_eq!(hex(k.system_fill_color_success), "#FF6CCB5F");
    assert_eq!(hex(k.system_fill_color_caution), "#FFFCE100");
    assert_eq!(hex(k.system_fill_color_critical), "#FFFF99A4");
    assert_eq!(hex(k.accent_fill_color_default), "#FF60CDFF");
    assert_eq!(hex(k.accent_text_fill_color_primary), "#FF99EBFF");
    assert_eq!(hex(k.system_fill_color_attention), "#FF60CDFF");
    assert_eq!(
        k.accent_fill_color_secondary.to_rgba8()[3],
        (0.9f32 * 255.0).round() as u8
    );
}

#[test]
fn light_values_match_winui() {
    let t = Theme::light();
    let k = t.tokens();
    assert_eq!(hex(k.text_fill_color_primary), "#E4000000");
    assert_eq!(hex(k.text_fill_color_secondary), "#9E000000");
    assert_eq!(hex(k.text_fill_color_tertiary), "#72000000");
    assert_eq!(hex(k.text_fill_color_disabled), "#5C000000");
    assert_eq!(hex(k.text_on_accent_fill_color_primary), "#FFFFFFFF");
    assert_eq!(hex(k.control_fill_color_default), "#B3FFFFFF");
    assert_eq!(hex(k.control_stroke_color_default), "#0F000000");
    assert_eq!(hex(k.control_stroke_color_secondary), "#29000000");
    assert_eq!(hex(k.card_background_fill_color_default), "#B3FFFFFF");
    assert_eq!(hex(k.solid_background_fill_color_base), "#FFF3F3F3");
    assert_eq!(hex(k.system_fill_color_success), "#FF0F7B0F");
    assert_eq!(hex(k.system_fill_color_critical), "#FFC42B1C");
    assert_eq!(hex(k.accent_fill_color_default), "#FF0067C0");
    assert_eq!(hex(k.accent_text_fill_color_primary), "#FF003E92");
    assert_eq!(hex(k.system_fill_color_attention), "#FF0078D4");
}

#[test]
fn elevation_borders() {
    let l = Theme::light();
    let g = l.tokens().control_elevation_border;
    assert!(g.flip_y);
    // Light: darker stroke at the bottom edge, lighter above.
    assert_eq!(hex(g.bottom(32.0)), "#29000000");
    assert_eq!(hex(g.top(32.0)), "#0F000000");
    let d = Theme::dark();
    let g = d.tokens().control_elevation_border;
    assert!(!g.flip_y);
    assert_eq!(hex(g.top(32.0)), "#18FFFFFF");
    assert_eq!(hex(g.bottom(32.0)), "#12FFFFFF");
}

#[test]
fn named_lookup_covers_contract() {
    let t = Theme::light();
    let named = t.tokens().named();
    for key in [
        "TextFillColorPrimary",
        "TextFillColorSecondary",
        "TextFillColorTertiary",
        "TextFillColorDisabled",
        "AccentTextFillColorDisabled",
        "TextOnAccentFillColorPrimary",
        "TextOnAccentFillColorSecondary",
        "TextOnAccentFillColorDisabled",
        "AccentFillColorDefault",
        "AccentFillColorSecondary",
        "AccentFillColorTertiary",
        "AccentFillColorDisabled",
        "ControlFillColorDefault",
        "ControlFillColorSecondary",
        "ControlFillColorTertiary",
        "ControlFillColorDisabled",
        "ControlAltFillColorSecondary",
        "ControlAltFillColorTertiary",
        "ControlAltFillColorQuarternary",
        "ControlStrokeColorDefault",
        "ControlStrokeColorSecondary",
        "CardBackgroundFillColorDefault",
        "CardBackgroundFillColorSecondary",
        "LayerFillColorDefault",
        "LayerFillColorAlt",
        "SolidBackgroundFillColorBase",
        "SubtleFillColorSecondary",
        "SubtleFillColorTertiary",
        "SystemFillColorAttention",
        "SystemFillColorSuccess",
        "SystemFillColorCaution",
        "SystemFillColorCritical",
    ] {
        assert!(named.iter().any(|(n, _)| *n == key), "missing {key}");
    }
    let mut names: Vec<_> = named.iter().map(|(n, _)| *n).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), named.len());
}

#[test]
fn themes_differ_and_switch() {
    let l = Theme::light();
    let d = l.toggled();
    assert_eq!(d.mode(), ThemeMode::Dark);
    assert_eq!(d, Theme::dark());
    assert_ne!(l.tokens(), d.tokens());
    assert_eq!(d.toggled(), l);
}

#[test]
fn default_accent_roundtrips() {
    let t = Theme::dark().with_accent(Color::rgb(0x00, 0x78, 0xD4));
    assert_eq!(t, Theme::dark());
    assert_eq!(*t.accent(), AccentRamp::DEFAULT);
}

#[test]
fn derived_ramp_close_to_default_for_near_default() {
    // Off-by-one accent should land within a few units of the real ramp.
    let r = AccentRamp::derive(Color::rgb(0x00, 0x78, 0xD5));
    for (a, b) in r.steps().iter().zip(AccentRamp::DEFAULT.steps()) {
        let (a, b) = (a.to_rgba8(), b.to_rgba8());
        for i in 0..3 {
            assert!((a[i] as i32 - b[i] as i32).abs() <= 3, "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn derived_ramp_is_ordered_and_in_gamut() {
    for hex in [
        "#881798", "#E74856", "#10893E", "#FFB900", "#767676", "#000000", "#FFFFFF", "#00B7C3",
    ] {
        let accent = Color::from_hex(hex).unwrap();
        let ramp = AccentRamp::derive(accent);
        assert!(ramp.base.eq_rgba8(accent));
        let ls: Vec<f32> = ramp.steps().iter().map(|c| c.to_oklab()[0]).collect();
        for w in ls.windows(2) {
            assert!(w[0] <= w[1] + 1e-3, "{hex}: lightness not monotonic {ls:?}");
        }
        for c in ramp.steps() {
            assert!(c.in_gamut() && c.a == 1.0, "{hex}: {c}");
        }
        let t = Theme::light().with_accent(accent);
        assert_eq!(t.tokens().accent_fill_color_default, ramp.dark1);
        assert_eq!(
            t.tokens().accent_fill_color_tertiary,
            ramp.dark1.with_opacity(0.8)
        );
        let t = Theme::dark().with_accent(accent);
        assert_eq!(t.tokens().accent_fill_color_default, ramp.light2);
    }
}

#[test]
fn derived_accent_buttons_keep_text_contrast() {
    for hex in [
        "#FFB900", "#FFFF00", "#00B7C3", "#000000", "#1A1A1A", "#FFFFFF", "#E74856", "#881798",
    ] {
        let accent = Color::from_hex(hex).unwrap();
        for theme in [Theme::light(), Theme::dark()] {
            let t = theme.with_accent(accent);
            let k = t.tokens();
            let ratio = k
                .accent_fill_color_default
                .contrast_ratio(k.text_on_accent_fill_color_primary);
            assert!(ratio >= 4.5, "{hex} {:?}: {ratio}", theme.mode());
        }
    }
}

#[test]
fn type_ramp() {
    let r = *Theme::light().type_ramp();
    let sizes: Vec<f32> = [
        r.caption,
        r.body,
        r.subtitle,
        r.title,
        r.title_large,
        r.display,
    ]
    .iter()
    .map(|s| s.size)
    .collect();
    assert_eq!(sizes, [12.0, 14.0, 20.0, 28.0, 40.0, 68.0]);
    assert_eq!(r.body.weight, 400);
    assert_eq!(r.body_strong.weight, 600);
    assert_eq!(r.display.line_height, 92.0);
    assert_eq!(r.caption.optical_family(), fonts::SMALL);
    assert_eq!(r.body.optical_family(), fonts::TEXT);
    assert_eq!(r.title.optical_family(), fonts::DISPLAY);
}

#[test]
fn metrics() {
    assert_eq!(radii::CONTROL, 4.0);
    assert_eq!(radii::OVERLAY, 8.0);
    assert_eq!(radii::CIRCLE, 999.0);
    assert_eq!(
        [
            spacing::XXS,
            spacing::XS,
            spacing::S,
            spacing::M,
            spacing::L,
            spacing::XL,
            spacing::XXL
        ],
        [2.0, 4.0, 8.0, 12.0, 16.0, 20.0, 28.0]
    );
    assert_eq!(sizes::CONTROL_HEIGHT, 32.0);
    assert_eq!(motion::FASTER.as_millis(), 83);
    let e = motion::ease(motion::FAST_OUT_SLOW_IN, 0.5);
    assert!(e > 0.8 && e < 1.0, "{e}");
    assert_eq!(motion::ease(motion::FAST_OUT_SLOW_IN, 1.0), 1.0);
}
