use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // println!("{:#?}", ast);
    // TokenStream::new()

    let name = &ast.ident;
    println!("{:#?}", name);
    let bname = format!("{}Builder", name);
    println!("{:#?}", bname);
    let bident = syn::Ident::new(&bname, name.span());
    println!("name.span -> {:#?}", name.span());
    println!("Span::call_site() -> {:#?}", Span::call_site());
    println!("{:#?}", bident);

    let expanded = quote! {
        struct #bident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name {
            fn builder() -> #bident {
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    expanded.into()
}
