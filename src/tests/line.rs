use fixed::{types::extra::U16, FixedI32};
use heapless::Vec;

use crate::*;

type Value = FixedI32<U16>;

#[derive(Debug)]
struct Types;

impl BufferTypes<Value> for Types {
    type Words = Vec<Word<Value>, 5>;
}

fn test_line_parse(s: &str, words: &[Word<Value>]) {
    let line: Line<Value, Types> = s.parse().unwrap();
    assert_eq!(
        line.words.len(),
        words.len(),
        "number of words {:?} does not match expected value {:?}",
        line.words,
        words
    );

    for (index, (value, expected)) in line.words.iter().zip(words.iter()).enumerate() {
        assert_eq!(
            value, expected,
            "argument {index} in {:?} does not match {:?}",
            line.words, words
        );
    }
}

#[test]
fn single_word_line_parses() {
    test_line_parse("G0", &[Word::new('G', 0)]);
}

#[test]
fn multiple_words_line_parses() {
    test_line_parse(
        "G0 X0.1 Y-1. Z+.05",
        &[
            Word::new('G', 0),
            Word::new('X', 0.1),
            Word::new('Y', -1),
            Word::new('Z', 0.05),
        ],
    );
}

#[test]
fn trailing_characters_returns_error() {
    assert_eq!(
        "G0 X0.1 Y-1. Z+.05____"
            .parse::<Line<Value, Types>>()
            .unwrap_err(),
        Error::ParseError
    );
}

#[test]
fn non_gcode_returns_error() {
    assert_eq!(
        "Hello World!".parse::<Line<Value, Types>>().unwrap_err(),
        Error::ParseError
    );
}

#[test]
fn too_many_arguments() {
    assert_eq!(
        "G0 A1 B2 C3 D4 E5 F6"
            .parse::<Line<Value, Types>>()
            .unwrap_err(),
        Error::ArgumentOverflow(2)
    );
}
