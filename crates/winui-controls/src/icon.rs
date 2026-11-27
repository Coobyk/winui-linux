//! `IconElement` / `SymbolIcon` / `FontIcon`: Fluent System Icons from `winui-icons` as Rune icons.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use rune::{El, Icon, icon};
use winui_icons::{FillRule, Symbol, VIEWBOX};
use winui_theme::{Color, Theme};

use crate::style::rc;

/// The Rune icon for `s`, built once and cached for the process.
pub fn icon_of(s: Symbol) -> &'static Icon {
    static CACHE: OnceLock<Mutex<HashMap<Symbol, &'static Icon>>> = OnceLock::new();
    let mut map = CACHE.get_or_init(Default::default).lock().unwrap();
    map.entry(s).or_insert_with(|| {
        let rule = match s.fill_rule() {
            FillRule::EvenOdd => "evenodd",
            FillRule::NonZero => "nonzero",
        };
        let body = format!("<path d=\"{}\" fill-rule=\"{rule}\"/>", s.path());
        Box::leak(Box::new(Icon { name: s.name(), view: VIEWBOX, body: Box::leak(body.into_boxed_str()) }))
    })
}

/// A symbol glyph at `size` points in `color`.
pub fn glyph(s: Symbol, size: f32, color: Color) -> El {
    icon(icon_of(s), size).color(rc(color))
}

/// `SymbolIcon` / `FontIcon`: an icon in the control foreground (`TextFillColorPrimary`),
/// 16px by default as in Gallery's IconElement samples.
pub struct IconElement {
    symbol: Symbol,
    size: f32,
    color: Option<Color>,
}

impl IconElement {
    pub fn new(symbol: Symbol) -> Self {
        Self { symbol, size: 16.0, color: None }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }
    pub fn view(self, theme: &Theme) -> El {
        let c = self.color.unwrap_or(theme.tokens().text_fill_color_primary);
        glyph(self.symbol, self.size, c).label(self.symbol.name()).role("img")
    }
}
