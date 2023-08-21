use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

fn unwrap_debug_attribute(
    attrs: &[syn::Attribute],
) -> Result<Vec<String>, proc_macro2::TokenStream> {
    let mut attrs_values = vec![];
    for attr in attrs {
        match attr.parse_args::<syn::MetaNameValue>() {
            Ok(named) if named.path.is_ident("debug") => {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(ref liststr),
                    ..
                }) = named.value
                {
                    attrs_values.push(liststr.value())
                }
            }
            Ok(_) => {
                return Err(
                    syn::Error::new(attr.span(), "only debug attributes can be applied")
                        .to_compile_error(),
                )
            }
            Err(err) => return Err(err.to_compile_error()),
        }
    }
    Ok(attrs_values)
}

#[proc_macro_derive(CustomDebug, attributes(debug))]
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
