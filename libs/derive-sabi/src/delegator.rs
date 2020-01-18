use quote::quote;

pub fn generate_macro_rules(original_item: &syn::ItemTrait) -> proc_macro2::TokenStream {
    let rules = quote! {
        macro_rules! define__operations__put_object__requester__methods {
            () => {
                pub fn put_object<A: operations::put_object::Request>(&self, request: A) -> String {
                    operations::put_object::Requester::put_object(self, request)
                }
            }
        }
    };
    rules
}
