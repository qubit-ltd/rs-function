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
//! `Callable` represents a one-time computation that returns
//! `Result<R, E>`. `Runnable` represents a one-time action that returns
//! `Result<(), E>`. Both abstractions are intentionally fallible and consume
//! `self`, making them suitable for deferred work, workflows, retry steps,
//! cleanup hooks, and executor task submission.
//!
//! # Author
//!
//! Haixing Hu

pub mod callable;
pub mod runnable;

pub use callable::{
    BoxCallable,
    Callable,
};
pub use runnable::{
    BoxRunnable,
    Runnable,
};
