use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FlatXML)]
pub fn flat_xml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let field_inits = if let Data::Struct(data) = input.data {
        data.fields
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                // let field_type = &field.ty;

                quote! {
                    #field_name: Default::default()
                }
            })
            .collect::<Vec<_>>()
    } else {
        panic!("FlatXML can only be derived for structs");
    };

    let expand = quote! {
        impl #name {
            pub fn parse_xml(xml_path: &std::path::Path) -> Self {
                Self {
                    #(#field_inits),*
                }
            }
        }
    };

    TokenStream::from(expand)
}
