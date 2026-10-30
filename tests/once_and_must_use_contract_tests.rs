// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::process::Output;

mod support;

fn diagnostics(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn assert_compile_failure(output: &Output, case_name: &str, expected: &str) {
    let diagnostics = diagnostics(output);
    assert!(
        !output.status.success(),
        "case {case_name:?} unexpectedly compiled\n{diagnostics}"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(expected),
        "case {case_name:?}: expected diagnostic containing {expected:?}\n{diagnostics}",
    );
}

#[test]
fn test_once_accepts_explicit_once_method_names() {
    let output = support::compile_consumer(
        &["once"],
        r#"
use qubit_function::CallableOnce;
use qubit_function::RunnableOnce;

fn main() {
    assert_eq!(CallableOnce::call_once(|| Ok::<i32, ()>(42)), Ok(42));
    assert_eq!(RunnableOnce::run_once(|| Ok::<(), ()>(())), Ok(()));
}
"#,
    );
    assert!(output.status.success(), "{}", diagnostics(&output));
}

#[test]
fn test_once_rejects_reusable_method_names() {
    let callable = support::compile_consumer(
        &["once"],
        r#"
use qubit_function::CallableOnce;
struct OnceCallable;
impl CallableOnce<i32, ()> for OnceCallable {
    fn call_once(self) -> Result<i32, ()> { Ok(42) }
}
fn main() { let _ = OnceCallable.call(); }
"#,
    );
    assert_compile_failure(
        &callable,
        "once callable rejects reusable method",
        "no method named `call`",
    );
    let runnable = support::compile_consumer(
        &["once"],
        r#"
use qubit_function::RunnableOnce;
struct OnceRunnable;
impl RunnableOnce<()> for OnceRunnable {
    fn run_once(self) -> Result<(), ()> { Ok(()) }
}
fn main() { let _ = OnceRunnable.run(); }
"#,
    );
    assert_compile_failure(
        &runnable,
        "once runnable rejects reusable method",
        "no method named `run`",
    );
}

#[test]
fn test_once_method_names_are_unambiguous_with_glob_import() {
    let output = support::compile_consumer(
        &["once"],
        r#"
use qubit_function::*;
fn main() {
    let mut reusable = || Ok::<i32, ()>(1);
    assert_eq!(reusable.call(), Ok(1));
    assert_eq!(reusable.call(), Ok(1));
    assert_eq!(BoxCallableOnce::new(|| Ok::<i32, ()>(2)).call_once(), Ok(2));
    assert_eq!(BoxRunnableOnce::new(|| Ok::<(), ()>(())).run_once(), Ok(()));
}
"#,
    );
    assert!(output.status.success(), "{}", diagnostics(&output));
}

#[test]
fn test_discarded_apply_result_triggers_its_must_use_contract() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::Function;
fn main() { (|value: &i32| *value + 1).apply(&1); }
"#,
    );
    assert_compile_failure(
        &output,
        "discarded apply result",
        "the computed callback result should be used",
    );
}

#[test]
fn test_discarded_test_result_triggers_its_must_use_contract() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::Tester;
fn main() { (|| true).test(); }
"#,
    );
    assert_compile_failure(&output, "discarded test result", "the test result should be used");
}

#[test]
fn test_discarded_get_result_triggers_its_must_use_contract() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::Supplier;
fn main() { (|| 42).get(); }
"#,
    );
    assert_compile_failure(&output, "discarded get result", "the supplied value should be used");
}

#[test]
fn test_discarded_compare_result_triggers_its_must_use_contract() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::Comparator;
fn main() { (|left: &i32, right: &i32| left.cmp(right)).compare(&1, &2); }
"#,
    );
    assert_compile_failure(
        &output,
        "discarded compare result",
        "the ordering result should be used",
    );
}

#[test]
fn test_discarded_into_fn_result_triggers_its_must_use_contract() {
    let output = support::compile_consumer(
        &[],
        r#"
#![deny(unused_must_use)]
use qubit_function::BoxComparator;
fn main() {
    BoxComparator::<i32>::new(|left: &i32, right: &i32| left.cmp(right)).into_fn();
}
"#,
    );
    assert_compile_failure(
        &output,
        "discarded into_fn result",
        "the returned comparator closure should be stored or invoked",
    );
}
