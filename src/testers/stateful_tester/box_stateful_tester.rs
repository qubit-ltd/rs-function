/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Defines the `BoxStatefulTester` public type.

use std::ops::Not;

use super::{
    StatefulTester,
    rc_stateful_tester::RcStatefulTester,
};

/// A single-ownership stateful tester backed by `Box<dyn FnMut() -> bool>`.
///
/// `BoxStatefulTester` is the owned wrapper for zero-argument tests that need
/// native `FnMut() -> bool` semantics. Calling [`StatefulTester::test`] may
/// mutate state captured by the wrapped closure.
pub struct BoxStatefulTester {
    pub(super) function: Box<dyn FnMut() -> bool>,
}

impl BoxStatefulTester {
    /// Creates a new boxed stateful tester.
    #[inline]
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() -> bool + 'static,
    {
        BoxStatefulTester { function: Box::new(f) }
    }

    /// Combines this tester with another stateful tester using logical AND.
    ///
    /// The second tester is evaluated only when the first tester returns
    /// `true`.
    #[inline]
    pub fn and<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self.test() && next.test())
    }

    /// Combines this tester with another stateful tester using logical OR.
    ///
    /// The second tester is evaluated only when the first tester returns
    /// `false`.
    #[inline]
    pub fn or<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self.test() || next.test())
    }

    /// Combines this tester with another stateful tester using logical NAND.
    #[inline]
    pub fn nand<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || !(self.test() && next.test()))
    }

    /// Combines this tester with another stateful tester using logical XOR.
    #[inline]
    pub fn xor<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self.test() ^ next.test())
    }

    /// Combines this tester with another stateful tester using logical NOR.
    #[inline]
    pub fn nor<T>(mut self, mut next: T) -> BoxStatefulTester
    where
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || !(self.test() || next.test()))
    }
}

impl Not for BoxStatefulTester {
    type Output = BoxStatefulTester;

    #[inline]
    fn not(mut self) -> Self::Output {
        BoxStatefulTester::new(move || !self.test())
    }
}

impl StatefulTester for BoxStatefulTester {
    #[inline]
    fn test(&mut self) -> bool {
        (self.function)()
    }

    #[inline]
    fn into_box(self) -> BoxStatefulTester {
        self
    }

    #[inline]
    fn into_rc(self) -> RcStatefulTester {
        let function = self.function;
        RcStatefulTester::new(function)
    }
}
