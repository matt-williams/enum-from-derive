extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, ConstExpr, Ident, MetaItem, NestedMetaItem};

#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    let name = &ast.ident;
    let repr = get_repr(&ast);
    let variants = match ast.body {
        Body::Struct(_) => panic!("#[derive(EnumFrom)] can only be used with enums"),
        Body::Enum(ref variants) => variants,
    };
    let variant_values: Vec<ConstExpr> = variants.iter()
        .map(|variant| variant.discriminant.clone().unwrap_or_else(|| panic!("#[derive{EnumFrom}] requires all variants to have explicit values")))
        .collect();
    let names = std::iter::repeat(name);
    let variant_names: Vec<Ident> = variants.iter().map(|variant| variant.ident.clone()).collect();

    let gen = quote! {
        impl std::convert::From<#repr> for #name {
            fn from(value: #repr) -> #name {
                match value {
                    #(#variant_values => #names::#variant_names,)*
                    _ => panic!(concat!("enum ", stringify!(#name), " does not contain value {}"), value)
                }
            }
        }
    };

    gen.parse().unwrap()
}

#[proc_macro_derive(EnumTryFrom)]
pub fn enum_try_from(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    let name = &ast.ident;
    let repr = get_repr(&ast);
    let variants = match ast.body {
        Body::Struct(_) => panic!("#[derive(EnumTryFrom)] can only be used with enums"),
        Body::Enum(ref variants) => variants,
    };
    let variant_values: Vec<ConstExpr> = variants.iter()
        .map(|variant| variant.discriminant.clone().unwrap_or_else(|| panic!("#[derive{EnumTryFrom}] requires all variants to have explicit values")))
        .collect();
    let names = std::iter::repeat(name);
    let variant_names: Vec<Ident> = variants.iter().map(|variant| variant.ident.clone()).collect();

    let gen = quote! {
        impl TryFrom<#repr> for #name {
            type Err = #repr;
            fn try_from(value: #repr) -> std::result::Result<Self, Self::Err> {
                match value {
                    #(#variant_values => std::result::Result::Ok(#names::#variant_names),)*
                    _ => std::result::Result::Err(value)
                }
            }
        }
    };
    
    gen.parse().unwrap()
}

fn get_repr(ast: &syn::DeriveInput) -> syn::Ident {
    let repr = ast.attrs.iter()
        .flat_map(|e| match e.value {
            MetaItem::List(ref x, ref y) if *x == Ident::new("repr") => y.clone(),
            _ => Vec::new()})
        .filter_map(|e| match e {
            NestedMetaItem::MetaItem(MetaItem::Word(ref x)) => Some(x.clone()),
            _ => None})
        .nth(0);
    match repr {
        Some(ref x) if *x == Ident::new("u8") || *x == Ident::new("u16") || *x == Ident::new("u32") || *x == Ident::new("u64") => {},
        _ => panic!("#[derive(EnumTryFrom)] can only handle enums of one of repr(u8), repr(u16), repr(u32) or repr(u64)")
    };
    repr.unwrap()
}
