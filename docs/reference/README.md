# Reference screenshots

Real Windows 11 inbox apps (dark theme) — the visual target for winui-linux
components and demo scenes. Rebuild each app as closely as possible.

- `calculator.png` — Calculator, Standard mode
- `settings-system.png` — Settings > System (NavigationView, card list)
- `store-updates.png` — Microsoft Store, Updates & downloads (left rail nav, list rows)
- `clock-timer.png` — Clock, Timer page (card grid, circular progress)
- `clock-settings.png` — Clock, Settings page (Expander rows, ToggleSwitch, ComboBox)
- `snipping-tool.png` — Snipping Tool (toolbar, accent "New" button, icon buttons + dropdowns)
- `terminal.png` — Windows Terminal (tab strip over acrylic, caption buttons)

## Our renders

`ours/` holds the matching `winui-gallery` scenes, rendered at 1x and the reference size:

```
cargo run -p winui-gallery -- --scene ref-settings --theme dark --scale 1 --snap docs/reference/ours/ref-settings.png
```

Scenes: `ref-settings`, `ref-store`, `ref-clock-timer`, `ref-clock-settings`, `ref-calculator`, `ref-terminal`, `ref-snippets`.
