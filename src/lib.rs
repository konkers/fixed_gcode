#![cfg_attr(not(feature = "std"), no_std)]

use core::{fmt::Display, ops::Deref, str::FromStr};

use fixed::traits::{FixedSigned, ToFixed};
use heapless::Vec;

pub(crate) mod parser;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    ArgumentOverflow(usize),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word<Value: FixedSigned> {
    pub letter: char,
    pub value: Value,
}

impl<Value: FixedSigned> Word<Value> {
    pub fn new<T: ToFixed>(letter: char, value: T) -> Self {
        Self {
            letter,
            value: value.to_fixed(),
        }
    }

    pub const fn lit(letter: char, value: Value) -> Self {
        Self { letter, value }
    }
}

impl<Value: FixedSigned> Display for Word<Value> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.letter, self.value)
    }
}

pub trait TryPush<T> {
    fn try_push(&mut self, item: T) -> core::result::Result<(), T>;
}

impl<T, const N: usize> TryPush<T> for Vec<T, N> {
    fn try_push(&mut self, item: T) -> core::result::Result<(), T> {
        self.push(item)
    }
}

pub trait BufferTypes<Value: FixedSigned> {
    type Words: Default + Deref<Target = [Word<Value>]> + TryPush<Word<Value>> + core::fmt::Debug;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line<Value: FixedSigned, Types: BufferTypes<Value>> {
    pub words: Types::Words,
}

impl<Value: FixedSigned, Types: BufferTypes<Value>> Line<Value, Types> {
    fn new() -> Self {
        Self {
            words: Default::default(),
        }
    }

    pub fn command(&self) -> Option<&Word<Value>> {
        self.words.get(0)
    }

    pub fn arguments(&self) -> impl Iterator<Item = &Word<Value>> {
        self.words.iter().skip(1)
    }
}

impl<Value: FixedSigned, Types: BufferTypes<Value>> FromStr for Line<Value, Types> {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let Ok((rest, line)) = parser::line(s) else {
            return Err(Error::ParseError);
        };

        if !rest.is_empty() {
            return Err(Error::ParseError);
        }

        if line.overflow_count > 0 {
            return Err(Error::ArgumentOverflow(line.overflow_count));
        }

        Ok(line.line)
    }
}

#[cfg(test)]
mod tests;
