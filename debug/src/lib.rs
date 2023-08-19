use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);

    let original_ident = parsed.ident;
    let named = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = parsed.data
    {
        named
    } else {
        return syn::Error::new(
            original_ident.span(),
            "CustomDerive can only be applied to struct with named fields",
        )
        .to_compile_error()
        .into();
    };

    let field_calls = named.iter().map(|f| {
        let field_ident = &f.ident;

        quote! {
            .field(stringify!(#field_ident), &self.#field_ident)
        }
    });

    let expanded = quote! {
        impl std::fmt::Debug for #original_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#original_ident))
                    #(#field_calls)*
                    .finish()
            }
        }
    };

    expanded.into()
}
