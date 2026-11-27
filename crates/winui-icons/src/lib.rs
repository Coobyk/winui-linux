//! Fluent System Icons glyph data. See docs/ARCHITECTURE.md for the contract.
//!
//! Path data is vendored from
//! [microsoft/fluentui-system-icons](https://github.com/microsoft/fluentui-system-icons)
//! (MIT, Copyright (c) 2020 Microsoft Corporation; see
//! `LICENSE-fluentui-system-icons`). Every path is in a 24x24 viewBox.
//! `src/generated.rs` is produced by `tools/gen.py` and committed, so building
//! needs no network.

#[rustfmt::skip]
mod generated;

pub use generated::Symbol;

/// Edge length of the viewBox every [`Glyph::Path`] is authored in.
pub const VIEWBOX: f32 = 24.0;

/// How a glyph is drawn: a font codepoint (Segoe Fluent Icons / vendor
/// subset) or raw SVG path data from fluentui-system-icons (MIT).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Glyph {
    Font(char),
    /// SVG path data in a [`VIEWBOX`]-sized square, fill-rule evenodd unless
    /// [`Symbol::fill_rule`] says otherwise.
    Path(&'static str),
}

/// SVG fill rule for a [`Glyph::Path`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FillRule {
    #[default]
    EvenOdd,
    NonZero,
}

/// fluentui-system-icons variant a symbol was taken from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Style {
    Regular,
    Filled,
}

impl Symbol {
    /// The vendored Fluent path for this symbol.
    pub const fn glyph(self) -> Glyph {
        Glyph::Path(self.path())
    }

    /// The Segoe Fluent Icons glyph, for hosts that have that font installed.
    pub const fn font_glyph(self) -> Option<Glyph> {
        match self.segoe_codepoint() {
            Some(c) => Some(Glyph::Font(c)),
            None => None,
        }
    }

    /// Looks a symbol up by its variant name, e.g. `"ChevronDown"`.
    pub fn from_name(name: &str) -> Option<Symbol> {
        Symbol::ALL.iter().copied().find(|s| s.name() == name)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn enough_symbols() {
        assert!(Symbol::ALL.len() >= 120, "{}", Symbol::ALL.len());
    }

    #[test]
    fn every_glyph_non_empty_path() {
        for &s in Symbol::ALL {
            let Glyph::Path(d) = s.glyph() else {
                panic!("{s}: not a path")
            };
            assert!(!d.trim().is_empty(), "{s}: empty path");
            assert!(
                d.starts_with(['M', 'm']),
                "{s}: path must start with moveto"
            );
            assert!(
                d.chars()
                    .all(|c| c.is_ascii_digit() || " .,-eEMmLlHhVvCcSsQqTtAaZz".contains(c)),
                "{s}: unexpected character in path data"
            );
        }
    }

    #[test]
    fn all_unique_and_complete() {
        let set: HashSet<Symbol> = Symbol::ALL.iter().copied().collect();
        assert_eq!(set.len(), Symbol::ALL.len(), "duplicate in Symbol::ALL");
        let names: HashSet<&str> = Symbol::ALL.iter().map(|s| s.name()).collect();
        assert_eq!(names.len(), Symbol::ALL.len());
        for &s in Symbol::ALL {
            assert_eq!(Symbol::from_name(s.name()), Some(s));
            assert_eq!(format!("{s:?}"), s.name());
        }
    }

    /// Distinct symbols draw distinct icons, except deliberate WinUI aliases.
    #[test]
    fn paths_unique_except_aliases() {
        const ALIASES: &[(Symbol, Symbol)] = &[
            (Symbol::Accept, Symbol::CheckMark),
            (Symbol::Cancel, Symbol::Dismiss),
        ];
        let mut seen: HashMap<&str, Symbol> = HashMap::new();
        for &s in Symbol::ALL {
            if let Some(&prev) = seen.get(s.path()) {
                assert!(ALIASES.contains(&(prev, s)), "{prev} and {s} share a path");
            } else {
                seen.insert(s.path(), s);
            }
        }
    }

    #[test]
    fn contract_symbols_exist() {
        for n in [
            "Accept",
            "Add",
            "Back",
            "Cancel",
            "CheckMark",
            "ChevronDown",
            "ChevronRight",
            "Delete",
            "Dismiss",
            "Edit",
            "Find",
            "Home",
            "MoreHorizontal",
            "Settings",
            "Share",
        ] {
            assert!(Symbol::from_name(n).is_some(), "{n}");
        }
    }

    #[test]
    fn segoe_codepoints_are_pua() {
        for &s in Symbol::ALL {
            if let Some(Glyph::Font(c)) = s.font_glyph() {
                assert!(('\u{E000}'..='\u{F8FF}').contains(&c), "{s}: {c:?}");
            }
        }
        assert_eq!(Symbol::Add.segoe_codepoint(), Some('\u{E710}'));
        assert_eq!(Symbol::Settings.segoe_codepoint(), Some('\u{E713}'));
    }

    #[test]
    fn fill_rules_default_evenodd() {
        assert_eq!(FillRule::default(), FillRule::EvenOdd);
        for &s in Symbol::ALL {
            let _ = s.fill_rule();
        }
    }
}
