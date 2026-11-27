//! Text input: `TextBox`, `PasswordBox`, `NumberBox`, `AutoSuggestBox`-lite (WinUI Gallery ›
//! Text). Editing is Rune's `TextEdit`; the app owns it and hands it back through `on_edit`.
use std::rc::Rc;

use rune::{col, row, El, TextEdit};

use crate::impl_control;
use crate::selection::header;
use crate::shim::icons::Symbol;
use crate::shim::theme::{sizes, Theme};
use crate::style::{self, flyout_surface, glyph, rc, Hook, R};

fn edit_fn<S, F: for<'a> Fn(&'a mut S) -> &'a mut TextEdit>(f: F) -> F {
    f
}

/// `TextBox` — Gallery "TextBox": 32px field, 10/6px side padding, elevation border whose
/// bottom edge is `ControlStrongStrokeColorDefault`, and a 2px accent underline while focused.
#[derive(Clone)]
pub struct TextBox {
    edit: TextEdit,
    header: Option<String>,
    placeholder: String,
    enabled: bool,
    focused: bool,
    width: f32,
    secure: bool,
    revealed: bool,
    key: String,
    trailing: Vec<El>,
    edits: Hook<()>,
    focus: Hook<()>,
    submit: Hook<()>,
}

impl TextBox {
    pub fn new(edit: &TextEdit) -> Self {
        TextBox {
            edit: edit.clone(),
            header: None,
            placeholder: String::new(),
            enabled: true,
            focused: false,
            width: 296.0,
            secure: false,
            revealed: false,
            key: "textbox".into(),
            trailing: Vec::new(),
            edits: Hook::default(),
            focus: Hook::default(),
            submit: Hook::default(),
        }
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    /// Whether the field has focus (app-owned; wire `on_focus`). Draws `Focused` visuals.
    pub fn focused(mut self, on: bool) -> Self {
        self.focused = on;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    /// An element inside the field after the text (a button, a glyph).
    pub fn trailing(mut self, e: El) -> Self {
        self.trailing.push(e);
        self
    }
    pub(crate) fn secure(mut self, revealed: bool) -> Self {
        self.secure = true;
        self.revealed = revealed;
        self
    }
    /// Where Rune finds the app's `TextEdit` to apply keystrokes to.
    pub fn on_edit<S: 'static>(mut self, f: impl for<'a> Fn(&'a mut S) -> &'a mut TextEdit + 'static) -> Self {
        let f = Rc::new(f);
        self.edits.raw(move |e, ()| {
            let f = f.clone();
            e.on_edit(edit_fn(move |s: &mut S| f(s)))
        });
        self
    }
    pub fn on_focus<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        let f = Rc::new(f);
        self.focus.raw(move |e, ()| {
            let f = f.clone();
            e.on_focus_within_change(move |s: &mut S, on, _tx: &mut rune::Tx| f(s, on))
        });
        self
    }
    pub fn on_submit<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        let f = Rc::new(f);
        self.submit.raw(move |e, ()| {
            let f = f.clone();
            e.on_submit(move |s: &mut S, _tx: &mut rune::Tx| f(s))
        });
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let mut input = rune::text_input(&self.edit, style::body(t))
            .single_line()
            .grow(1.0)
            .shrink()
            .color(rc(fg))
            .placeholder(self.placeholder.clone(), rc(if en { k.text_fill_color_secondary } else { k.text_fill_color_disabled }))
            .caret(rc(k.text_fill_color_primary), rc(k.accent_fill_color_default.with_alpha(0.45)))
            .label(self.header.clone().unwrap_or_else(|| self.placeholder.clone()))
            .key(format!("{}:input", self.key))
            .disabled(!en);
        if self.secure {
            input = input.secure().revealed(self.revealed);
        }
        if en {
            input = self.submit.attach(self.edits.attach(input, ()), ());
        }
        let mut field = row()
            .w(self.width)
            .h(sizes::CONTROL_HEIGHT)
            .pl(10.0)
            .pr(if self.trailing.is_empty() { 6.0 } else { 2.0 })
            .gap(4.0)
            .center_y()
            .radius(R)
            .clip_rounded(R)
            .key(self.key.clone())
            .child(input)
            .children(self.trailing.iter().cloned());
        field = match (en, self.focused) {
            (false, _) => field.fill(rc(k.control_fill_color_disabled)).border(1.0, rc(k.control_stroke_color_default)),
            (true, true) => field
                .fill(rc(k.control_fill_color_input_active))
                .border(1.0, rc(k.control_stroke_color_default))
                .child(rune::el().absolute(0.0, sizes::CONTROL_HEIGHT - 2.0).size(self.width, 2.0).fill(rc(k.accent_fill_color_default))),
            // TextControlElevationBorderBrush: ControlStrokeColorDefault with a 1px
            // ControlStrongStrokeColorDefault bottom edge (clipped to the 4px corners).
            (true, false) => {
                let f = field
                    .fill(rc(style::by_state(k.control_fill_color_default, k.control_fill_color_secondary, k.control_fill_color_default)))
                    .border(1.0, rc(k.control_stroke_color_default))
                    .child(rune::el().absolute(0.0, sizes::CONTROL_HEIGHT - 1.0).size(self.width, 1.0).fill(rc(k.control_strong_stroke_color_default)));
                if style::pinned().is_none() { f.hover_fill(rc(k.control_fill_color_secondary)) } else { f }
            }
        };
        if en {
            field = self.focus.attach(field, ());
        }
        match &self.header {
            Some(h) => col().child(header(h, t, en)).child(field),
            None => field,
        }
    }
}

