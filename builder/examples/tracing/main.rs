use quote::quote;
use std::collections::HashSet;
use syn::{ext::IdentExt, punctuated::Punctuated, Token};

mod kw {
    syn::custom_keyword!(name);
    syn::custom_keyword!(skip);
    syn::custom_keyword!(fields);
}

#[derive(Debug, Default)]
struct Args {
    name: Option<syn::LitStr>,
    skips: HashSet<syn::Ident>,
    fields: Option<Fields>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::name) {
                let NameValue(name) = input.parse()?;
                args.name = Some(name);
            } else if lookahead.peek(kw::skip) {
                let Skips(skips) = input.parse()?;
                args.skips = skips
            } else if lookahead.peek(kw::fields) {
                args.fields = Some(input.parse()?);
            } else if lookahead.peek(Token![,]) {
                // 属性間の区切り記号 skip(xxx), <- こういったもの
                let _ = input.parse::<Token![,]>()?;
            } else {
                return Err(syn::Error::new(input.span(), "unexpected token"));
            }
        }

        Ok(args)
    }
}

struct NameValue(syn::LitStr);

impl syn::parse::Parse for NameValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<kw::name>()?;
        let _ = input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self(value))
    }
}

struct Skips(HashSet<syn::Ident>);

impl syn::parse::Parse for Skips {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<kw::skip>()?;
        let content;
        let _ = syn::parenthesized!(content in input);
        let names = content.parse_terminated(syn::Ident::parse_any, Token![,])?;
        let mut skips = HashSet::new();
        for name in names {
            if skips.contains(&name) {
                continue;
            }
            skips.insert(name);
        }
        Ok(Self(skips))
    }
}

#[derive(Debug)]
struct Fields(Punctuated<Field, Token![,]>);

impl syn::parse::Parse for Fields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<kw::fields>()?;
        let content;
        let _ = syn::parenthesized!(content in input);
        let fields = content.parse_terminated(Field::parse, Token![,])?;
        Ok(Self(fields))
    }
}

#[derive(Debug)]
struct Field {
    key: syn::Ident,
    value: Option<syn::Expr>,
}

impl syn::parse::Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let value = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { key, value })
    }
}

fn main() {
    let tokens = quote! {
        name = "sample",
        skip(form, state),
        fields(
            username=name,
        )
    };

    match syn::parse2::<Args>(tokens) {
        Ok(args) => {
            println!("args - {:#?}", args.name);
            println!("skips - {:#?}", args.skips);
            println!("fields - {:#?}", args.fields);
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
