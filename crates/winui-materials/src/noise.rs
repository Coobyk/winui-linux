//! The Acrylic noise texture: a deterministic grey tile with the same six-level histogram as
//! WinUI's `NoiseAsset_256X256_PNG.png` (levels 0, 51, …, 255), drawn at 2% opacity.

use std::sync::OnceLock;

use rune::{El, Pixels, el};

/// Tile side in pixels.
pub const NOISE_SIZE: u32 = 64;
/// `AcrylicBrush::sc_noiseOpacity`.
pub const NOISE_OPACITY: f32 = 0.02;
/// Grey levels of the WinUI noise asset and each one's share of its 65,536 pixels.
pub const NOISE_LEVELS: [(u8, f32); 6] = [
    (0, 1311.0 / 65536.0),
    (51, 9094.0 / 65536.0),
    (102, 22034.0 / 65536.0),
    (153, 22214.0 / 65536.0),
    (204, 9351.0 / 65536.0),
    (255, 1532.0 / 65536.0),
];

fn hash(mut x: u32) -> u32 {
    // lowbias32 (Chris Wellons): a well-mixed stateless integer hash.
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^ (x >> 16)
}

fn generate() -> Vec<u8> {
    let n = (NOISE_SIZE * NOISE_SIZE) as usize;
    // Rank pixels by hash, then hand out levels by exact quota so the histogram matches.
    let mut order: Vec<(u32, usize)> = (0..n).map(|i| (hash(i as u32 ^ 0xACE1_0000), i)).collect();
    order.sort_unstable();
    let mut out = vec![0u8; n];
    let mut start = 0usize;
    let mut cum = 0.0f32;
    for (k, &(level, share)) in NOISE_LEVELS.iter().enumerate() {
        cum += share;
        let end = if k + 1 == NOISE_LEVELS.len() {
            n
        } else {
            (cum * n as f32).round() as usize
        };
        for &(_, i) in &order[start..end] {
            out[i] = level;
        }
        start = end;
    }
    out
}

/// The 64×64 single-channel noise tile, row-major. Identical on every run and platform.
pub fn noise_tile() -> &'static [u8] {
    static TILE: OnceLock<Vec<u8>> = OnceLock::new();
    TILE.get_or_init(generate)
}

/// The tile as opaque grey RGBA [`Pixels`] (build once and keep it; it's a handle).
pub fn noise_pixels() -> Pixels {
    let rgba = noise_tile().iter().flat_map(|&v| [v, v, v, 255]).collect();
    let px = Pixels::new();
    px.set(NOISE_SIZE, NOISE_SIZE, rgba);
    px
}

thread_local! {
    static PIXELS: Pixels = noise_pixels();
}

/// A layer pinned to its parent's edges that tiles the noise at [`NOISE_OPACITY`]
/// (source-over, like the Acrylic recipe's `CompositeStepEffect`).
pub fn noise_layer(radius: f32) -> El {
    PIXELS.with(|px| {
        el().key("winui-noise")
            .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .radius(radius)
            .pixels_tile(px, [NOISE_SIZE as f32, NOISE_SIZE as f32])
            .opacity(NOISE_OPACITY)
    })
}
