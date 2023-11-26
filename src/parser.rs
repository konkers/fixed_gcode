use fixed::traits::FixedSigned;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit0, digit1, one_of, space0},
    combinator::{map_res, opt, recognize, verify},
    multi::fold_many0,
    IResult,
};

use crate::{BufferTypes, Line, TryPush, Word};

pub(crate) struct LineBuilder<Value: FixedSigned, Types: BufferTypes<Value>> {
    pub line: Line<Value, Types>,
    pub overflow_count: usize,
}

impl<Value: FixedSigned, Types: BufferTypes<Value>> LineBuilder<Value, Types> {
    fn new() -> Self {
        Self {
            line: Line::new(),
            overflow_count: 0,
        }
    }

    fn add_word(&mut self, word: Word<Value>) {
        if self.line.words.try_push(word).is_err() {
            self.overflow_count += 1;
        }
    }
}

pub fn just_integer_recognizer(i: &str) -> IResult<&str, ()> {
    let (i, _) = digit1(i)?;

    Ok((i, ()))
}

pub fn just_fract_recognizer(i: &str) -> IResult<&str, ()> {
    let (i, _) = tag(".")(i)?;
    let (i, _) = digit1(i)?;

    Ok((i, ()))
}

pub fn integer_and_fract_recognizer(i: &str) -> IResult<&str, ()> {
    let (i, _) = digit1(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = digit0(i)?;

    Ok((i, ()))
}

pub fn value_recognizer(i: &str) -> IResult<&str, ()> {
    let (i, _) = opt(one_of("+-"))(i)?;
    let (i, _) = alt((
        // `integer_and_fract_recognizer` must be first to keep one of the
        // other recognizers from only partially matching the value.
        integer_and_fract_recognizer,
        just_fract_recognizer,
        just_integer_recognizer,
    ))(i)?;

    Ok((i, ()))
}

pub fn value<Value: FixedSigned>(i: &str) -> IResult<&str, Value> {
    map_res(recognize(value_recognizer), |s: &str| s.parse::<Value>())(i)
}

pub fn word<Value: FixedSigned>(i: &str) -> IResult<&str, Word<Value>> {
    let (i, letter) = verify(anychar, |c: &char| c.is_alphabetic())(i)?;
    let (i, value) = value(i)?;

    Ok((
        i,
        Word {
            letter: letter.to_ascii_uppercase(),
            value,
        },
    ))
}

pub fn word_and_whitespace<Value: FixedSigned>(i: &str) -> IResult<&str, Word<Value>> {
    let (i, word) = word(i)?;
    // Consume any whitespace after the word.
    let (i, _) = space0(i)?;
    Ok((i, word))
}

pub fn line<Value: FixedSigned, Types: BufferTypes<Value>>(
    i: &str,
) -> IResult<&str, LineBuilder<Value, Types>> {
    fold_many0(
        word_and_whitespace,
        || LineBuilder::new(),
        |mut line: LineBuilder<Value, Types>, word| {
            line.add_word(word);
            line
        },
    )(i)
}
