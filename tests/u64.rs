#[macro_use]
extern crate enum_from_derive;
extern crate try_from;
use try_from::TryFrom;

#[derive(EnumFrom, EnumTryFrom, PartialEq, Debug)]
#[repr(u64)]
enum U16Enum {
    Foo = 1,
    Bar = 2,
}

#[test]
fn u64enum_foo_from() {
    assert_eq!(U16Enum::from(1 as u64), U16Enum::Foo);
}

#[test]
fn u64enum_bar_from() {
    assert_eq!(U16Enum::from(2 as u64), U16Enum::Bar);
}

#[test]
#[should_panic]
fn u64enum_unknown_from() {
    U16Enum::from(3 as u64);
}

#[test]
fn u64enum_foo_try_from() {
    assert_eq!(U16Enum::try_from(1 as u64), Ok(U16Enum::Foo));
}

#[test]
fn u64enum_bar_try_from() {
    assert_eq!(U16Enum::try_from(2 as u64), Ok(U16Enum::Bar));
}

#[test]
fn u64enum_unknown_try_from() {
    assert_eq!(U16Enum::try_from(3 as u64), Err(3 as u64));
}
