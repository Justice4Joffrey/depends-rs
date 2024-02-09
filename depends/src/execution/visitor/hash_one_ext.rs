//! Only stabilised since Rust 1.70.0 so must implement ourselves to retain
//! MSRV.
use std::hash::{BuildHasher, Hash, Hasher};

pub fn hash_one<BH: BuildHasher, T: Hash>(hash_builder: &BH, x: T) -> u64 {
    let mut hasher = hash_builder.build_hasher();
    x.hash(&mut hasher);
    hasher.finish()
}
