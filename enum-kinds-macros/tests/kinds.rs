#[macro_use]
extern crate enum_kinds_macros;
extern crate enum_kinds_traits;

use enum_kinds_traits::ToKind;

#[derive(EnumKind)]
#[enum_kind_name(SomeEnumKind)]
#[allow(dead_code)]
enum SomeEnum {
    First(String, u32),
    Second(char),
    Third
}

#[test]
fn test_enum_kind() {
    let first = SomeEnum::First("Example".to_owned(), 32);
    assert_eq!(first.kind(), SomeEnumKind::First);
}
