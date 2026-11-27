# winui-linux

**WinUI 3, for Linux.** Microsoft's Fluent Design UI toolkit — the look and
feel of Windows 11's built-in apps — reimplemented in Rust on
[Rune](https://github.com/anomalyco/rune) (`anomalyco/rune`, pre-release —
pin a rev of the `dev` branch).

Feature targets, à la WinUI 3:

- The Fluent control set: buttons, toggles, sliders, text boxes, combo boxes,
  NavigationView, TabView, InfoBar, ProgressBar/Ring, flyouts, teaching tips
- Fluent materials: Mica and Acrylic backdrops, reveal highlight, elevation
- The Fluent type ramp, colour tokens, and Fluent System Icons
- Light/dark theming and accent colours, switchable at runtime
- A WinUI 3 Gallery–style showcase app exercising every control

## Workspace layout

| Crate             | Responsibility |
| ----------------- | -------------- |
| `winui-theme`     | Fluent design tokens: colours, type ramp, spacing, radii, elevation |
| `winui-icons`     | Fluent System Icons: glyph data and `Symbol` enum |
| `winui-materials` | Mica, Acrylic, reveal, shadow effects in Rune |
| `winui-controls`  | The widget set as Rune components |
| `winui-gallery`   | Showcase app: nav shell + a page per control family |

Each crate compiles independently against the contracts in
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Run

```
cargo run -p winui-gallery
```

Headless snapshot (no display needed):

```
cargo run -p winui-gallery -- --scene buttons --snap buttons.png
```
