use std::{
    borrow::Cow,
    cell::{BorrowError, BorrowMutError},
};

use thiserror::Error;

pub type ResolveResult<T> = Result<T, ResolveError>;

/// Any error that can occur when resolving a node.
///
/// We may be able to introduce a generic type parameter `<E>` for the custom
/// error, but this would require some invasive changes to the `Resolve`
/// trait to maintain flexibility. For instance, we'd want to assert that
/// a caller may only `resolve<E>` a node if it provides an `E` where
/// _all_ inner nodes can be `Into<E>`. This is a non-trivial constraint (if
/// it's even possible).
#[derive(Debug, Error)]
pub enum ResolveError {
    /// Either a borrow or borrow_mut error occurred when resolving a node.
    /// This either means there's a cyclic dependency or a read-reference to
    /// a previous result is being held.
    #[error("{0}")]
    BorrowError(#[from] AnyBorrowError),
    /// A custom Error. This is just a string detailing the error. Use this
    /// if you want a node to abort a resolution early.
    #[error("early exit: {0}")]
    EarlyExit(#[from] EarlyExit),
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

/// Abort the resolution of a graph immediately and return this custom error.
///
/// We might be able to make this a generic type eventually, but for now it's
/// just a string describing the error.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct EarlyExit(Cow<'static, str>);

impl EarlyExit {
    pub fn new<S: Into<Cow<'static, str>>>(err: S) -> Self {
        Self(err.into())
    }
}

impl From<String> for EarlyExit {
    fn from(err: String) -> Self {
        Self(Cow::Owned(err))
    }
}

impl From<&'static str> for EarlyExit {
    fn from(err: &'static str) -> Self {
        Self(Cow::Borrowed(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_result() {
        let res = Ok::<_, ResolveError>(());
        assert_eq!(format!("{:?}", res), "Ok(())");
    }

    #[test]
    fn test_resolve_error() {
        let refcell = std::cell::RefCell::new(());
        {
            let _a = refcell.borrow();
            if let Err(e) = refcell.try_borrow_mut() {
                let err: AnyBorrowError = e.into();
                let err: ResolveError = err.into();
                assert!(matches!(err, ResolveError::BorrowError(_)));
                assert_eq!(
                    format!("{:?}", err),
                    "BorrowError(BorrowMutError(BorrowMutError))"
                );
                assert_eq!(format!("{}", err), "borrow mut error");
            } else {
                panic!("expected borrow mut error");
            };
            if let Err(e) = refcell.try_borrow_mut() {
                let err: ResolveError = e.into();
                assert!(matches!(err, ResolveError::BorrowError(_)));
                assert_eq!(
                    format!("{:?}", err),
                    "BorrowError(BorrowMutError(BorrowMutError))"
                );
                assert_eq!(format!("{}", err), "borrow mut error");
            } else {
                panic!("expected borrow mut error");
            };
        }
        {
            let _a = refcell.borrow_mut();
            if let Err(e) = refcell.try_borrow() {
                let err: AnyBorrowError = e.into();
                let err: ResolveError = err.into();
                assert!(matches!(err, ResolveError::BorrowError(_)));
                assert_eq!(
                    format!("{:?}", err),
                    "BorrowError(BorrowError(BorrowError))"
                );
                assert_eq!(format!("{}", err), "borrow error");
            } else {
                panic!("expected borrow error");
            };
            if let Err(e) = refcell.try_borrow() {
                let err: ResolveError = e.into();
                assert!(matches!(err, ResolveError::BorrowError(_)));
                assert_eq!(
                    format!("{:?}", err),
                    "BorrowError(BorrowError(BorrowError))"
                );
                assert_eq!(format!("{}", err), "borrow error");
            } else {
                panic!("expected borrow error");
            };
        }
        let err = EarlyExit::new("test");
        let err: ResolveError = err.into();
        assert!(matches!(err, ResolveError::EarlyExit(_)));
        assert_eq!(format!("{:?}", err), r#"EarlyExit(EarlyExit("test"))"#);
        assert_eq!(format!("{}", err), "early exit: test");
    }
    #[test]
    fn test_any_borrow_error() {
        let refcell = std::cell::RefCell::new(());
        {
            let _a = refcell.borrow();
            if let Err(e) = refcell.try_borrow_mut() {
                let err: AnyBorrowError = e.into();
                assert!(matches!(err, AnyBorrowError::BorrowMutError(_)));
                assert_eq!(format!("{:?}", err), "BorrowMutError(BorrowMutError)");
                assert_eq!(format!("{}", err), "borrow mut error");
            };
        }
        {
            let _a = refcell.borrow_mut();
            if let Err(e) = refcell.try_borrow() {
                let err: AnyBorrowError = e.into();
                assert!(matches!(err, AnyBorrowError::BorrowError(_)));
                assert_eq!(format!("{:?}", err), "BorrowError(BorrowError)");
                assert_eq!(format!("{}", err), "borrow error");
            };
        }
    }

    #[test]
    fn test_early_exit() {
        let err = EarlyExit::new("test");
        assert_eq!(err.0, Cow::Borrowed("test"));
        let err = EarlyExit::from("exit");
        assert_eq!(err.0, Cow::Borrowed("exit"));
        let err = EarlyExit::from(String::from("see ya"));
        assert_eq!(err.0, Cow::<'static, str>::Owned(String::from("see ya")));
        assert_eq!(format!("{}", err), "see ya");
        assert_eq!(format!("{:?}", err), r#"EarlyExit("see ya")"#);
    }
}
