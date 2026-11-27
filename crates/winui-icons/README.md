# winui-icons

Fluent System Icons for winui-linux: `Symbol` (211 variants), `Symbol::ALL`,
`Symbol::glyph() -> Glyph::Path(&'static str)`. See
[../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) for the contract.

- Paths are the **24px** set (regular, plus `StarFilled` / `CircleFilled` from
  the filled set) in a 24x24 viewBox (`winui_icons::VIEWBOX`).
- Multi-path assets are merged into one path. `tools/gen.py` renders each merged
  path under both fill rules and checks it against the original SVG; every
  current symbol renders identically with **evenodd**, and
  `Symbol::fill_rule()` reports the rule to use if that ever changes.
- `Symbol::segoe_codepoint()` / `font_glyph()` give the matching Segoe Fluent
  Icons codepoint (`Glyph::Font`) for 159 symbols, from Microsoft's published
  [Segoe Fluent Icons list](https://learn.microsoft.com/windows/apps/design/style/segoe-fluent-icons-font)
  (`tools/segoe-fluent-icons.json`). `glyph()` always returns the vendored path
  since the font is not available on Linux.
- Aliases mirroring WinUI's `Symbol` enum share a path: `Accept`/`CheckMark`,
  `Cancel`/`Dismiss`.

## Regenerating

`src/generated.rs` is committed; builds never touch the network. To change the
set, edit `tools/symbols.txt` (`Variant | Fluent asset folder | style | Segoe name`)
and run:

```
pip install cairosvg
python3 tools/gen.py
```

## Licence

Icon data © Microsoft Corporation, from
[microsoft/fluentui-system-icons](https://github.com/microsoft/fluentui-system-icons)
at rev `a3de63245b4fa49c30e20d9838593a105affe774`, MIT licensed — see
[LICENSE-fluentui-system-icons](LICENSE-fluentui-system-icons).
