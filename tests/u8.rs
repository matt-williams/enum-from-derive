#[macro_use]
extern crate enum_from_derive;
extern crate try_from;
use try_from::TryFrom;

#[derive(EnumFrom, EnumTryFrom, PartialEq, Debug)]
#[repr(u8)]
enum U8Enum {
    Foo = 1,
    Bar = 2,
}

#[test]
fn u8enum_foo_from() {
    assert_eq!(U8Enum::from(1 as u8), U8Enum::Foo);
}

#[test]
fn u8enum_bar_from() {
    assert_eq!(U8Enum::from(2 as u8), U8Enum::Bar);
}

#[test]
#[should_panic]
fn u8enum_unknown_from() {
    U8Enum::from(3 as u8);
}

#[test]
fn u8enum_foo_try_from() {
    assert_eq!(U8Enum::try_from(1 as u8), Ok(U8Enum::Foo));
}

#[test]
fn u8enum_bar_try_from() {
    assert_eq!(U8Enum::try_from(2 as u8), Ok(U8Enum::Bar));
}

#[test]
fn u8enum_unknown_try_from() {
    assert_eq!(U8Enum::try_from(3 as u8), Err(3 as u8));
}
