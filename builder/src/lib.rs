use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());

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

        impl #bident {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&mut self) ->Result<#name, Box<dyn std::error::Error>> {
                if self.executable.is_none() {
                    return Err("executable is not set".into());
                }
                if self.args.is_none() {
                    return Err("args is not set".into());
                }
                if self.env.is_none() {
                    return Err("env is not set".into());
                }
                if self.current_dir.is_none() {
                    return Err("current_dir is not set".into());
                }

                Ok(#name {
                    executable: self.executable.clone().ok_or("executable is not set")?,
                    args: self.args.clone().ok_or("args is not set")?,
                    env: self.env.clone().ok_or("env is not set")?,
                    current_dir: self.current_dir.clone().ok_or("current_dir is not set")?,
                })
            }
        }
    };

    expanded.into()
}
