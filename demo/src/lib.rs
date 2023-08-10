use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = parsed.ident;
    let builder_ident = format_ident!("{}Builder", ident);

    let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }), .. }) = parsed.data else {
        panic!("This macro can only be applied to struct");
    };

    let builder_fields = named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            #ident: Option<#ty>
        }
    });

    let builder_setters = named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let builder_init = named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: None
        }
    });

    let build_fields = named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: self.#ident.take().ok_or(format!("{} is not set", stringify!(#ident)))?
        }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_fields,)*
        }

        impl #builder_ident {
            #(#builder_setters)*

            fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#build_fields,)*
                })
            }
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_init,)*
                }
            }
        }
    };

    expanded.into()
}
