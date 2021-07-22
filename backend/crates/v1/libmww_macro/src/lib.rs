use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(NamedTupleFrom)]
pub fn named_tuple_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;
    let expanded = quote! {
        impl From<#type_name> for String {
            fn from(v: #type_name) -> String {
                v.0
            }
        }
    };
    expanded.into()
}
