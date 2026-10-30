// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Callable Once Types
//!
//! Provides fallible, one-time, zero-argument computations.
//!
//! A `CallableOnce<R, E>` is equivalent to `FnOnce() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `RunnableOnce<E>` when the operation only
//! needs to report success or failure.
//!
//! The semantic trait is thread-neutral. `Box*` task wrappers are `Send` for
//! executor submission, while matching `LocalBox*` wrappers permit non-`Send`
//! captures for local execution.

mod box_callable_once;
pub use box_callable_once::BoxCallableOnce;
#[path = "callable_once/callable_once.rs"]
mod callable_once_trait;
pub use callable_once_trait::CallableOnce;
mod local_box_callable_once;
pub use local_box_callable_once::LocalBoxCallableOnce;
