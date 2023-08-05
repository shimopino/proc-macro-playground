use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    let optionized = fields.iter().map(|f| {
        // let original_type = f.ty.clone();

        // let mut segments = syn::punctuated::Punctuated::new();
        // segments.push_value(syn::PathSegment {
        //     ident: original_type.ident,
        //     // arguments:
        // });
        // let ty = syn::Type::Path(syn::TypePath {
        //     qself: None,
        //     path: syn::Path {
        //         leading_colon: None,
        //         segments,
        //     },
        // });

        // syn::Field {
        //     attrs: Vec::new(),
        //     vis: syn::Visibility::Inherited,
        //     ident: f.ident.clone(),
        //     colon_token: f.colon_token,
        //     mutability: f.mutability.clone(),
        //     ty,
        // }

        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            #name: std::option::Option<#ty>
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not sef"))?
        }
    });

    let build_empty = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: None
        }
    });

    let expanded = quote! {
        struct #bident {
            #(#optionized,)*
        }

        impl #name {
            fn builder() -> #bident {
                #bident {
                    #(#build_empty,)*
                }
            }
        }

        impl #bident {
            #(#methods)*
            // pub fn executable(&mut self, executable: String) -> &mut Self {
            //     self.executable = Some(executable);
            //     self
            // }

            // pub fn args(&mut self, args: Vec<String>) -> &mut Self {
            //     self.args = Some(args);
            //     self
            // }

            // pub fn env(&mut self, env: Vec<String>) -> &mut Self {
            //     self.env = Some(env);
            //     self
            // }

            // pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
            //     self.current_dir = Some(current_dir);
            //     self
            // }

            pub fn build(&mut self) ->Result<#name, Box<dyn std::error::Error>> {
                // if self.executable.is_none() {
                //     return Err("executable is not set".into());
                // }
                // if self.args.is_none() {
                //     return Err("args is not set".into());
                // }
                // if self.env.is_none() {
                //     return Err("env is not set".into());
                // }
                // if self.current_dir.is_none() {
                //     return Err("current_dir is not set".into());
                // }

                Ok(#name {
                    // executable: self.executable.clone().ok_or("executable is not set")?,
                    // args: self.args.clone().ok_or("args is not set")?,
                    // env: self.env.clone().ok_or("env is not set")?,
                    // current_dir: self.current_dir.clone().ok_or("current_dir is not set")?,

                    #(#build_fields,)*
                })
            }
        }
    };

    expanded.into()
}
