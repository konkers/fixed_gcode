use fixed::{traits::ToFixed, types::extra::U16, FixedI32};

use crate::parser::*;

#[test]
fn plain_integer_parses_correctly() {
    assert_eq!(value("123"), Ok(("", 123.to_fixed::<FixedI32<U16>>())));
}

#[test]
fn plus_integer_parses_correctly() {
    assert_eq!(value("+123"), Ok(("", 123.to_fixed::<FixedI32<U16>>())));
}

#[test]
fn negative_integer_parses_correctly() {
    assert_eq!(value("-123"), Ok(("", (-123).to_fixed::<FixedI32<U16>>())));
}

#[test]
fn integer_with_trailing_decimal_point_parses_correctly() {
    assert_eq!(value("123."), Ok(("", 123.to_fixed::<FixedI32<U16>>())));
}

#[test]
fn plus_integer_with_trailing_decimal_point_parses_correctly() {
    assert_eq!(value("+123."), Ok(("", 123.to_fixed::<FixedI32<U16>>())));
}

#[test]
fn negative_integer_with_trailing_decimal_point_parses_correctly() {
    assert_eq!(value("-123."), Ok(("", (-123).to_fixed::<FixedI32<U16>>())));
}

#[test]
fn plain_fixed_parses_correctly() {
    assert_eq!(
        value("123.456"),
        Ok(("", 123.456f32.to_fixed::<FixedI32<U16>>()))
    );
}

#[test]
fn plus_fixed_parses_correctly() {
    assert_eq!(
        value("+123.456"),
        Ok(("", 123.456f32.to_fixed::<FixedI32<U16>>()))
    );
}

#[test]
fn negative_fixed_parses_correctly() {
    assert_eq!(
        value("-123.456"),
        Ok(("", (-123.456f32).to_fixed::<FixedI32<U16>>()))
    );
}

// TODO:#1 - Add bad input tests for value.
