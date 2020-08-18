extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// usage:
/// [wasabi/error.rs at d190b336fd33d2c6b1d1e4429b74a7408990883d](https://github.com/x7c1/wasabi/blob/d190b336fd33d2c6b1d1e4429b74a7408990883d/apps/s3api/src/error.rs#L38-L45)
#[proc_macro_attribute]
pub fn impl_plus_s3_errors(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item: syn::ItemTrait = syn::parse(item).unwrap();
    let trait_name = &original_item.ident;

    let expanded = quote! {
        #original_item

        impl #trait_name for sabi_s3::actions::Error {}
        impl #trait_name for sabi_s3::client::Error {}
        impl #trait_name for sabi_s3::core::Error {}
        impl #trait_name for sabi_s3::internal::Error {}
    };
    TokenStream::from(expanded)
}
