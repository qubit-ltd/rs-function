// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Task Function Module
//!
//! Provides zero-argument task-oriented functional abstractions.
//!
//! `Callable` represents a reusable computation that returns `Result<R, E>`.
//! `Runnable` represents a reusable action that returns `Result<(), E>`. Both
//! abstractions are intentionally fallible and support task submission in
//! executor-style workflows.
//! `CallableWith` and `RunnableWith` are their mutable-input counterparts for
//! executor APIs that pass protected state into the task.
//!
//! One-time equivalents are also provided as `CallableOnce` and `RunnableOnce`
//! for move-only callable use cases.
//!
//! Semantic task traits remain thread-neutral. Choose `Box*` for movable
//! executor tasks, `LocalBox*` for local callbacks with non-`Send` captures,
//! and `Arc*` for shared synchronized tasks.

pub mod callable;
#[cfg(feature = "once")]
pub mod callable_once;
pub mod callable_with;
pub mod runnable;
#[cfg(feature = "once")]
pub mod runnable_once;
pub mod runnable_with;

#[cfg(feature = "stateful")]
pub use callable::ArcCallable;
#[cfg(feature = "rc")]
pub use callable::RcCallable;
pub use callable::{
    BoxCallable,
    Callable,
    LocalBoxCallable,
};
#[cfg(feature = "once")]
pub use callable_once::{
    BoxCallableOnce,
    CallableOnce,
    LocalBoxCallableOnce,
};
#[cfg(feature = "stateful")]
pub use callable_with::ArcCallableWith;
#[cfg(feature = "rc")]
pub use callable_with::RcCallableWith;
pub use callable_with::{
    BoxCallableWith,
    CallableWith,
    LocalBoxCallableWith,
};
#[cfg(feature = "stateful")]
pub use runnable::ArcRunnable;
#[cfg(feature = "rc")]
pub use runnable::RcRunnable;
pub use runnable::{
    BoxRunnable,
    LocalBoxRunnable,
    Runnable,
};
#[cfg(feature = "once")]
pub use runnable_once::{
    BoxRunnableOnce,
    LocalBoxRunnableOnce,
    RunnableOnce,
};
#[cfg(feature = "stateful")]
pub use runnable_with::ArcRunnableWith;
#[cfg(feature = "rc")]
pub use runnable_with::RcRunnableWith;
pub use runnable_with::{
    BoxRunnableWith,
    LocalBoxRunnableWith,
    RunnableWith,
};
