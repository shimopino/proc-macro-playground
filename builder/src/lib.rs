use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // println!("{:#?}", ast);
    // TokenStream::new()

    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());
    let expanded = quote! {
        struct #bident {

        }

        impl #name {
            fn builder() -> #bident {
                #bident {

                }
            }
        }
    };

    expanded.into()
}
