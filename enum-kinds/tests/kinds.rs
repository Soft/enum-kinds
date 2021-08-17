#[macro_use]
extern crate enum_kinds;

use std::fmt::Debug;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(EnumKind)]
#[enum_kind(UnnamedEnumKind)]
#[allow(dead_code)]
enum UnnamedEnum {
    First(String, u32),
    Second(char),
    Third,
}

#[derive(EnumKind)]
#[enum_kind(NamedEnumKind)]
#[allow(dead_code)]
enum NamedEnum {
    Foo { foo: String, bar: u32 },
    Bar { zap: char },
}

#[derive(EnumKind)]
#[enum_kind(WithLifetimeKind)]
#[allow(dead_code)]
enum WithLifetime<'a> {
    First(&'a str),
}

#[derive(EnumKind)]
#[enum_kind(WithWhereClauseKind)]
#[allow(dead_code)]
enum WithWhereClause<'b, T>
where
    T: Debug,
    T: 'b,
    T: ?Sized,
{
    First { value: &'b T },
}

#[derive(EnumKind)]
#[enum_kind(WithCollisionKind)]
#[allow(dead_code)]
enum WithCollision<'__enum_kinds1> {
    First(&'__enum_kinds1 str),
}

#[derive(EnumKind)]
#[enum_kind(UninhabitedEnumKind)]
#[allow(dead_code)]
enum UninhabitedEnum {}

#[derive(EnumKind)]
#[enum_kind(WithExtraTraitsKind, derive(Serialize, Deserialize))]
#[allow(dead_code)]
enum WithExtraTraits {
    First(u32, u32),
    Second(String),
}

#[derive(EnumKind)]
#[enum_kind(WithExtraTraitsMultipleKind, derive(Serialize), derive(Deserialize))]
#[allow(dead_code)]
enum WithExtraTraitsMultiple {
    First(u32, u32),
    Second(String),
}

mod forbids_missing_docs {
    #![forbid(missing_docs)]

    #[derive(EnumKind)]
    #[enum_kind(WithDocumentationKind, doc = "a documented kind enum")]
    #[allow(dead_code)]
    enum WithDocumentation {
        First(u32, u32),
        Second(String),
    }
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
        bar: 32,
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
    let first = WithWhereClause::First { value: "hello" };
    assert_eq!(
        WithWhereClauseKind::from(&first),
        WithWhereClauseKind::First
    );
}

#[test]
fn test_with_collision() {
    let first = WithCollision::First("hello");
    assert_eq!(WithCollisionKind::from(&first), WithCollisionKind::First);
}

#[test]
fn test_with_extra_traits() {
    let first = WithExtraTraits::First(20, 30);
    let kind: WithExtraTraitsKind = first.into();
    serde_json::to_string(&kind).unwrap();
}

#[test]
fn test_with_extra_traits_multiple() {
    let first = WithExtraTraitsMultiple::First(20, 30);
    let kind: WithExtraTraitsMultipleKind = first.into();
    serde_json::to_string(&kind).unwrap();
}
