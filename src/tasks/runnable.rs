// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Runnable Types
//!
//! Provides fallible, reusable, zero-argument actions.
//!
//! A `Runnable<E>` is equivalent to `FnMut() -> Result<(), E>`, but uses
//! task-oriented vocabulary. Use it when the operation's side effect matters
//! and only success or failure should be reported.
//!
//! The semantic trait is thread-neutral. `Box*` task wrappers are `Send` for
//! executor submission, while matching `LocalBox*` wrappers permit non-`Send`
//! captures for local execution.

#[path = "runnable/runnable.rs"]
mod runnable_trait;
pub use runnable_trait::Runnable;
mod box_runnable;
pub use box_runnable::BoxRunnable;
mod local_box_runnable;
pub use local_box_runnable::LocalBoxRunnable;
#[cfg(feature = "rc")]
mod rc_runnable;
#[cfg(feature = "rc")]
pub use rc_runnable::RcRunnable;
#[cfg(feature = "stateful")]
mod arc_runnable;
#[cfg(feature = "stateful")]
pub use arc_runnable::ArcRunnable;
