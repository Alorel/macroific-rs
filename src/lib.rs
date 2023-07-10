#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]

#![warn(missing_docs)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// # Returns
///
/// Always false :(
pub const fn take_over_the_world() -> bool {
    false
}
