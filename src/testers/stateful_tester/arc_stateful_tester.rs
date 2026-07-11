// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulTester` public type.

use std::ops::Not;

use parking_lot::Mutex;

use super::{
    Arc,
    StatefulTester,
};

type ArcStatefulTesterFn = Arc<Mutex<dyn FnMut() -> bool + Send + 'static>>;

/// A thread-safe shared stateful tester backed by `Arc<Mutex<_>>`.
///
/// Clones of an `ArcStatefulTester` share the same mutable closure state and
/// can be moved across threads.
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared wrapper
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
pub struct ArcStatefulTester {
    pub(super) function: ArcStatefulTesterFn,
}

impl ArcStatefulTester {
    /// Creates a new thread-safe shared stateful tester.
    #[inline]
    pub fn new<F>(mut source: F) -> Self
    where
        F: StatefulTester + Send + 'static,
    {
        ArcStatefulTester {
            function: Arc::new(Mutex::new(move || source.test())),
        }
    }

    /// Combines this tester with another stateful tester using logical AND.
    #[inline]
    pub fn and<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = (self_fn.lock())();
            matched && next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical OR.
    #[inline]
    pub fn or<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = (self_fn.lock())();
            matched || next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical NAND.
    #[inline]
    pub fn nand<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = (self_fn.lock())();
            !(matched && next.test())
        })
    }

    /// Combines this tester with another stateful tester using logical XOR.
    #[inline]
    pub fn xor<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = (self_fn.lock())();
            matched ^ next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical NOR.
    #[inline]
    pub fn nor<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = (self_fn.lock())();
            !(matched || next.test())
        })
    }
}

impl Clone for ArcStatefulTester {
    #[inline]
    fn clone(&self) -> Self {
        ArcStatefulTester {
            function: Arc::clone(&self.function),
        }
    }
}

impl Not for ArcStatefulTester {
    type Output = ArcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let function = self.function;
        ArcStatefulTester::new(move || !((function.lock())()))
    }
}

impl Not for &ArcStatefulTester {
    type Output = ArcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let function = Arc::clone(&self.function);
        ArcStatefulTester::new(move || !((function.lock())()))
    }
}

impl StatefulTester for ArcStatefulTester {
    #[inline]
    fn test(&mut self) -> bool {
        (self.function.lock())()
    }
}
