extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::MacroInput;

const NAME_ATTRIBUTE: &str = "enum_kind_name";

#[proc_macro_derive(EnumKind, attributes(enum_kind_name))]
pub fn enum_kind(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let ast = syn::parse_macro_input(&string).unwrap();
    let name = get_enum_name(&ast)
        .expect(&format!("#[derive(EnumKind)] requires associated #[{}(NAME)] to be specified", NAME_ATTRIBUTE));
    let enum_ = create_kind_enum(&ast, &name);
    let impl_ = create_impl(&ast, &name);
    let code = quote! {
        #enum_
        #impl_
    };
    code.parse().unwrap()
}

fn get_enum_name(definition: &MacroInput) -> Option<syn::Ident> {
    definition.attrs.iter().find(|attr| {
        match attr.value {
            syn::MetaItem::List(ref ident, _) if ident == NAME_ATTRIBUTE => true,
            _ => false
        }})
        .map(|attr| attr.value.clone())
        .map(|item| {
            match item {
                syn::MetaItem::List(_, vec) => {
                    if vec.len() != 1 {
                        panic!("#[{}(NAME)] requires exactly one argument", NAME_ATTRIBUTE);
                    }
                    let item = vec.first().unwrap();
                    if let &syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) = item {
                        ident.clone()
                    } else {
                        panic!("#[{}(NAME)] requires identifier", NAME_ATTRIBUTE);
                    }
                }
                _ => {
                    panic!("#[{}(NAME)] requires identifier", NAME_ATTRIBUTE);
                }
            }})
}

fn create_kind_enum(definition: &MacroInput, kind_ident: &syn::Ident) -> Tokens {
    let variant_idents = match &definition.body {
        &syn::Body::Enum(ref variants) => {
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

fn create_impl(definition: &MacroInput, kind_ident: &syn::Ident) -> Tokens {
    let (impl_generics, ty_generics, where_clause) = definition.generics.split_for_impl();
    let ident = &definition.ident;

    let arms = match &definition.body {
        &syn::Body::Enum(ref variants) => {
            variants.iter().map(|ref v| {
                let variant = &v.ident;
                match v.data {
                    syn::VariantData::Unit => quote! {
                        &#ident::#variant => #kind_ident::#variant,
                    },
                    syn::VariantData::Tuple(_) => quote! {
                        &#ident::#variant(..) => #kind_ident::#variant,
                    },
                    syn::VariantData::Struct(_) => quote! {
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
                
                fn kind(&self) -> Self::Kind {
                    match self {
                        #(#arms)*
                    }
                }
        }
    }
}
