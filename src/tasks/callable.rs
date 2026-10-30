// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Callable Types
//!
//! Provides fallible, reusable, zero-argument computations.
//!
//! A `Callable<R, E>` is equivalent to `FnMut() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `Runnable<E>` when the operation only needs
//! to report success or failure.
//!
//! The semantic trait is thread-neutral. `Box*` task wrappers are `Send` for
//! executor submission, while matching `LocalBox*` wrappers permit non-`Send`
//! captures for local execution.

mod box_callable;
pub use box_callable::BoxCallable;
mod local_box_callable;
pub use local_box_callable::LocalBoxCallable;
#[path = "callable/callable.rs"]
mod callable_trait;
pub use callable_trait::Callable;
#[cfg(feature = "rc")]
mod rc_callable;
#[cfg(feature = "rc")]
pub use rc_callable::RcCallable;
#[cfg(feature = "stateful")]
mod arc_callable;
#[cfg(feature = "stateful")]
pub use arc_callable::ArcCallable;
