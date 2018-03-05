#[macro_use]
extern crate enum_kinds_macros;
extern crate enum_kinds_traits;

use enum_kinds_traits::ToKind;

#[derive(EnumKind)]
#[enum_kind_name(UnnamedEnumKind)]
#[allow(dead_code)]
enum UnnamedEnum {
    First(String, u32),
    Second(char),
    Third
}

#[derive(EnumKind)]
#[enum_kind_name(NamedEnumKind)]
#[allow(dead_code)]
enum NamedEnum {
    Foo {
        foo: String,
        bar: u32
    },
    Bar {
        zap: char
    }
}

#[test]
fn test_unnamed() {
    let first = UnnamedEnum::First("Example".to_owned(), 32);
    assert_eq!(first.kind(), UnnamedEnumKind::First);
}

#[test]
fn test_named() {
    let foo = NamedEnum::Foo {
        foo: "Example".to_owned(),
        bar: 32
    };
    assert_eq!(foo.kind(), NamedEnumKind::Foo);
}
