extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn impl_sabi_s3_errors(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
