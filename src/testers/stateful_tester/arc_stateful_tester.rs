// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulTester` public type.

use std::fmt;
use std::ops::Not;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::StatefulTester;
use crate::internal::CallbackMetadata;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;

/// The erased callback representation used by this implementation.
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
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcStatefulTester {
    /// The wrapped callback implementation.
    pub(super) function: ArcStatefulTesterFn,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: CallbackMetadata,
}

impl ArcStatefulTester {
    impl_common_new_methods!(
        semantic_mut(StatefulTester + Send + 'static),
        |source| move || source.test(),
        |function| Arc::new(Mutex::new(function)),
        "stateful tester"
    );

    impl_common_name_methods!("stateful tester");

    /// Combines this tester with another stateful tester using logical AND.
    ///
    /// The second tester is evaluated only when this tester returns `true`.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester evaluated after this tester succeeds.
    ///
    /// # Returns
    ///
    /// A thread-safe tester that evaluates the logical AND of both testers.
    #[inline]
    pub fn and<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = {
                let mut function = self_fn.lock();
                function()
            };
            matched && next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical OR.
    ///
    /// The second tester is evaluated only when this tester returns `false`.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester evaluated after this tester fails.
    ///
    /// # Returns
    ///
    /// A thread-safe tester that evaluates the logical OR of both testers.
    #[inline]
    pub fn or<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = {
                let mut function = self_fn.lock();
                function()
            };
            matched || next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical NAND.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester combined with this tester.
    ///
    /// # Returns
    ///
    /// A thread-safe tester that negates the logical AND of both testers.
    #[inline]
    pub fn nand<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = {
                let mut function = self_fn.lock();
                function()
            };
            !(matched && next.test())
        })
    }

    /// Combines this tester with another stateful tester using logical XOR.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester combined with this tester.
    ///
    /// # Returns
    ///
    /// A thread-safe tester that evaluates whether exactly one tester succeeds.
    #[inline]
    pub fn xor<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = {
                let mut function = self_fn.lock();
                function()
            };
            matched ^ next.test()
        })
    }

    /// Combines this tester with another stateful tester using logical NOR.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester combined with this tester.
    ///
    /// # Returns
    ///
    /// A thread-safe tester that negates the logical OR of both testers.
    #[inline]
    pub fn nor<T>(&self, mut next: T) -> ArcStatefulTester
    where
        T: StatefulTester + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcStatefulTester::new(move || {
            let matched = {
                let mut function = self_fn.lock();
                function()
            };
            !(matched || next.test())
        })
    }
}

impl Clone for ArcStatefulTester {
    #[inline]
    fn clone(&self) -> Self {
        ArcStatefulTester {
            function: Arc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl Not for ArcStatefulTester {
    type Output = ArcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let function = self.function;
        ArcStatefulTester::new_with_metadata(
            move || {
                let mut function = function.lock();
                !function()
            },
            metadata,
        )
    }
}

impl Not for &ArcStatefulTester {
    type Output = ArcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let function = Arc::clone(&self.function);
        ArcStatefulTester::new_with_metadata(
            move || {
                let mut function = function.lock();
                !function()
            },
            self.metadata.clone(),
        )
    }
}

impl StatefulTester for ArcStatefulTester {
    #[inline]
    fn test(&mut self) -> bool {
        let mut function = self.function.lock();
        function()
    }
}

impl fmt::Debug for ArcStatefulTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ArcStatefulTester")
            .field("name", &self.metadata.name())
            .finish()
    }
}

impl fmt::Display for ArcStatefulTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.metadata.name() {
            Some(name) => write!(formatter, "ArcStatefulTester({name})"),
            None => formatter.write_str("ArcStatefulTester"),
        }
    }
}
