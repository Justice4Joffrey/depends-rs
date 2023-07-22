//! Derive node requirements for primitive types.

macro_rules! impl_for_primitive {
    ($($ty:ty),*) => {
        $(
            impl crate::execution::Named for $ty {
                fn name() -> &'static str {
                    stringify!($ty)
                }
            }

            impl crate::execution::HashValue for $ty {
                fn hash_value(&self, hasher: &mut impl std::hash::Hasher) -> crate::execution::NodeHash {
                    use std::hash::Hash;
                    crate::execution::NodeHash::Hashed({
                        self.hash(hasher);
                        hasher.finish()
                    })
                }
            }

            impl crate::execution::Clean for $ty {
                fn clean(&mut self) {}
            }
        )*
    };
}

impl_for_primitive! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    bool,
    char,
    String
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    use crate::execution::{Clean, HashValue, Named};

    #[test]
    fn test_hash_value() {
        let mut hasher = DefaultHasher::new();
        let hash = 42u32.hash_value(&mut hasher);
        assert_eq!(
            hash,
            crate::execution::NodeHash::Hashed(15387811073369036852)
        );
    }

    #[test]
    fn test_name() {
        assert_eq!(u32::name(), "u32");
    }

    #[test]
    fn test_clean() {
        let mut value = 420u16;
        value.clean();
        assert_eq!(value, 420u16);
    }
}
