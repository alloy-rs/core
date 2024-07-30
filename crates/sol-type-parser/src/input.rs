#![allow(missing_docs, missing_copy_implementations, missing_debug_implementations)]

// Recursion implementation modified from `toml`: https://github.com/toml-rs/toml/blob/a02cbf46cab4a8683e641efdba648a31498f7342/crates/toml_edit/src/parser/mod.rs#L99

use core::fmt;
use winnow::{
    error::{ContextError, FromExternalError},
    Parser,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CustomError {
    RecursionLimitExceeded,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CustomError {}

pub type Input<'a> = winnow::Stateful<&'a str, RecursionCheck>;

pub fn new_input(input: &str) -> Input<'_> {
    winnow::Stateful { input, state: Default::default() }
}

pub fn check_recursion<'a, O>(
    mut parser: impl Parser<Input<'a>, O, ContextError>,
) -> impl Parser<Input<'a>, O, ContextError> {
    move |input: &mut Input<'a>| {
        input.state.enter().map_err(|err| {
            winnow::error::ErrMode::from_external_error(input, winnow::error::ErrorKind::Eof, err)
                .cut()
        })?;
        let result = parser.parse_next(input);
        input.state.exit();
        result
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RecursionCheck {
    #[cfg(not(feature = "unbounded"))]
    current: usize,
}

#[cfg(not(feature = "unbounded"))]
const LIMIT: usize = 80;

impl RecursionCheck {
    pub fn check_depth(_depth: usize) -> Result<(), CustomError> {
        #[cfg(not(feature = "unbounded"))]
        if LIMIT <= _depth {
            return Err(CustomError::RecursionLimitExceeded);
        }

        Ok(())
    }

    fn enter(&mut self) -> Result<(), CustomError> {
        #[cfg(not(feature = "unbounded"))]
        {
            self.current += 1;
            if LIMIT <= self.current {
                return Err(CustomError::RecursionLimitExceeded);
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        #[cfg(not(feature = "unbounded"))]
        {
            self.current -= 1;
        }
    }
}
