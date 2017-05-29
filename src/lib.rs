//! Library for working with unused results.
//!
//! Assert properties of values in a method-chaining style.

// FIXME: errors printed are horrible because we don't have file/line/column without a macro :(

use std::fmt::Debug;

pub trait EqAssert: Eq + Debug {
    fn eq(&self, other: &Self) {
        debug_assert_eq!(self, other);
    }
}

pub trait BoolAssert {
    fn assert_true(&self);
    fn assert_false(&self);
}

impl BoolAssert for bool {
    fn assert_true(&self) {
        debug_assert!(self);
    }

    fn assert_false(&self) {
        debug_assert!(!self);
    }
}

pub trait ResultAssert {
    fn assert_ok(self);
    fn assert_err(self);
}

// TODO: could produce better assertion messages
impl <T, E> ResultAssert for Result<T, E> {
    fn assert_ok(self) {
        debug_assert!(self.is_ok());
    }

    fn assert_err(self) {
        debug_assert!(self.is_err());
    }
}

pub trait OptionAssert {
    fn assert_some(self);
    fn assert_none(self);
}

impl <T> OptionAssert for Option<T> {
    fn assert_some(self) {
        debug_assert!(self.is_some());
    }

    fn assert_none(self) {
        debug_assert!(self.is_none());
    }
}

pub trait Ignore: Sized {
    fn ignore(self) {
        drop(self);
    }
}

impl <T> Ignore for T where T: Sized {}
