extern crate thiserror;

use crate::parser;
use crate::Location;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IDLError {
    #[error("Internal error: {0}")]
    InternalError(&'static str),
    #[error("Incorrect usage: {0}")]
    UsageError(String),
    #[error("{0}")]
    ParseError(#[from] parser::ParseError),
    #[error("{0}")]
    ValidationError(#[from] ValidationError),
    #[error("{0}")]
    Io(#[from] io::Error),
}

/*
impl From<io::Error> for IDLError {
    fn from(e: io::Error) -> Self {
        IDLError::Io(e)
    }
}

impl From<parser::ParseError> for IDLError {
    fn from(e: parser::ParseError) -> Self {
        IDLError::ParseError(e)
    }
}

impl From<ValidationError> for IDLError {
    fn from(e: ValidationError) -> Self {
        IDLError::ValidationError(e)
    }
}
*/

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum ValidationError {
    #[error("Redefinition of name `{name}`")]
    NameAlreadyExists {
        name: String,
        at_location: Location,
        previous_location: Location,
    },
    #[error("Use of unknown name `{name}`")]
    NameNotFound {
        name: String,
        use_location: Location,
    },
    #[error("Empty definition for `{name}`")]
    Empty { name: String, location: Location },
    #[error("Infinite definition for `{name}`")]
    Infinite { name: String, location: Location },
    #[error("Syntax error: expected {expected}")]
    Syntax {
        expected: &'static str,
        location: Location,
    },
    #[error("Name `{name}` bound to another sort")]
    NameSortError {
        name: String,
        use_location: Location,
        bound_location: Location,
    },
    #[error("Name `{name}` already bound")]
    BindingNameAlreadyBound {
        name: String,
        at_location: Location,
        bound_location: Location,
    },
    #[error("Binding type error: expected {expected}")]
    BindingTypeError {
        expected: &'static str,
        location: Location,
    },
}
