use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Type};

enum InnerType {
    OptionType(Type),
    VecType(Type),
    PrimitiveType,
}

/// Returns InnerType enum with unwrapped Type
fn unwrap_ty(ty: &Type) -> InnerType {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        if segments.len() == 1 {
            if let Some(syn::PathSegment {
                ident,
                arguments:
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args, ..
                    }),
            }) = segments.first()
            {
                if args.len() == 1 {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        if ident == "Option" {
                            return InnerType::OptionType(inner_ty.clone());
                        } else if ident == "Vec" {
                            return InnerType::VecType(inner_ty.clone());
                        }
                    }
                }
            }
        }
    }

    InnerType::PrimitiveType
}

fn unwrap_builder_attr_value(attr: &syn::Attribute) -> Option<String> {
    if attr.path().is_ident("builder") {
        if let Ok(syn::MetaNameValue {
            value:
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(ref liststr),
                    ..
                }),
            ..
        }) = attr.parse_args::<syn::MetaNameValue>()
        {
            return Some(liststr.value());
        } else {
            return None;
        }
    }

    None
}

#[proc_macro_derive(Builder, attributes(builder))]
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

        match unwrap_ty(ty) {
            InnerType::OptionType(_) => {
                quote! {
                    #ident: #ty
                }
            }
            _ => quote! {
                #ident: Option<#ty>
            },
        }
    });

    let builder_setters = named.iter().map(|f| {
        if !f.attrs.is_empty() {
            for attr in &f.attrs {
                match unwrap_builder_attr_value(attr) {
                    Some(value) => println!("each = {}", value),
                    None => println!("unexpected"),
                }
            }
        }

        let ident = &f.ident;
        let ty = &f.ty;

        match unwrap_ty(ty) {
            InnerType::OptionType(inner_ty) => quote! {
                fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            },
            _ => quote! {
                fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            },
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
        let ty = &f.ty;

        match unwrap_ty(ty) {
            InnerType::OptionType(_) => quote! {
                #ident: self.#ident.take()
            },
            _ => quote! {
                #ident: self.#ident.take().ok_or(format!("{} is not set", stringify!(#ident)))?
            },
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
