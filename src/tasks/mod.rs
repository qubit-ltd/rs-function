/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Task Function Module
//!
//! Provides zero-argument task-oriented functional abstractions.
//!
//! `Callable` represents a reusable computation that returns `Result<R, E>`.
//! `Runnable` represents a reusable action that returns `Result<(), E>`. Both
//! abstractions are intentionally fallible and support task submission in
//! executor-style workflows.
//!
//! One-time equivalents are also provided as `CallableOnce` and `RunnableOnce`
//! for move-only callable use cases.
//!
//! # Author
//!
//! Haixing Hu

pub mod callable;
pub mod callable_once;
pub mod runnable;
pub mod runnable_once;

pub use callable::{
    ArcCallable,
    BoxCallable,
    Callable,
    RcCallable,
};
pub use callable_once::{
    BoxCallableOnce,
    CallableOnce,
};
pub use runnable::{
    ArcRunnable,
    BoxRunnable,
    RcRunnable,
    Runnable,
};
pub use runnable_once::{
    BoxRunnableOnce,
    RunnableOnce,
};
