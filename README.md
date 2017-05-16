# enum-kinds

This repository contains two interconnected crates for working with `enum`
variants in Rust. `enum-kinds-macros` enables automatic generation of `enum`s
that represent different `enum` variants. `enum-kinds-traits` contains trait
definitions used by this macro.

In other words, `enum-kinds` derives `enums` with matching variants, but without
any of the associated data.

The crates are compatible with stable Rust releases.

# Example

```rust
#[macro_use]
extern crate enum_kinds_macros;
extern crate enum_kinds_traits;

use enum_kinds_traits::ToKind;

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
    assert_eq!(first.kind(), SomeEnumKind::First);
}
```

The `#[derive(EnumKind)]` attribute automatically generates another `enum` named
`SomeEnumKind` that contains matching unit variant for each variant in SomeEnum.
Additionally, `SomeEnum` implements `ToKind` trait that provides the `kind`
method for constructing matching values from `SomeEnumKind`.

# Issues

While the crates are fairly simple, issues are still possible. If you encounter
any problems using these crates, please report them
at [the issue tracker](https://bitbucket.org/Soft/enum-kinds/issues).

# License

The crates are available under the terms of [MIT license](https://opensource.org/licenses/MIT).
