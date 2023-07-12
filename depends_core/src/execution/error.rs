use std::cell::{BorrowError, BorrowMutError};

use thiserror::Error;

pub type ResolveResult<T> = Result<T, ResolveError>;

/// Any error that can occur when borrowing a [RefCell](std::cell::RefCell)
/// inside a node.
#[derive(Debug, Error)]
pub enum AnyBorrowError {
    /// Tried to borrow while a write-reference was held.
    #[error("borrow error")]
    BorrowError(#[from] BorrowError),
    /// Tried to borrow mutably while a read-reference was held.
    #[error("borrow mut error")]
    BorrowMutError(#[from] BorrowMutError),
}

// TODO: add a custom error type <E> for early exit.
/// Any error that can occur when resolving a node.
#[derive(Debug, Error)]
pub enum ResolveError {
    /// Either a borrow or borrow_mut error occurred when resolving a node.
    /// This either means there's a cyclic dependency or a read-reference to
    /// a previous result is being held.
    #[error("{0}")]
    BorrowError(#[from] AnyBorrowError),
}

impl From<BorrowError> for ResolveError {
    fn from(err: BorrowError) -> Self {
        Self::BorrowError(AnyBorrowError::BorrowError(err))
    }
}

impl From<BorrowMutError> for ResolveError {
    fn from(err: BorrowMutError) -> Self {
        Self::BorrowError(AnyBorrowError::BorrowMutError(err))
    }
}
