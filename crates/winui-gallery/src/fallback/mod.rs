//! Local stand-ins for sibling crate APIs that were not available when the gallery was built.
//! Each module mirrors its crate's contract in docs/ARCHITECTURE.md; the integrator swaps
//! `crate::fallback::x` for `winui_x` (see README "API gaps").

pub mod controls;
pub mod icons;
pub mod materials;
pub mod theme;
