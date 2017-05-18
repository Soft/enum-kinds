//! Generate enums with matching variants, but without any of the associated data.
//! `enum-kinds-traits` crate contains trait definitions used by this crate.
//! 
//! In other words, `enum-kinds-macros` automatically generates `enum`s that have
//! the same set of variants as the original `enum`, but with all the embedded data
//! stripped away (that is, all the variants are unit variants). Additionally,
//! `enum-kinds-macros` implements `ToKind` trait for the original `enum` allowing
//! one to get the associated unit variant.
//! 
//! The crates are compatible with stable Rust releases.
//! 
//! # Example
//! 
//! ```rust,ignore
//! #[macro_use]
//! extern crate enum_kinds_macros;
//! extern crate enum_kinds_traits;
//! 
//! use enum_kinds_traits::ToKind;
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
//!     assert_eq!(first.kind(), SomeEnumKind::First);
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
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{MacroInput, MetaItem, NestedMetaItem,
          Ident, Body, VariantData};

#[proc_macro_derive(EnumKind, attributes(enum_kind_name))]
pub fn enum_kind(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let ast = syn::parse_macro_input(&string).unwrap();
    let name = get_enum_name(&ast)
        .expect("#[derive(EnumKind)] requires associated #[enum_kind_name(NAME)] to be specified");
    let enum_ = create_kind_enum(&ast, &name);
    let impl_ = create_impl(&ast, &name);
    let code = quote! {
        #enum_
        #impl_
    };
    code.parse().unwrap()
}

fn get_enum_name(definition: &MacroInput) -> Option<Ident> {
    definition.attrs.iter().find(|attr| {
        match attr.value {
            MetaItem::List(ref ident, _) if ident == "enum_kind_name" => true,
            _ => false
        }})
        .map(|attr| attr.value.clone())
        .map(|item| {
            if let MetaItem::List(_, vec) = item {
                if vec.len() != 1 {
                    panic!("#[enum_kind_name(NAME)] requires exactly one argument");
                }
                let item = vec.first().unwrap();
                if let &NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) = item {
                    return ident.clone()
                }
            }
            panic!("#[enum_kind_name(NAME)] requires an identifier");
        })
}

fn create_kind_enum(definition: &MacroInput, kind_ident: &Ident) -> Tokens {
    let variant_idents = match &definition.body {
        &Body::Enum(ref variants) => {
            variants.iter().map(|ref v| v.ident.clone())
        }
        _ => {
            panic!("#[derive(EnumKind)] is only defined for enums, not for structs");
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

fn create_impl(definition: &MacroInput, kind_ident: &Ident) -> Tokens {
    let (impl_generics, ty_generics, where_clause) = definition.generics.split_for_impl();
    let ident = &definition.ident;

    let arms = match &definition.body {
        &Body::Enum(ref variants) => {
            variants.iter().map(|ref v| {
                let variant = &v.ident;
                match v.data {
                    VariantData::Unit => quote! {
                        &#ident::#variant => #kind_ident::#variant,
                    },
                    VariantData::Tuple(_) => quote! {
                        &#ident::#variant(..) => #kind_ident::#variant,
                    },
                    VariantData::Struct(_) => quote! {
                        &#ident::#variant{..} => #kind_ident::#variant,
                    }
                }
            })
        }
        _ => {
            panic!("#[derive(EnumKind)] is only defined for enums, not for structs");
        }
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_attributes)]
        impl #impl_generics ::enum_kinds_traits::ToKind
            for #ident #ty_generics #where_clause {
                type Kind = #kind_ident;
                
                #[inline]
                fn kind(&self) -> Self::Kind {
                    match self {
                        #(#arms)*
                    }
                }
        }
    }
}
