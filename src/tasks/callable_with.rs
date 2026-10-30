// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # CallableWith Types
//!
//! Provides fallible, reusable computations that operate on a mutable input.
//!
//! A `CallableWith<T, R, E>` is equivalent to
//! `FnMut(&mut T) -> Result<R, E>`, but uses task-oriented vocabulary. Use it
//! when the operation needs access to protected or caller-provided state and
//! returns a success value.
//!
//! The semantic trait is thread-neutral. `Box*` task wrappers are `Send` for
//! executor submission, while matching `LocalBox*` wrappers permit non-`Send`
//! captures for local execution.

mod box_callable_with;
pub use box_callable_with::BoxCallableWith;
#[path = "callable_with/callable_with.rs"]
mod callable_with_trait;
pub use callable_with_trait::CallableWith;
mod local_box_callable_with;
pub use local_box_callable_with::LocalBoxCallableWith;
#[cfg(feature = "rc")]
mod rc_callable_with;
#[cfg(feature = "rc")]
pub use rc_callable_with::RcCallableWith;
#[cfg(feature = "stateful")]
mod arc_callable_with;
#[cfg(feature = "stateful")]
pub use arc_callable_with::ArcCallableWith;
