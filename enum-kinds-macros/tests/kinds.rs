#[macro_use]
extern crate enum_kinds_macros;
extern crate enum_kinds_traits;

use enum_kinds_traits::ToKind;

#[derive(EnumKind)]
#[enum_kind_name(FooKind)]
#[allow(dead_code)]
enum Foo {
    Bar(String, u32),
    Baz(u32),
    Zap
}

#[test]
fn test_enum_kind() {
    let bar = Foo::Bar("Hello".to_owned(), 42);
    assert_eq!(bar.kind(), FooKind::Bar);
}
