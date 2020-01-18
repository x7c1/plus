//#![crate_type = "proc-macro"]

//#![recursion_limit = "192"]

extern crate proc_macro;

mod delegator;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn delegate_requesters(attr: TokenStream, item: TokenStream) -> TokenStream {
    //    println!("--- attr: {:#?}", &attr);
    //    for i in attr.clone().into_iter() {
    //        println!("i...{:#?}", i);
    //    }

    item
}

#[proc_macro_attribute]
pub fn derive_requester_macros(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_item: syn::ItemTrait = syn::parse(item).unwrap();
    let add_macro_rules = delegator::generate_macro_rules(&original_item);

    println!("add_macro_rules: {}", add_macro_rules);

    let expanded = quote! {
        #original_item
        #add_macro_rules
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn requester_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
