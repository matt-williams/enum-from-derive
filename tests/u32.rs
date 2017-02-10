#[macro_use]
extern crate enum_from_derive;
extern crate try_from;
use try_from::TryFrom;

#[derive(EnumFrom, EnumTryFrom, PartialEq, Debug)]
#[repr(u32)]
enum U32Enum {
    Foo = 1,
    Bar = 2,
}

#[test]
fn u32enum_foo_from() {
    assert_eq!(U32Enum::from(1 as u32), U32Enum::Foo);
}

#[test]
fn u32enum_bar_from() {
    assert_eq!(U32Enum::from(2 as u32), U32Enum::Bar);
}

#[test]
#[should_panic]
fn u32enum_unknown_from() {
    U32Enum::from(3 as u32);
}

#[test]
fn u32enum_foo_try_from() {
    assert_eq!(U32Enum::try_from(1 as u32), Ok(U32Enum::Foo));
}

#[test]
fn u32enum_bar_try_from() {
    assert_eq!(U32Enum::try_from(2 as u32), Ok(U32Enum::Bar));
}

#[test]
fn u32enum_unknown_try_from() {
    assert_eq!(U32Enum::try_from(3 as u32), Err(3 as u32));
}
