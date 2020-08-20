extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// usage:
/// [s3api/error.rs at b7a0f4b39a1b03ed010d362fc91ca4f1e0b18a2d](https://github.com/x7c1/plus/blob/b7a0f4b39a1b03ed010d362fc91ca4f1e0b18a2d/apps/s3api/src/error.rs#L41-L48)
#[proc_macro_attribute]
pub fn impl_plus_s3_errors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item: syn::ItemTrait = syn::parse(item).unwrap();
    let trait_name = &original_item.ident;

    let expanded = quote! {
        #original_item

        impl #trait_name for plus_s3::actions::Error {}
        impl #trait_name for plus_s3::client::Error {}
        impl #trait_name for plus_s3::core::Error {}
        impl #trait_name for plus_s3::internal::Error {}
    };
    TokenStream::from(expanded)
}
