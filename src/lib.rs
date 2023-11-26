#![cfg_attr(not(feature = "std"), no_std)]

use core::{ops::Deref, str::FromStr};

use fixed::traits::{FixedSigned, ToFixed};
use heapless::Vec;

pub(crate) mod parser;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    ArgumentOverflow(usize),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Line<Value: FixedSigned, Types: BufferTypes<Value>> {
    pub words: Types::Words,
}

impl<Value: FixedSigned, Types: BufferTypes<Value>> Line<Value, Types> {
    fn new() -> Self {
        Self {
            words: Default::default(),
        }
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
