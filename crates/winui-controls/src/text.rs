//! Text controls: TextBox, PasswordBox, NumberBox, AutoSuggestBox-lite.
//! Matches "Text" Gallery samples.

use rune::{El, TextEdit, col, el, row, text, text_input};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, sizes};

use crate::{body, glyph};

/// The WinUI text-control chrome: CardRadius box, `ControlFillColorInputActive`
/// fill, hairline border, and the stronger bottom stroke that turns accent
/// while focused. `focused` paints that 2px accent underline for snapshots.
fn text_chrome(theme: &Theme, enabled: bool, focused: bool, inner: El) -> El {
    let t = theme.tokens();
    let fill = if enabled {
        rc(t.control_fill_color_input_active)
    } else {
        rc(t.control_fill_color_disabled)
    };
    let mut e = el()
        .min_h(sizes::CONTROL_HEIGHT)
        .min_w(sizes::TEXT_CONTROL_MIN_WIDTH)
        .radius(4.0)
        .fill(fill)
        .border(1.0, rc(t.control_stroke_color_secondary))
        .clip_rounded(4.0)
        .child(inner);
    if focused && enabled {
        e = e.child(
            el().absolute(0.0, sizes::CONTROL_HEIGHT - 2.0)
                .w_full()
                .h(2.0)
                .fill(rc(t.accent_fill_color_default)),
        );
    } else {
        e = e.child(
            el().absolute(0.0, sizes::CONTROL_HEIGHT - 1.0)
                .w_full()
                .h(1.0)
                .fill(rc(t.control_strong_stroke_color_default)),
        );
    }
    e
}

/// A single-line TextBox.
#[derive(Clone, Debug)]
pub struct TextBox {
    pub placeholder: String,
    pub enabled: bool,
    pub focused: bool,
    pub header: Option<String>,
}

impl TextBox {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            enabled: true,
            focused: false,
            header: None,
        }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn focused(mut self, on: bool) -> Self {
        self.focused = on;
        self
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }

    /// `edit` lives in your state — `TextEdit::new("")` for an empty box.
    pub fn view(&self, theme: &Theme, edit: &TextEdit) -> El {
        let t = theme.tokens();
        let text_c = rc(if self.enabled {
            t.text_fill_color_primary
        } else {
            t.text_fill_color_disabled
        });
        let inner = el().w_full()
            .min_h(sizes::CONTROL_HEIGHT)
            .center_y()
            .pl(sizes::TEXT_CONTROL_PADDING.left)
            .pr(sizes::TEXT_CONTROL_PADDING.right)
            .child(
                text_input(edit, body())
                    .w_full()
                    .color(text_c)
                    .placeholder(self.placeholder.clone(), rc(t.text_fill_color_tertiary))
                    .label(self.header.clone().unwrap_or_else(|| self.placeholder.clone())),
            );
        let boxed = text_chrome(theme, self.enabled, self.focused, inner);
        match &self.header {
            Some(h) => col().gap(4.0)
                .child(text(h.clone()).font(body()).color(rc(t.text_fill_color_primary)))
                .child(boxed),
            None => boxed,
        }
    }
}

/// PasswordBox: bullets + a reveal (eye) button on the right.
#[derive(Clone, Debug)]
pub struct PasswordBox {
    pub placeholder: String,
    pub enabled: bool,
    pub revealed: bool,
}

impl PasswordBox {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self { placeholder: placeholder.into(), enabled: true, revealed: false }
    }
    pub fn revealed(mut self, on: bool) -> Self {
        self.revealed = on;
        self
    }
    pub fn view(&self, theme: &Theme, edit: &TextEdit) -> El {
        let t = theme.tokens();
        let inner = row().items_center().w_full().min_h(sizes::CONTROL_HEIGHT)
            .pl(sizes::TEXT_CONTROL_PADDING.left)
            .child(
                text_input(edit, body())
                    .secure()
                    .grow(1.0)
                    .min_w(0.0)
                    .color(rc(t.text_fill_color_primary))
                    .placeholder(self.placeholder.clone(), rc(t.text_fill_color_tertiary))
                    .label("Password"),
            )
            .child(
                el().size(28.0, 24.0)
                    .mr(4.0)
                    .radius(4.0)
                    .center()
                    .role("button")
                    .label("Reveal password")
                    .hover_fill(rc(t.subtle_fill_color_secondary))
                    .child(glyph(if self.revealed { Symbol::EyeOff } else { Symbol::Eye }, 12.0, rc(t.text_fill_color_secondary))),
            );
        text_chrome(theme, self.enabled, false, inner)
    }
}

