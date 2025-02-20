//! Only stabilised since Rust 1.70.0 so must implement ourselves to retain
//! MSRV.
use std::hash::{BuildHasher, Hash};

pub fn hash_one<BH: BuildHasher, T: Hash>(hash_builder: &BH, x: T) -> u64 {
    hash_builder.hash_one(&x)
}
