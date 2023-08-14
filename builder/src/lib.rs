use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Type};

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

/// unwrap first value from #[builder(each = value)] attribute
fn unwrap_builder_attr_value(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
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
    })
}

fn extract_named_fields(data: &syn::Data) -> &Punctuated<syn::Field, syn::token::Comma> {
    let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed{ named, .. }), .. }) = data else {
        unimplemented!("This macro can only be applied to struct");
    };

    named
}

// ty の場合も inner_ty の場合も同じ構造なので、依存を引数に移動させて、生成するストリームを制御する
fn generate_default_setter_with(
    ident: &Option<syn::Ident>,
    ty: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        fn #ident(&mut self, #ident: #ty) -> &mut Self {
            self.#ident = Some(#ident);
            self
        }
    }
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = parsed.ident;
    let builder_ident = format_ident!("{}Builder", ident);
    let named = extract_named_fields(&parsed.data);

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
        let ident = &f.ident;
        let ty = &f.ty;

        match unwrap_ty(ty) {
            InnerType::VecType(inner_ty) => {
                let default_setter = generate_default_setter_with(ident, ty);

                if let Some(each) = unwrap_builder_attr_value(&f.attrs) {
                    let each_ident = format_ident!("{}", each);
                    let vec_setters = quote! {
                        fn #each_ident(&mut self, #each_ident: #inner_ty) -> &mut Self {
                            if let Some(ref mut values) = self.#ident {
                                values.push(#each_ident);
                            } else {
                                self.#ident = Some(vec![#each_ident]);
                            }
                            self
                        }
                    };

                    if ident.clone().unwrap() == each_ident {
                        return vec_setters;
                    } else {
                        return quote! {
                            #vec_setters
                            #default_setter
                        };
                    }
                } else {
                    return default_setter;
                }
            }
            InnerType::OptionType(inner_ty) => generate_default_setter_with(ident, &inner_ty),
            InnerType::PrimitiveType => generate_default_setter_with(ident, ty),
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
            InnerType::VecType(_) => quote! {
                #ident: self.#ident.take().unwrap_or_else(Vec::new)
            },
            InnerType::PrimitiveType => quote! {
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