/// A subtle 30×24 button inside a field (reveal, clear, spin, search).
fn field_button(t: &Theme, s: Symbol, key: String, label: &str) -> El {
    let k = t.tokens();
    rune::el()
        .size(30.0, 24.0)
        .center()
        .radius(R)
        .fill(rc(k.subtle_fill_color_transparent))
        .hover_fill(rc(k.subtle_fill_color_secondary))
        .press_fill(rc(k.subtle_fill_color_tertiary))
        .press_color(rc(k.text_fill_color_tertiary))
        .child(glyph(s, 12.0, k.text_fill_color_secondary))
        .role("button")
        .label(label.to_owned())
        .key(key)
}

/// `PasswordBox` — Gallery "PasswordBox": a secure `TextBox` with a reveal (eye) button.
#[derive(Clone)]
pub struct PasswordBox {
    field: TextBox,
    revealed: bool,
    reveal: Hook<bool>,
}

impl PasswordBox {
    pub fn new(edit: &TextEdit) -> Self {
        PasswordBox { field: TextBox::new(edit).key("password"), revealed: false, reveal: Hook::default() }
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.field = self.field.header(h);
        self
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.field = self.field.placeholder(p);
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.field = self.field.enabled(on);
        self
    }
    pub fn focused(mut self, on: bool) -> Self {
        self.field = self.field.focused(on);
        self
    }
    pub fn revealed(mut self, on: bool) -> Self {
        self.revealed = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.field = self.field.key(k);
        self
    }
    pub fn on_edit<S: 'static>(mut self, f: impl for<'a> Fn(&'a mut S) -> &'a mut TextEdit + 'static) -> Self {
        self.field = self.field.on_edit(f);
        self
    }
    pub fn on_focus<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.field = self.field.on_focus(f);
        self
    }
    /// `f(state, revealed)` from the eye button.
    pub fn on_reveal<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.reveal.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let mut f = self.field.clone().secure(self.revealed);
        if f.enabled && !f.edit.text.is_empty() {
            let b = field_button(t, Symbol::Eye, format!("{}:reveal", f.key), "Reveal password");
            f = f.trailing(self.reveal.attach(b, !self.revealed));
        }
        f.view(t)
    }
}

/// `NumberBox` — Gallery "NumberBox" with `SpinButtonPlacementMode="Inline"`.
#[derive(Clone)]
pub struct NumberBox {
    field: TextBox,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    spin: Hook<f64>,
}

