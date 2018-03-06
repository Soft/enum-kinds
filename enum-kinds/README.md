# enum-kinds

[![Build Status](https://api.travis-ci.org/Soft/enum-kinds.svg?branch=master)](https://travis-ci.org/Soft/enum-kinds)
[![Latest Version](https://img.shields.io/crates/v/enum-kinds.svg)](https://crates.io/crates/enum-kinds)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/enum-kinds)

Custom derive for generating enums with matching variants but without any of the data.

In other words, `enum-kinds` automatically generates `enum`s that have the same
set of variants as the original `enum`, but with all the embedded data stripped
away (that is, all the variants of the newly generated enum are unit variants).
Additionally, `enum-kinds` implements `From` trait for going form the original
enum to the unit variant version.

The crate is compatible with stable Rust releases. This crate replaces earlier
enum_kinds_macros and enum_kinds_traits crates.

# Example

```rust
#[macro_use]
extern crate enum_kinds;

#[derive(EnumKind)]
#[enum_kind_name(SomeEnumKind)]
enum SomeEnum {
    First(String, u32),
    Second(char),
    Third
}

#[test]
fn test_enum_kind() {
    let first = SomeEnum::First("Example".to_owned(), 32);
    assert_eq!(SomeEnumKind::from(&first), SomeEnumKind::First);
}
```

The `#[derive(EnumKind)]` attribute automatically creates another `enum` named
`SomeEnumKind` that contains matching unit variant for each of the variants in
`SomeEnum`.

# no_std support

`enum-kinds` can be used without the standard library by enabling `no-stdlib`
feature.

# Issues

While the enum-kinds is fairly simple, issues are still possible. If you
encounter any problems using the crate, please report them at [the issue
tracker](https://bitbucket.org/Soft/enum-kinds/issues).

# License

The crates are available under the terms of [MIT license](https://opensource.org/licenses/MIT).