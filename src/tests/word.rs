use fixed::{traits::ToFixed, types::extra::U16, FixedI32};

use crate::parser::*;
use crate::*;

type Value = FixedI32<U16>;

#[test]
fn interger_word_parses_correctly() {
    assert_eq!(
        word("G0"),
        Ok((
            "",
            Word::<Value> {
                letter: 'G',
                value: 0.to_fixed()
            }
        ))
    );
}

#[test]
fn letter_converts_to_upper_case() {
    assert_eq!(
        word("g0"),
        Ok((
            "",
            Word::<Value> {
                letter: 'G',
                value: 0.to_fixed()
            }
        ))
    );
}

#[test]
fn positive_interger_word_parses_correctly() {
    assert_eq!(
        word("X+1"),
        Ok((
            "",
            Word::<Value> {
                letter: 'X',
                value: 1.to_fixed()
            }
        ))
    );
}

#[test]
fn negative_interger_word_parses_correctly() {
    assert_eq!(
        word("X-1"),
        Ok((
            "",
            Word::<Value> {
                letter: 'X',
                value: (-1).to_fixed()
            }
        ))
    );
}

#[test]
fn non_alphanumeric_letter_returns_error() {
    assert!(word::<Value>("@-1").is_err(),);
}

#[test]
fn bare_letter_returns_error() {
    assert!(word::<Value>("X").is_err(),);
}

#[test]
fn no_letter_returns_error() {
    assert!(word::<Value>("1234").is_err(),);
}
