# winui-materials

Fluent materials as composable Rune elements: Mica, Acrylic, Reveal and elevation.
See [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) for the contract.

| Contract | API | Notes |
| --- | --- | --- |
| `mica(theme)` | `mica`, `mica_alt`, `mica::mica_color` | `SolidBackgroundFillColorBase`/`BaseAlt` + 2% noise. |
| `acrylic(theme, tint, tint_opacity, luminosity_opacity)` | `acrylic`, `Acrylic` (`in_app_default`, `in_app_base`, `accent_default`, `.fallback(true)`) | 30 pt backdrop blur, WinUI's tint/luminosity opacity maths, 2% noise. |
| `reveal(...)` | `reveal(theme, &RevealLight)`, `reveal_with(&RevealStyle, ..)` | Hover pool + border spotlight sized like `RevealHoverLight`/`RevealBorderLight`; pointer moves only touch `Live` values. |
| `shadow(depth)` | `shadow`, `shadow_for(depth, mode)`, `Elevation::apply`, `ElevationExt::elevation` | Ambient + key shadow for depths 2/4/8/16/32/64/128. |
| noise helper | `noise_tile(seed)`, `noise_pixels()`, `noise_layer(opacity)` | Deterministic 64×64 `u8` tile. |

Every returned value is a plain `rune::El`: size it and add children as usual.

```rust
let theme = Theme::dark();
let pane = Acrylic::in_app_default(&theme).el().w(320.0).h_full().radius(8.0).child(items);
let row = reveal(&theme, &state.rows[i]).w_full().h(40.0).child(text("Home"));
let card = el().radius(8.0).fill(white).elevation(8.0, theme.mode());
```

## Demo

```sh
cargo run -p winui-materials --example demo -- --scene mica|acrylic|reveal|shadow --snap out.png [--theme light|dark]
```

## Gaps

- **winui-theme API not yet available.** `winui-theme` is still a stub, so `src/theme.rs` is a
  local shim of the contract (`ThemeMode`, `Color`, `Tokens`, `Theme`) carrying the token fields
  materials read, with real WinUI values. When winui-theme lands, replace that module's body with
  `pub use winui_theme::{Color, Theme, ThemeMode, Tokens};`. Assumptions to confirm: `Color`
  components are sRGB in `0.0..=1.0`; `Tokens` has `solid_background_fill_color_base`,
  `solid_background_fill_color_base_alt`, `accent_fill_color_default`, `text_fill_color_*`,
  `control_stroke_color_default`, `card_*` fields. The shim's accent ramp for non-default accents
  is approximate.
- Rune has no luminosity/colour blend modes, so Acrylic's luminosity and tint layers composite
  source-over onto the blurred backdrop. Colourful backdrops keep slightly less hue than on Windows.
- Accent Acrylic tints with `AccentFillColorDefault` rather than WinUI's `SystemAccentColorDark1`/`Light3`.
- Reveal's light switches on/off instantly; WinUI fades the hover light in over 166 ms.
