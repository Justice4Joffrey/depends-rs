use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// How we should calculate this node's hash value.
pub enum HashLogic {
    /// Node doesn't implement hashing.
    Unhashable,
    /// Node derives `std::hash::Hash`.
    Struct,
    /// Field is used as a has value.
    Field(Ident),
}

impl HashLogic {
    pub fn to_tokens(&self) -> TokenStream {
        match self {
            HashLogic::Struct => {
                quote! {
                    ::depends::NodeHash::Hashed({
                        self.hash(hasher);
                        hasher.finish()
                    })
                }
            }
            HashLogic::Field(ident) => {
                quote! {
                    ::depends::NodeHash::Hashed({
                        self.#ident.hash(hasher);
                        hasher.finish()
                    })
                }
            }
            HashLogic::Unhashable => {
                quote! {
                    ::depends::NodeHash::NotHashed
                }
            }
        }
    }
}
