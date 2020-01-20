use proc_macro2::Ident;
use quote::quote;
use syn::Signature;

pub fn generate_macro_rules(original_item: &syn::ItemTrait) -> proc_macro2::TokenStream {
    println!("...item trait: {:#?}", original_item);

    println!("trait ident: {}", original_item.ident.to_string());
    let trait_id = &original_item.ident;

    let original_trait_methods: Vec<&syn::TraitItemMethod> = original_item
        .items
        .iter()
        .map(|n| match n {
            syn::TraitItem::Method(method) => method,
            _ => unimplemented!(),
        })
        .collect();

    let methods = build_methods(trait_id, &*original_trait_methods);
    let xs = quote! {
        #(#methods)*
    };
    println!(".....{}", xs);
    println!("module...{}", module_path!());

    let rules = quote! {
            macro_rules! define__operations__put_object__requester__methods {
                ($field_ident:path) => {
    //                #(#methods)*

                    pub fn put_object<A: operations::put_object::Request>(&self, request: A) -> String {
                        operations::put_object::Requester::put_object(self, request)
                    }
                }
            }
        };
    rules
}

fn build_methods(
    trait_id: &Ident,
    original_trait_methods: &[&syn::TraitItemMethod],
) -> Vec<proc_macro2::TokenStream> {
    let mut methods = vec![];
    for original_method in original_trait_methods {
        let method_sig = &original_method.sig;
//        method_sig.generics.where_clause.unwrap().predicates[0].0

        let mut cloned_sig = method_sig.clone();
        let method_sig2 = build_method_sig(&mut cloned_sig);

        //        println!("method_sig: {:#?}", method_sig2);

        let method_invocation =
            build_method_invocation(original_method, &quote!($field_ident::#trait_id));

        let method_impl = quote! {
            #method_sig {
                #method_invocation
            }
        };
        methods.push(method_impl);
    }
    methods
}

fn build_method_sig(cloned_sig: &mut Signature) -> proc_macro2::TokenStream {
    // attempt at replacing ident
    //    cloned_sig.ident = syn::parse_str("hoge").unwrap();
    let sig = quote! { #cloned_sig };

    println!("original_sig: {:#?}", cloned_sig);

    println!("original_sig: {}", sig);
    //    original_sig.generics.
    sig
}

fn build_method_invocation(
    original_method: &syn::TraitItemMethod,
    field_ident: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let method_sig = &original_method.sig;
    let method_ident = &method_sig.ident;
    let argument_list: syn::punctuated::Punctuated<&Box<syn::Pat>, syn::token::Comma> = method_sig
        .inputs
        .iter()
        .filter_map(|fn_arg| match fn_arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(pat_type) => Some(&pat_type.pat),
        })
        .collect();

    let method_invocation = quote! { #field_ident::#method_ident(self, #argument_list) };
    method_invocation
}
