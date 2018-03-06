#[macro_use]
extern crate enum_kinds;

use std::fmt::Debug;

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

#[derive(EnumKind)]
#[enum_kind_name(WithLifetimeKind)]
#[allow(dead_code)]
enum WithLifetime<'a> {
    First(&'a str)
}

#[derive(EnumKind)]
#[enum_kind_name(WithWhereClauseKind)]
#[allow(dead_code)]
enum WithWhereClause<'b, T> where T: Debug, T: 'b, T: ?Sized {
    First { value: &'b T }
}

#[test]
fn test_unnamed() {
    let first = UnnamedEnum::First("Example".to_owned(), 32);
    assert_eq!(UnnamedEnumKind::from(first), UnnamedEnumKind::First);
}

#[test]
fn test_named() {
    let foo = NamedEnum::Foo {
        foo: "Example".to_owned(),
        bar: 32
    };
    assert_eq!(NamedEnumKind::from(&foo), NamedEnumKind::Foo);
}

#[test]
fn test_with_lifetimes() {
    let first = WithLifetime::First("hello");
    assert_eq!(WithLifetimeKind::from(&first), WithLifetimeKind::First);
}

#[test]
fn test_with_where_clause() {
    let first = WithWhereClause::First {
        value: "hello"
    };
    assert_eq!(WithWhereClauseKind::from(&first), WithWhereClauseKind::First);
}



