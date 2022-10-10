/// This is a dumping ground for all proc-macros used by passfish.
/// There's a corresponding runtime module in the main crate.
extern crate syn;
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Adds a global constant name ENUMTYPE_
/// Thanks, StackOverflow: https://stackoverflow.com/questions/41637978/how-to-get-the-number-of-elements-variants-in-an-enum-as-a-constant-value
#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let len = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let enum_name = syn_item.ident;
    let result = quote! {
        impl EnumVariantCount for #enum_name { fn variant_count() -> usize { #len } }
    };
    result.into()
}
