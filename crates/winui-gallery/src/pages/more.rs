//! Navigation, Status and Media samples.

use super::{Sample, c};
use crate::fallback::controls::{self as k, ButtonStyle, Severity, body, caption, icon, strong};
use crate::fallback::icons::Symbol;
use crate::fallback::materials;
use crate::fallback::theme::{Theme, radii};
use crate::state::{Page, State};
use rune::{El, col, el, row};

fn tab(t: &Theme, i: usize, title: &str, selected: bool) -> El {
    let tk = t.tokens();
    let close = el().key(format!("tab-{i}-close")).role("button").label("Close tab").size(24.0, 24.0).radius(radii::CONTROL).center()
        .hover_fill(c(tk.subtle_fill_color_secondary)).child(icon(Symbol::Dismiss, 12.0, tk.text_fill_color_secondary))
        .on_press(move |s: &mut State| {
            if s.tabs.len() > 1 {
                s.tabs.remove(i);
                s.tab_sel = s.tab_sel.min(s.tabs.len() - 1);
            }
        });
    let mut e = row().key(format!("tab-{i}")).role("tab").label(title).checked(selected).h(32.0).w(200.0).pl(12.0).pr(4.0).gap(8.0).items_center()
        .corners([radii::OVERLAY, radii::OVERLAY, 0.0, 0.0])
        .child(icon(Symbol::Document, 16.0, tk.text_fill_color_primary)).child(body(title, tk.text_fill_color_primary).grow(1.0)).child(close)
        .on_press(move |s: &mut State| s.tab_sel = i);
    e = if selected { e.fill(c(tk.solid_background_fill_color_tertiary)).border(1.0, c(tk.card_stroke_color_default)) } else { materials::reveal(e, t) };
    e
}

