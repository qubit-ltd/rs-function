// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Runnable Once Types
//!
//! Provides fallible, one-time, zero-argument actions.
//!
//! A `RunnableOnce<E>` is equivalent to `FnOnce() -> Result<(), E>`, but uses
//! task-oriented vocabulary. Use it when the operation's side effect matters
//! and only success or failure should be reported.
//!
//! The semantic trait is thread-neutral. `Box*` task wrappers are `Send` for
//! executor submission, while matching `LocalBox*` wrappers permit non-`Send`
//! captures for local execution.

mod box_runnable_once;
pub use box_runnable_once::BoxRunnableOnce;
mod local_box_runnable_once;
pub use local_box_runnable_once::LocalBoxRunnableOnce;
#[path = "runnable_once/runnable_once.rs"]
mod runnable_once_trait;
pub use runnable_once_trait::RunnableOnce;
