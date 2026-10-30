// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![allow(
    dead_code,
    unused_imports,
    reason = "split contract fixtures share support definitions"
)]
// behavior.

use std::fmt::Debug;
use std::fmt::Display;
use std::process::Output;

mod support;

#[cfg(feature = "stateful")]
use qubit_function::ArcCallable;
#[cfg(feature = "stateful")]
use qubit_function::ArcCallableWith;
#[cfg(feature = "stateful")]
use qubit_function::ArcRunnableWith;
use qubit_function::BoxCallable;
use qubit_function::BoxCallableWith;
use qubit_function::BoxRunnableWith;
#[cfg(feature = "rc")]
use qubit_function::RcCallable;
#[cfg(feature = "rc")]
use qubit_function::RcCallableWith;
#[cfg(feature = "rc")]
use qubit_function::RcRunnableWith;

/// Asserts at compile time that T supports both formatting traits.
fn assert_debug_and_display<T: Debug + Display>() {}

/// Formats Cargo diagnostics for an assertion failure.
fn cargo_diagnostics(output: &Output) -> String {
    format!(
        "status: {}
stdout:
{}
stderr:
{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn assert_case_diagnostic_contains(output: &Output, case_name: &str, expected: &str) {
    let diagnostics = cargo_diagnostics(output);
    assert!(
        diagnostics.contains(expected),
        "case {case_name:?}: expected diagnostic containing {expected:?}\n{diagnostics}"
    );
}

/// Asserts that compilation fails for the intended API-contract reason.
fn assert_compile_failure(output: &Output, case_name: &str, expected_diagnostic: &str) {
    let diagnostics = cargo_diagnostics(output);
    assert!(
        !output.status.success(),
        "case {case_name:?} unexpectedly compiled\n{diagnostics}"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(expected_diagnostic),
        "case {case_name:?}: expected diagnostic containing {expected_diagnostic:?}
{diagnostics}",
    );
}

#[test]
fn test_discarded_callback_wrappers_trigger_unused_must_use() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::BoxCallable;
fn main() { BoxCallable::<i32, ()>::new(|| Ok(1)); }
"#,
    );
    assert_compile_failure(
        &output,
        "discarded BoxCallable",
        "callback wrappers do nothing unless stored or invoked",
    );
    assert_case_diagnostic_contains(&output, "discarded BoxCallable", "BoxCallable");
}

#[test]
fn test_discarded_local_box_callable_triggers_unused_must_use() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::LocalBoxCallable;
fn main() { LocalBoxCallable::<i32, ()>::new(|| Ok(1)); }
"#,
    );
    assert_compile_failure(
        &output,
        "discarded LocalBoxCallable",
        "callback wrappers do nothing unless stored or invoked",
    );
    assert_case_diagnostic_contains(&output, "discarded LocalBoxCallable", "LocalBoxCallable");
}

#[test]
fn test_discarded_arc_function_triggers_unused_must_use() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::ArcFunction;
fn main() { ArcFunction::<i32, i32>::new(|value: &i32| *value + 1); }
"#,
    );
    assert_compile_failure(
        &output,
        "discarded ArcFunction",
        "callback wrappers do nothing unless stored or invoked",
    );
    assert_case_diagnostic_contains(&output, "discarded ArcFunction", "ArcFunction");
}

#[test]
fn test_discarded_rc_function_triggers_unused_must_use() {
    let output = support::compile_consumer(
        &["rc"],
        r#"
#![deny(unused_must_use)]
use qubit_function::RcFunction;
fn main() { RcFunction::<i32, i32>::new(|value: &i32| *value + 1); }
"#,
    );
    assert_compile_failure(
        &output,
        "discarded RcFunction",
        "callback wrappers do nothing unless stored or invoked",
    );
    assert_case_diagnostic_contains(&output, "discarded RcFunction", "RcFunction");
}

#[test]
fn test_stored_callback_wrappers_satisfy_unused_must_use() {
    let output = support::compile_consumer(
        &["full"],
        r#"
#![deny(unused_must_use)]

use qubit_function::ArcFunction;
use qubit_function::BoxCallable;
use qubit_function::LocalBoxCallable;
use qubit_function::RcFunction;

fn main() {
    let _boxed = BoxCallable::<i32, ()>::new(|| Ok(1));
    let _local = LocalBoxCallable::<i32, ()>::new(|| Ok(1));
    let _shared = ArcFunction::<i32, i32>::new(|value: &i32| *value + 1);
    let _local_shared = RcFunction::<i32, i32>::new(|value: &i32| *value + 1);
}
"#,
    );

    assert!(output.status.success(), "{}", cargo_diagnostics(&output));
}