pub static SAMPLES: &[Sample] = &[
    Sample { id: "navigationview", page: Page::Navigation, title: "NavigationView in Left mode", keywords: "NavigationView nav pane hamburger", view: |s, t| {
        let tk = t.tokens();
        let mut pane = col().w(200.0).h_full().py(4.0);
        for (i, (sym, label)) in [(Symbol::Play, "Menu Item 1"), (Symbol::Save, "Menu Item 2"), (Symbol::ArrowSync, "Menu Item 3")].into_iter().enumerate() {
            let content = row().gap(16.0).items_center().child(icon(sym, 16.0, tk.text_fill_color_primary)).child(body(label, tk.text_fill_color_primary));
            pane = pane.child(k::list_row(t, &format!("navsample-{i}"), 36.0, s.nav_sel == i, content, move |s: &mut State| s.nav_sel = i).label(label));
        }
        let content = col().grow(1.0).h_full().padding(24.0).gap(8.0).fill(c(tk.layer_fill_color_default)).border(1.0, c(tk.card_stroke_color_default)).corners([radii::OVERLAY, 0.0, 0.0, 0.0])
            .child(strong(format!("Menu Item {}", s.nav_sel + 1), tk.text_fill_color_primary)).child(body("Page content", tk.text_fill_color_secondary));
        row().key("navsample").w_full().h(200.0).radius(radii::CONTROL).clip_rounded(radii::CONTROL).fill(c(tk.solid_background_fill_color_base)).child(pane).child(content)
    }},
    Sample { id: "tabview", page: Page::Navigation, title: "A TabView with close and add buttons", keywords: "TabView tabs document", view: |s, t| {
        let tk = t.tokens();
        let mut strip = row().key("tabview").role("tablist").w_full().pt(8.0).px(8.0).gap(1.0).items_center().fill(c(tk.solid_background_fill_color_base));
        for (i, title) in s.tabs.iter().enumerate() {
            strip = strip.child(tab(t, i, title, s.tab_sel == i));
        }
        strip = strip.child(k::icon_button(t, "tab-add", Symbol::Add, None, ButtonStyle::Subtle, |s: &mut State| {
            s.tabs.push(format!("Document {}", s.next_tab));
            s.next_tab += 1;
            s.tab_sel = s.tabs.len() - 1;
        }).h(28.0));
        let page = col().w_full().h(80.0).padding(16.0).fill(c(tk.solid_background_fill_color_tertiary)).child(body(s.tabs.get(s.tab_sel).cloned().unwrap_or_default(), tk.text_fill_color_secondary));
        col().w_full().radius(radii::CONTROL).clip_rounded(radii::CONTROL).child(strip).child(page)
    }},
    Sample { id: "breadcrumb", page: Page::Navigation, title: "BreadcrumbBar", keywords: "BreadcrumbBar breadcrumb path", view: |s, t| {
        let tk = t.tokens();
        let parts = ["Home", "Documents", "Design", "Northwind", "Images", "Folder1"];
        let mut bar = row().key("breadcrumb").role("navigation").gap(4.0).items_center();
        for (i, p) in parts.iter().take(s.crumbs + 1).enumerate() {
            let last = i == s.crumbs;
            if i > 0 {
                bar = bar.child(icon(Symbol::ChevronRight, 12.0, tk.text_fill_color_secondary));
            }
            let fg = if last { tk.text_fill_color_primary } else { tk.text_fill_color_secondary };
            bar = bar.child(el().key(format!("crumb-{i}")).role("link").label(*p).px(4.0).radius(radii::CONTROL).child(k::strong(*p, fg).font(crate::fallback::theme::TYPE_RAMP.subtitle.rune())).on_press(move |s: &mut State| s.crumbs = i));
        }
        bar.child(k::button(t, "crumb-reset", "Reset", ButtonStyle::Standard, true, |s: &mut State| s.crumbs = 4).ml(16.0))
    }},
    Sample { id: "selectorbar", page: Page::Navigation, title: "SelectorBar", keywords: "SelectorBar pivot segmented", view: |s, t| {
        let tk = t.tokens();
        let mut bar = row().key("selectorbar").role("tablist").gap(4.0);
        for (i, label) in ["Recent", "Shared", "Favorites"].into_iter().enumerate() {
            let sel = s.selector == i;
            let fg = if sel { tk.text_fill_color_primary } else { tk.text_fill_color_secondary };
            let pill = el().size(16.0, 3.0).radius(1.5).fill(c(if sel { tk.accent_fill_color_default } else { tk.subtle_fill_color_transparent }));
            bar = bar.child(materials::reveal(col().key(format!("selector-{i}")).role("tab").label(label).checked(sel).h(40.0).px(12.0).center().gap(6.0).radius(radii::CONTROL)
                .child(if sel { strong(label, fg) } else { body(label, fg) }).child(pill).on_press(move |s: &mut State| s.selector = i), t));
        }
        bar
    }},
    // Status
    Sample { id: "infobar", page: Page::Status, title: "InfoBar severities", keywords: "InfoBar info success warning error message", view: |s, t| {
        let bars = [
            (Severity::Informational, "Informational", "Essential app message for your users to be informed of, acknowledge, or take action on."),
            (Severity::Success, "Success", "The document was saved."),
            (Severity::Warning, "Warning", "Your connection is metered."),
            (Severity::Error, "Error", "Couldn't save the file."),
        ];
        let mut colm = col().key("infobars").w_full().gap(8.0);
        for (i, (sev, title, msg)) in bars.into_iter().enumerate() {
            if s.infobars[i] {
                colm = colm.child(k::info_bar(t, &format!("infobar-{i}"), sev, title, msg, move |s: &mut State| s.infobars[i] = false));
            }
        }
        colm.child(k::button(t, "infobar-reset", "Show all", ButtonStyle::Standard, true, |s: &mut State| s.infobars = [true; 4]))
    }},
    Sample { id: "progressbar", page: Page::Status, title: "ProgressBar: determinate and indeterminate", keywords: "ProgressBar progress loading", view: |s, t| {
        let tk = t.tokens();
        col().gap(20.0)
            .child(row().gap(16.0).items_center().child(k::progress_bar(t, "progress-det", 260.0, Some(s.slider))).child(caption(format!("{:.0}%", s.slider * 100.0), tk.text_fill_color_secondary)))
            .child(k::progress_bar(t, "progress-indet", 260.0, None))
    }},
    Sample { id: "progressring", page: Page::Status, title: "ProgressRing: determinate and indeterminate", keywords: "ProgressRing spinner loading", view: |s, t| {
        row().gap(32.0).items_center().child(k::progress_ring(t, "ring-indet", 32.0, None)).child(k::progress_ring(t, "ring-det", 48.0, Some(s.slider)))
    }},
    Sample { id: "tooltip", page: Page::Status, title: "ToolTip and InfoBadge", keywords: "ToolTip tooltip hover InfoBadge badge", view: |_, t| {
        let tk = t.tokens();
        let mail = el().size(40.0, 36.0).center().child(icon(Symbol::Mail, 20.0, tk.text_fill_color_primary)).child(el().absolute(24.0, 4.0).child(k::info_badge(t, Some(8))));
        let bell = el().size(40.0, 36.0).center().child(icon(Symbol::Alert, 20.0, tk.text_fill_color_primary)).child(el().absolute(28.0, 8.0).child(k::info_badge(t, None)));
        row().gap(24.0).items_center()
            .child(col().gap(6.0).child(k::button(t, "tooltip-target", "Button with a ToolTip", ButtonStyle::Standard, true, |_: &mut State| {})).child(k::tooltip(t, "Simple ToolTip")))
            .child(mail).child(bell)
    }},
    Sample { id: "teachingtip", page: Page::Status, title: "TeachingTip", keywords: "TeachingTip tip coach", view: |s, t| {
        let tk = t.tokens();
        let mut c0 = col().gap(8.0).child(k::button(t, "tip-toggle", "Show TeachingTip", ButtonStyle::Standard, true, |s: &mut State| s.tip_open = !s.tip_open));
        if s.tip_open {
            let tip = k::flyout_surface(t).key("teachingtip").role("dialog").w(320.0).padding(12.0).gap(8.0)
                .child(row().justify_between().child(strong("Saving automatically", tk.text_fill_color_primary))
                    .child(el().key("tip-close").role("button").label("Close").size(24.0, 24.0).center().radius(radii::CONTROL).hover_fill(c(tk.subtle_fill_color_secondary)).child(icon(Symbol::Dismiss, 12.0, tk.text_fill_color_primary)).on_press(|s: &mut State| s.tip_open = false)))
                .child(body("We save your changes as you go — no need to save.", tk.text_fill_color_secondary))
                .child(k::button(t, "tip-action", "Got it", ButtonStyle::Accent, true, |s: &mut State| s.tip_open = false));
            c0 = c0.child(tip);
        }
        c0
    }},
    // Media
    Sample { id: "image", page: Page::Media, title: "An Image", keywords: "Image picture photo", view: |_, t| {
        let a = t.accent();
        el().key("image").role("img").label("Landscape").size(320.0, 180.0).radius(radii::CONTROL)
            .linear_gradient(160.0, [(0.0, c(a.light3)), (0.55, c(a.base)), (1.0, c(a.dark3))])
            .child(el().absolute(200.0, 30.0).size(48.0, 48.0).radius(24.0).fill(rune::Color::WHITE.alpha(0.85)))
    }},
    Sample { id: "personpicture", page: Page::Media, title: "PersonPicture", keywords: "PersonPicture avatar initials", view: |_, t| {
        let tk = t.tokens();
        row().gap(24.0).items_center().child(k::person_picture(t, "JB", 96.0)).child(k::person_picture(t, "AL", 64.0)).child(k::person_picture(t, "KC", 32.0))
            .child(el().size(64.0, 64.0).radius(32.0).center().fill(c(tk.control_alt_fill_color_quarternary)).child(icon(Symbol::Person, 32.0, tk.text_fill_color_primary)))
    }},
    Sample { id: "mediaplayer", page: Page::Media, title: "MediaPlayerElement transport controls", keywords: "MediaPlayerElement media video play", view: |s, t| {
        let tk = t.tokens();
        let video = el().w(480.0).h(200.0).radius(radii::CONTROL).linear_gradient(90.0, [(0.0, rune::Color::BLACK), (1.0, c(t.accent().dark3))]);
        let controls = k::flyout_surface(t).key("transport").w(480.0).padding(8.0).gap(8.0)
            .child(k::progress_bar(t, "media-position", 464.0, Some(0.35)))
            .child(row().w_full().justify_between().items_center()
                .child(caption("1:24", tk.text_fill_color_secondary))
                .child(k::icon_button(t, "media-play", if s.playing { Symbol::Pause } else { Symbol::Play }, None, ButtonStyle::Subtle, |s: &mut State| s.playing = !s.playing))
                .child(caption("4:02", tk.text_fill_color_secondary)));
        col().gap(4.0).child(video).child(controls)
    }},
    Sample { id: "expander", page: Page::Media, title: "Expander", keywords: "Expander collapse disclosure", view: |s, t| {
        let content = body("This is in the content of the Expander.", t.tokens().text_fill_color_primary);
        k::expander(t, "expander", "This text is in the header", s.expander_open, content, |s: &mut State| s.expander_open = !s.expander_open)
    }},
];
