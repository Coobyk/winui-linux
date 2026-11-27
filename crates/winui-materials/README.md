# winui-materials

WinUI 3 materials for Rune, colours read from `winui_theme::Theme` so they follow light/dark
and the accent. Contract: [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md).

| API | WinUI source |
| --- | --- |
| `mica(theme)`, `mica_alt(theme)`, `MicaBrush` | `MicaController` defaults (Base: `SolidBackgroundFillColorBase`, luminosity 1.0) |
| `acrylic(theme, tint, tint_opacity, luminosity_opacity)`, `AcrylicBrush` presets | `AcrylicBrush.cpp` recipe (blur 30, luminosity → tint → 2% noise) and `AcrylicBrush_themeresources.xaml` |
| `reveal(theme, &Reveal, w, h, radius)`, `Reveal::track` | `RevealBorderLight` / `RevealHoverLight` spotlight geometry |
| `shadow(depth)`, `El::theme_shadow(theme, depth)` | `DropShadowRecipe.h` (ThemeShadow) |
| `noise_tile()`, `noise_pixels()`, `noise_layer()` | 64×64 deterministic tile with the histogram of `NoiseAsset_256X256_PNG.png` |

Rune has no Luminosity/Color blend modes, so the `El` form of Acrylic draws those steps
source-over; `AcrylicBrush::composite_over` computes the exact blended colour.

```sh
cargo run -p winui-materials --example demo -- --scene mica|acrylic|reveal|shadow --snap out.png
# add -light to a scene name for the light theme, e.g. --scene acrylic-light
```