impl NumberBox {
    pub fn new(value: f64, edit: &TextEdit) -> Self {
        NumberBox { field: TextBox::new(edit).key("numberbox").width(200.0), value, min: f64::MIN, max: f64::MAX, step: 1.0, spin: Hook::default() }
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.field = self.field.header(h);
        self
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn step(mut self, s: f64) -> Self {
        self.step = s;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.field = self.field.enabled(on);
        self
    }
    pub fn focused(mut self, on: bool) -> Self {
        self.field = self.field.focused(on);
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.field = self.field.key(k);
        self
    }
    pub fn on_edit<S: 'static>(mut self, f: impl for<'a> Fn(&'a mut S) -> &'a mut TextEdit + 'static) -> Self {
        self.field = self.field.on_edit(f);
        self
    }
    /// `f(state, new_value)` from the spin buttons, clamped to the range.
    pub fn on_spin<S: 'static>(mut self, f: impl Fn(&mut S, f64) + 'static) -> Self {
        self.spin.press(f);
        self
    }
    /// Parses a `NumberBox` edit's text (`None` for non-numbers), clamped to `[min, max]`.
    pub fn parse(text: &str, min: f64, max: f64) -> Option<f64> {
        text.trim().parse::<f64>().ok().filter(|v| v.is_finite()).map(|v| v.clamp(min, max))
    }
    pub fn view(&self, t: &Theme) -> El {
        let mut f = self.field.clone();
        if f.enabled {
            let up = field_button(t, Symbol::ChevronUp, format!("{}:up", f.key), "Increase");
            let down = field_button(t, Symbol::ChevronDown, format!("{}:down", f.key), "Decrease");
            f = f
                .trailing(self.spin.attach(up, (self.value + self.step).min(self.max)))
                .trailing(self.spin.attach(down, (self.value - self.step).max(self.min)));
        }
        f.view(t).range_value(self.value, self.min, self.max)
    }
}

/// `AutoSuggestBox`-lite — Gallery "AutoSuggestBox": a `TextBox` with a search glyph and a
/// suggestion list below while `open`.
#[derive(Clone)]
pub struct AutoSuggestBox {
    field: TextBox,
    suggestions: Vec<String>,
    open: bool,
    choose: Hook<usize>,
}

impl AutoSuggestBox {
    pub fn new(edit: &TextEdit) -> Self {
        AutoSuggestBox { field: TextBox::new(edit).key("autosuggest"), suggestions: Vec::new(), open: false, choose: Hook::default() }
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.field = self.field.placeholder(p);
        self
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.field = self.field.header(h);
        self
    }
    pub fn focused(mut self, on: bool) -> Self {
        self.field = self.field.focused(on);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.field = self.field.width(w);
        self
    }
    pub fn suggestions<I: Into<String>>(mut self, items: impl IntoIterator<Item = I>) -> Self {
        self.suggestions = items.into_iter().map(Into::into).collect();
        self
    }
    pub fn open(mut self, on: bool) -> Self {
        self.open = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.field = self.field.key(k);
        self
    }
    pub fn on_edit<S: 'static>(mut self, f: impl for<'a> Fn(&'a mut S) -> &'a mut TextEdit + 'static) -> Self {
        self.field = self.field.on_edit(f);
        self
    }
    pub fn on_focus<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.field = self.field.on_focus(f);
        self
    }
    pub fn on_choose<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.choose.press(f);
        self
    }
    /// Case-insensitive substring filter, the Gallery sample's behaviour.
    pub fn filter<'a>(items: &'a [&'a str], query: &str) -> Vec<&'a str> {
        let q = query.to_lowercase();
        items.iter().copied().filter(|s| !q.is_empty() && s.to_lowercase().contains(&q)).collect()
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let f = self.field.clone();
        let key = f.key.clone();
        let width = f.width;
        let search = rune::el().size(30.0, 24.0).center().child(glyph(Symbol::Find, 12.0, k.text_fill_color_secondary));
        let field = f.trailing(search).view(t);
        if !self.open || self.suggestions.is_empty() {
            return field;
        }
        let mut list = flyout_surface(t).w(width).py(2.0);
        for (i, s) in self.suggestions.iter().enumerate() {
            let item = crate::menus::list_row(t, true, false)
                .child(rune::text(s.clone()).font(style::body(t)).color(rc(k.text_fill_color_primary)))
                .key(format!("{key}:suggestion{i}"))
                .role("option")
                .label(s.clone());
            list = list.child(self.choose.attach(item, i));
        }
        col().child(list.anchored(key, rune::Side::Bottom, rune::Align::Start, 2.0).raise()).child(field)
    }
}

impl_control!(TextBox, PasswordBox, NumberBox, AutoSuggestBox);
