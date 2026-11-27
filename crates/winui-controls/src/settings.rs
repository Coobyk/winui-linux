//! `SettingsCard` — the Windows Settings row (Windows Community Toolkit `SettingsCard`): a
//! full-width card with an icon, a header over a secondary description, optional trailing
//! content and, when clickable, a chevron.

use crate::shim::icons::Symbol;
use crate::shim::theme::Theme;
use crate::style::{self, focus_rect, glyph, rc, Hook, R};
use rune::{col, row, El};

/// `SettingsCardMinHeight`, `SettingsCardPadding`, `SettingsCardHeaderIconMaxSize`,
/// `SettingsCardHeaderIconMargin` (right), `SettingsCardActionIconMaxSize`,
/// `SettingsCardActionIconMargin` (left).
const MIN_HEIGHT: f32 = 68.0;
const PADDING: f32 = 16.0;
const ICON: f32 = 20.0;
const ICON_GAP: f32 = 20.0;
const CHEVRON: f32 = 13.0;
const CHEVRON_GAP: f32 = 14.0;

#[derive(Default)]
pub struct SettingsCard {
    header: String,
    description: Option<String>,
    icon: Option<Symbol>,
    content: Option<El>,
    clickable: bool,
    disabled: bool,
    key: Option<String>,
    click: Hook<()>,
}

impl SettingsCard {
    pub fn new(header: impl Into<String>) -> Self {
        SettingsCard { header: header.into(), ..Default::default() }
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    /// Trailing content (a ToggleSwitch, ComboBox, value text), right-aligned.
    pub fn content(mut self, e: El) -> Self {
        self.content = Some(e);
        self
    }
    /// Makes the whole card a button with a trailing chevron (`IsClickEnabled`).
    pub fn clickable(mut self, on: bool) -> Self {
        self.clickable = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.disabled = !on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_click<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.clickable = true;
        self.click.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = !self.disabled;
        let (fg, fg2) = if en { (k.text_fill_color_primary, k.text_fill_color_secondary) } else { (k.text_fill_color_disabled, k.text_fill_color_disabled) };
        let mut text = col().grow(1.0).child(rune::text(self.header.clone()).font(style::body(t)).color(rc(fg)));
        if let Some(d) = &self.description {
            text = text.child(rune::text(d.clone()).font(style::caption(t)).color(rc(fg2)));
        }
        let mut e = row().min_h(MIN_HEIGHT).padding(PADDING).center_y().grow(1.0);
        // `SettingsCardBackground` (CardBackgroundFillColorDefault) swapped for
        // `SettingsCardBackgroundPointerOver` (ControlFillColorSecondary), both layers inside the
        // CardStrokeColorDefault ring so the stroke composites over the page.
        let i = Some(style::INSET);
        let layer = |c| rune::el().pin(i, i, i, i).corners(style::inner([R; 4])).fill(rc(c));
        let hover = self.clickable && en;
        let rest = layer(k.card_background_fill_color_default);
        e = e.radius(R).child(if hover { rest.group_opacity(1.0, 0.0) } else { rest });
        if hover {
            e = e.hover_group().child(layer(k.control_fill_color_secondary).group_opacity(0.0, 1.0));
        }
        e = e.child(style::ring([R; 4], 0.0, k.card_stroke_color_default, k.card_stroke_color_default, false, 1.0));
        if let Some(s) = self.icon {
            e = e.child(rune::el().size(ICON, ICON).mr(ICON_GAP).ml(2.0).center().child(glyph(s, ICON, fg)));
        }
        e = e.child(text);
        if let Some(c) = &self.content {
            e = e.child(row().ml(16.0).center_y().child(c.clone()));
        }
        if self.clickable {
            e = e.child(rune::el().ml(CHEVRON_GAP).child(glyph(Symbol::ChevronRight, CHEVRON, fg)));
            e = e.role("button").label(self.header.clone()).disabled(!en);
            if en {
                e = focus_rect(e.press_color(rc(k.text_fill_color_secondary)), t, R);
                e = self.click.attach(e, ());
            }
        }
        match &self.key {
            Some(key) => e.key(key.clone()),
            None => e,
        }
    }
}
