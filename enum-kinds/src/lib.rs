//! Custom derive for generating enums with matching variants but without any of
//! the data.
//! 
//! In other words, `enum-kinds` automatically generates `enum`s that have the
//! same set of variants as the original `enum`, but with all the embedded data
//! stripped away (that is, all the variants of the newly generated enum are
//! unit variants). Additionally, `enum-kinds` implements `From` trait for going
//! form the original enum to the unit variant version.
//! 
//! The crate is compatible with stable Rust releases. This crate replaces
//! earlier enum_kinds_macros and enum_kinds_traits crates.
//! 
//! # Example
//! 
//! ```rust,ignore
//! #[macro_use]
//! extern crate enum_kinds;
//! 
//! #[derive(EnumKind)]
//! #[enum_kind_name(SomeEnumKind)]
//! enum SomeEnum {
//!     First(String, u32),
//!     Second(char),
//!     Third
//! }
//! 
//! #[test]
//! fn test_enum_kind() {
//!     let first = SomeEnum::First("Example".to_owned(), 32);
//!     assert_eq!(SomeEnumKind::from(&first), SomeEnumKind::First);
//! }
//! ```
//! 
//! The `#[derive(EnumKind)]` attribute automatically creates another `enum` named
//! `SomeEnumKind` that contains matching unit variant for each of the variants in
//! `SomeEnum`. Additionally, `SomeEnum` implements `ToKind` trait that provides the
//! `kind` method for constructing matching values from `SomeEnumKind`.
//!

#[macro_use]
extern crate quote;
extern crate proc_macro;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{DeriveInput, Meta, NestedMeta, Ident, Data, MetaList, DataEnum,
          Fields, Path, LifetimeDef, GenericParam, Lifetime};
use syn::punctuated::Pair;
use std::collections::HashSet;

#[proc_macro_derive(EnumKind, attributes(enum_kind_name))]
pub fn enum_kind(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("#[derive(EnumKind)] failed to parse input");
    let name = get_enum_name(&ast)
        .expect("#[derive(EnumKind)] requires an associated #[enum_kind_name(NAME)] to be specified");
    let enum_ = create_kind_enum(&ast, &name);
    let impl_ = create_impl(&ast, &name);
    let code = quote! {
        #enum_
        #impl_
    };
    code.into()
}

fn get_enum_name(definition: &DeriveInput) -> Option<Ident> {
    for attr in definition.attrs.iter() {
        match attr.interpret_meta() {
            Some(Meta::List(MetaList { ident, ref nested, .. })) if ident == "enum_kind_name" => {
                if let Some(Pair::End(&NestedMeta::Meta(Meta::Word(ident)))) = nested.pairs().next() {
                    return Some(ident.clone());
                } else {
                    panic!("#[enum_kind_name(NAME)] requires an identifier NAME to be specified");
                }
            },
            _ => continue
        }
    }
    return None;
}

fn create_kind_enum(definition: &DeriveInput, kind_ident: &Ident) -> Tokens {
    let variant_idents = match &definition.data {
        &Data::Enum(DataEnum { ref variants, .. }) => {
            variants.iter().map(|ref v| v.ident.clone())
        }
        _ => {
            panic!("#[derive(EnumKind)] is only allowed for enums");
        }
    };
    let visibility = &definition.vis;
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #visibility enum #kind_ident {
            #(#variant_idents),*
        }
    }
}

fn create_impl(definition: &DeriveInput, kind_ident: &Ident) -> Tokens {
    let (_, ty_generics, where_clause) = definition.generics.split_for_impl();
    let ident = &definition.ident;

    let arms = match &definition.data {
        &Data::Enum(DataEnum { ref variants, .. }) => {
            variants.iter().map(|ref v| {
                let variant = &v.ident;
                match v.fields {
                    Fields::Unit => quote! {
                        &#ident::#variant => #kind_ident::#variant,
                    },
                    Fields::Unnamed(_) => quote! {
                        &#ident::#variant(..) => #kind_ident::#variant,
                    },
                    Fields::Named(_) => quote! {
                        &#ident::#variant{..} => #kind_ident::#variant,
                    }
                }
            })
        }
        _ => {
            panic!("#[derive(EnumKind)] is only allowed for enums");
        }
    };

    let trait_: Path = if cfg!(feature="no-stdlib") {
        parse_quote!(::core::convert::From)
    } else {
        parse_quote!(::std::convert::From)
    };

    let mut counter: u32 = 1;
    let used: HashSet<Lifetime> = definition.generics
        .lifetimes()
        .map(|ld| ld.lifetime.clone())
        .collect();
    let a = loop {
        let lifetime: Lifetime = syn::parse_str(&format!("'__enum_kinds{}", counter))
            .unwrap();
        if !used.contains(&lifetime) {
            break LifetimeDef::new(lifetime);
        }
        counter += 1;
    };

    let mut generics = definition.generics.clone();
    generics.params.insert(0, GenericParam::Lifetime(a.clone()));
    let (impl_generics, _, _) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        #[allow(unused_attributes)]
        impl #impl_generics #trait_<&#a #ident#ty_generics> for #kind_ident #where_clause {
            fn from(value: &#a #ident#ty_generics) -> Self {
                match value {
                    #(#arms)*
                }
            }
        }

        #[automatically_derived]
        #[allow(unused_attributes)]
        impl #impl_generics #trait_<#ident#ty_generics> for #kind_ident #where_clause {
            fn from(value: #ident#ty_generics) -> Self {
                #kind_ident::from(&value)
            }
        }
    }
}
