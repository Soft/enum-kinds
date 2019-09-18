//! Custom derive for generating enums with matching variants but without any of
//! the data.
//!
//! In other words, `enum-kinds` automatically generates enums that have the
//! same set of variants as the original enum, but with all the embedded data
//! stripped away (that is, all the variants of the newly generated enum are
//! unit variants). Additionally, `enum-kinds` implements `From` trait for going
//! from the original enum to the unit variant version.
//!
//! The crate is compatible with stable Rust releases. This crate replaces
//! earlier `enum_kinds_macros` and `enum_kinds_traits` crates.
//!
//! # Example
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate enum_kinds;
//!
//! #[derive(EnumKind)]
//! #[enum_kind(SomeEnumKind)]
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
//! The `#[derive(EnumKind)]` attribute automatically creates another `enum`
//! named `SomeEnumKind` that contains matching unit variant for each of the
//! variants in `SomeEnum`.
//!
//! # Additional Traits for Generated Enums
//!
//! By default, derived kind enums implement `Debug`, `Clone`, `Copy`,
//! `PartialEq` and `Eq` traits. Additional derives can be specified by passing
//! derive specifier to the `enum_kind` attribute: `#[enum_kind(NAME,
//! derive(TRAIT, ...))]`. For example, to implement [Serde's](https://serde.rs)
//! Serialize and Deserialize traits:
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate enum_kinds;
//!
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde;
//!
//! #[derive(EnumKind)]
//! #[enum_kind(AdditionalDerivesKind, derive(Serialize, Deserialize))]
//! enum AdditionalDerives {
//!     Variant(String, u32),
//!     Another(String)
//! }
//! ```
//!
//! # no_std support
//!
//! `enum-kinds` can be used without the standard library by enabling
//! `no-stdlib` feature.
//!

extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, NestedMeta, Data, MetaList, DataEnum,
          Fields, Path, LifetimeDef, GenericParam, Lifetime};
use syn::punctuated::Punctuated;
use std::collections::HashSet;
use std::iter::FromIterator;

#[proc_macro_derive(EnumKind, attributes(enum_kind))]
pub fn enum_kind(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("#[derive(EnumKind)] failed to parse input");
    let (name, traits) = get_enum_specification(&ast);
    let enum_ = create_kind_enum(&ast, &name, traits);
    let impl_ = create_impl(&ast, &name);
    let code = quote! {
        #enum_
        #impl_
    };
    proc_macro::TokenStream::from(code)
}

fn find_attribute(definition: &DeriveInput, name: &str)
                  -> Option<Punctuated<NestedMeta, syn::token::Comma>> {
    for attr in definition.attrs.iter() {
        match attr.parse_meta() {
            Ok(Meta::List(MetaList { ref path, ref nested, .. }))
                if path.is_ident(name) => return Some(nested.clone()),
            _ => continue
        }
    }
    None
}

fn get_enum_specification(definition: &DeriveInput) -> (Path, Vec<NestedMeta>) {
    let params = find_attribute(definition, "enum_kind")
        .expect("#[derive(EnumKind)] requires an associated enum_kind attribute to be specified");
    let mut iter = params.iter();
    if let Some(&NestedMeta::Meta(Meta::Path(ref path))) = iter.next() {
        return (path.to_owned(), iter.cloned().collect());
    } else {
        panic!("#[enum_kind(NAME)] attribute requires NAME to be specified");
    }
}

fn create_kind_enum(definition: &DeriveInput, kind_ident: &Path, traits: Vec<NestedMeta>) -> TokenStream {
    let variant_idents = match &definition.data {
        &Data::Enum(DataEnum { ref variants, .. }) => {
            variants.iter().map(|ref v| v.ident.clone())
        }
        _ => {
            panic!("#[derive(EnumKind)] is only allowed for enums");
        }
    };
    let visibility = &definition.vis;
    let code = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(missing_docs)]
        #( #[#traits] )*
        #visibility enum #kind_ident {
            #(#variant_idents),*
        }
    };
    TokenStream::from(code)
}

fn is_uninhabited_enum(definition: &DeriveInput) -> bool {
    if let Data::Enum(ref data) = definition.data {
        return data.variants.len() == 0;
    }
    return false;
}

fn create_impl(definition: &DeriveInput, kind_ident: &Path) -> TokenStream {
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

    let impl_ = if is_uninhabited_enum(definition) {
        quote! {
            unreachable!();
        }
    } else {
        quote!{
            match _value {
                #(#arms)*
            }
        }
    };

    let tokens = quote! {
        #[automatically_derived]
        #[allow(unused_attributes)]
        #[allow(missing_docs)]
        impl #impl_generics #trait_<&#a #ident#ty_generics> for #kind_ident #where_clause {
            fn from(_value: &#a #ident#ty_generics) -> Self {
                #impl_
            }
        }

        #[automatically_derived]
        #[allow(unused_attributes)]
        #[allow(missing_docs)]
        impl #impl_generics #trait_<#ident#ty_generics> for #kind_ident #where_clause {
            fn from(value: #ident#ty_generics) -> Self {
                #kind_ident::from(&value)
            }
        }
    };
    TokenStream::from(tokens)
}