/// NumberBox: a TextBox with up/down spin buttons docked on the right.
#[derive(Clone, Debug)]
pub struct NumberBox {
    pub enabled: bool,
    pub placeholder: String,
}

impl NumberBox {
    pub fn new() -> Self {
        Self { enabled: true, placeholder: String::new() }
    }
    pub fn view(&self, theme: &Theme, edit: &TextEdit) -> El {
        let t = theme.tokens();
        let spin = |sym: Symbol, top: bool| {
            el().size(24.0, 15.0)
                .center()
                .role("button")
                .hover_fill(rc(t.subtle_fill_color_secondary))
                .corners(if top { [0.0, 3.0, 0.0, 0.0] } else { [0.0, 0.0, 3.0, 0.0] })
                .child(glyph(sym, 8.0, rc(t.text_fill_color_secondary)))
        };
        let inner = row().items_center().w_full().min_h(sizes::CONTROL_HEIGHT)
            .pl(sizes::TEXT_CONTROL_PADDING.left)
            .child(
                text_input(edit, body())
                    .grow(1.0)
                    .min_w(0.0)
                    .color(rc(t.text_fill_color_primary))
                    .placeholder(self.placeholder.clone(), rc(t.text_fill_color_tertiary))
                    .label("Number"),
            )
            .child(
                col().mr(4.0).child(spin(Symbol::ChevronUp, true)).child(spin(Symbol::ChevronDown, false)),
            );
        text_chrome(theme, self.enabled, false, inner)
    }
}

impl Default for NumberBox {
    fn default() -> Self {
        Self::new()
    }
}

/// AutoSuggestBox-lite: a TextBox with a leading icon and, when `open`, the
/// suggestion flyout beneath it (two rows shown, first highlighted — the
/// static look of the Gallery's open dropdown).
#[derive(Clone, Debug)]
pub struct AutoSuggestBox {
    pub placeholder: String,
    pub enabled: bool,
    /// Suggestion rows; non-empty renders the open popup.
    pub suggestions: Vec<String>,
    pub selected: usize,
}

impl AutoSuggestBox {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            enabled: true,
            suggestions: Vec::new(),
            selected: 0,
        }
    }
    pub fn suggestions(mut self, s: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.suggestions = s.into_iter().map(Into::into).collect();
        self
    }
    pub fn view(&self, theme: &Theme, edit: &TextEdit) -> El {
        let t = theme.tokens();
        let inner = row().items_center().w_full().min_h(sizes::CONTROL_HEIGHT)
            .pl(sizes::TEXT_CONTROL_PADDING.left)
            .child(
                text_input(edit, body())
                    .grow(1.0)
                    .min_w(0.0)
                    .color(rc(t.text_fill_color_primary))
                    .placeholder(self.placeholder.clone(), rc(t.text_fill_color_tertiary))
                    .label("Search"),
            )
            .child(
                el().size(28.0, 24.0)
                    .mr(4.0)
                    .center()
                    .child(glyph(Symbol::Find, 12.0, rc(t.text_fill_color_secondary))),
            );
        let boxed = text_chrome(theme, self.enabled, false, inner);
        if self.suggestions.is_empty() {
            return boxed;
        }
        let mut list = col().py(4.0);
        for (i, s) in self.suggestions.iter().enumerate() {
            let mut row_e = el().w_full()
                .h(28.0)
                .center_y()
                .px(12.0)
                .radius(4.0)
                .mx(4.0)
                .child(text(s.clone()).font(body()).color(rc(t.text_fill_color_primary)).nowrap());
            if i == self.selected {
                row_e = row_e.fill(rc(t.subtle_fill_color_secondary));
            } else {
                row_e = row_e.hover_fill(rc(t.subtle_fill_color_secondary));
            }
            list = list.child(row_e);
        }
        let popup = el().w_full()
            .mt(4.0)
            .radius(8.0)
            .fill(rc(t.layer_on_acrylic_fill_color_default))
            .border(1.0, rc(t.surface_stroke_color_flyout))
            .box_shadow(winui_materials::elevation(theme, 16.0))
            .child(list);
        col().w_full().child(boxed).child(popup)
    }
}
