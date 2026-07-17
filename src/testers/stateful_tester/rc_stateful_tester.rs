// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulTester` public type.

use std::{
    cell::RefCell,
    fmt,
    ops::Not,
};

use {
    crate::{
        StatefulTester,
        internal::CallbackMetadata,
        macros::{
            impl_common_name_methods,
            impl_common_new_methods,
        },
    },
    std::rc::Rc,
};

/// The erased callback representation used by this implementation.
type RcStatefulTesterFn = Rc<RefCell<dyn FnMut() -> bool>>;

/// A single-threaded shared stateful tester backed by `Rc<RefCell<_>>`.
///
/// Clones of an `RcStatefulTester` share the same mutable closure state.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcStatefulTester {
    /// The wrapped callback implementation.
    pub(super) function: RcStatefulTesterFn,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: CallbackMetadata,
}

impl RcStatefulTester {
    impl_common_new_methods!(
        semantic_mut(StatefulTester + 'static),
        |source| move || source.test(),
        |function| Rc::new(RefCell::new(function)),
        "stateful tester"
    );

    impl_common_name_methods!("stateful tester");

    /// Combines this tester with another stateful tester using logical AND.
    #[inline]
    pub fn and<T>(&self, mut next: T) -> RcStatefulTester
    where
        T: StatefulTester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcStatefulTester::new(move || {
            let matched = {
                let mut function = self_fn.borrow_mut();
                function()
            };
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
            let matched = {
                let mut function = self_fn.borrow_mut();
                function()
            };
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
            let matched = {
                let mut function = self_fn.borrow_mut();
                function()
            };
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
            let matched = {
                let mut function = self_fn.borrow_mut();
                function()
            };
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
            let matched = {
                let mut function = self_fn.borrow_mut();
                function()
            };
            !(matched || next.test())
        })
    }
}

impl Clone for RcStatefulTester {
    #[inline]
    fn clone(&self) -> Self {
        RcStatefulTester {
            function: Rc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl Not for RcStatefulTester {
    type Output = RcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let function = self.function;
        RcStatefulTester::new_with_metadata(
            move || {
                let mut function = function.borrow_mut();
                !function()
            },
            metadata,
        )
    }
}

impl Not for &RcStatefulTester {
    type Output = RcStatefulTester;

    #[inline]
    fn not(self) -> Self::Output {
        let function = Rc::clone(&self.function);
        RcStatefulTester::new_with_metadata(
            move || {
                let mut function = function.borrow_mut();
                !function()
            },
            self.metadata.clone(),
        )
    }
}

impl StatefulTester for RcStatefulTester {
    #[inline]
    fn test(&mut self) -> bool {
        let mut function = self.function.borrow_mut();
        function()
    }
}

impl fmt::Debug for RcStatefulTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RcStatefulTester")
            .field("name", &self.metadata.name())
            .finish()
    }
}

impl fmt::Display for RcStatefulTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.metadata.name() {
            Some(name) => write!(formatter, "RcStatefulTester({name})"),
            None => formatter.write_str("RcStatefulTester"),
        }
    }
}
