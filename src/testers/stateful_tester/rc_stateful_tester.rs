// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulTester` public type.

use std::{cell::RefCell, ops::Not};

use super::{Rc, StatefulTester};

type RcStatefulTesterFn = Rc<RefCell<dyn FnMut() -> bool>>;

/// A single-threaded shared stateful tester backed by `Rc<RefCell<_>>`.
///
/// Clones of an `RcStatefulTester` share the same mutable closure state.
pub struct RcStatefulTester {
    pub(super) function: RcStatefulTesterFn,
}

impl RcStatefulTester {
    /// Creates a new single-threaded shared stateful tester.
    #[inline]
    pub fn new<F>(mut source: F) -> Self
    where
        F: StatefulTester + 'static,
    {
        RcStatefulTester {
            function: Rc::new(RefCell::new(move || source.test())),
        }
    }

    /// Combines this tester with another stateful tester using logical AND.
    #[inline]
    pub fn and<T>(&self, mut next: T) -> RcStatefulTester
    where
        T: StatefulTester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcStatefulTester::new(move || {
            let matched = (self_fn.borrow_mut())();
            matched && next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical OR.
    #[inline]
    pub fn or<T>(&self, mut next: T) -> RcStatefulTester
    where
        T: StatefulTester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcStatefulTester::new(move || {
            let matched = (self_fn.borrow_mut())();
            matched || next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical NAND.
    #[inline]
    pub fn nand<T>(&self, mut next: T) -> RcStatefulTester
    where
        T: StatefulTester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcStatefulTester::new(move || {
            let matched = (self_fn.borrow_mut())();
            !(matched && next.test())
        })
    }

    /// Combines this tester with another stateful tester using logical XOR.
    #[inline]
    pub fn xor<T>(&self, mut next: T) -> RcStatefulTester
    where
        T: StatefulTester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcStatefulTester::new(move || {
            let matched = (self_fn.borrow_mut())();
            matched ^ next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical NOR.
    #[inline]
    pub fn nor<T>(&self, mut next: T) -> RcStatefulTester
    where
        T: StatefulTester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcStatefulTester::new(move || {
            let matched = (self_fn.borrow_mut())();
            !(matched || next.test())
        })
    }
}

impl Clone for RcStatefulTester {
    #[inline]
    fn clone(&self) -> Self {
        RcStatefulTester {
            function: Rc::clone(&self.function),
        }
    }
}

impl Not for RcStatefulTester {
    type Output = RcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let function = self.function;
        RcStatefulTester::new(move || !((function.borrow_mut())()))
    }
}

impl Not for &RcStatefulTester {
    type Output = RcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let function = Rc::clone(&self.function);
        RcStatefulTester::new(move || !((function.borrow_mut())()))
    }
}

impl StatefulTester for RcStatefulTester {
    #[inline]
    fn test(&mut self) -> bool {
        (self.function.borrow_mut())()
    }
}
