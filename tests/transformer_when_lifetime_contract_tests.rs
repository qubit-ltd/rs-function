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

fn assert_static_lifetime_rejection(output: &Output, case: &str) {
    let diagnostics = diagnostics(output);
    assert!(!output.status.success(), "{case} unexpectedly compiled");
    assert!(
        diagnostics.contains("must outlive") && diagnostics.contains("'static"),
        "{case} should be rejected because its input type is not 'static:\n{diagnostics}",
    );
}

#[test]
fn test_shared_when_chain_requires_static_input_type() {
    let output = support::compile_consumer(
        &["full"],
        r#"
use qubit_function::ArcTransformer;
use qubit_function::Transformer;

fn run<'a>(input: &'a str) -> usize {
    let source = ArcTransformer::<&'a str, usize>::new(|value: &'a str| value.len());
    let pipeline = source
        .when(|value: &&'a str| !value.is_empty())
        .or_else(|_: &'a str| 0);
    pipeline.apply(input)
}

fn main() {
    let owned = String::from("abc");
    assert_eq!(run(owned.as_str()), 3);
}
"#,
    );
    assert_static_lifetime_rejection(&output, "shared single-input when/or_else chain");
}

#[test]
fn test_box_bi_when_chain_requires_static_input_type() {
    let output = support::compile_consumer(
        &["full"],
        r#"
use qubit_function::BiTransformer;
use qubit_function::BoxBiTransformer;

fn run<'a>(input: &'a str) -> usize {
    let source = BoxBiTransformer::<&'a str, usize, usize>::new(
        |value: &'a str, offset: usize| value.len() + offset,
    );
    let pipeline = source
        .when(|value: &&'a str, offset: &usize| !value.is_empty() && *offset > 0)
        .or_else(|_: &'a str, offset: usize| offset);
    pipeline.apply(input, 1)
}

fn main() {
    let owned = String::from("abc");
    assert_eq!(run(owned.as_str()), 4);
}
"#,
    );
    assert_static_lifetime_rejection(&output, "Box bi-transformer when/or_else chain");
}

#[test]
fn test_when_callbacks_still_reject_stack_borrows() {
    let output = support::compile_consumer(
        &["full"],
        r#"
use qubit_function::ArcTransformer;

fn main() {
    let owned = String::from("borrowed");
    let _source = ArcTransformer::new(|_: i32| owned.len());
}
"#,
    );
    let diagnostics = diagnostics(&output);
    assert!(
        !output.status.success(),
        "stack-capturing callback unexpectedly compiled"
    );
    assert!(
        diagnostics.contains("owned") && diagnostics.contains("'static"),
        "expected rejection for a callback capturing stack data:\n{diagnostics}",
    );
}

#[test]
fn test_direct_transformer_call_accepts_borrowed_input_type() {
    use qubit_function::ArcTransformer;
    use qubit_function::Transformer;

    fn run<'a>(input: &'a str) -> usize {
        ArcTransformer::<&'a str, usize>::new(|value: &'a str| value.len()).apply(input)
    }

    let owned = String::from("abc");
    assert_eq!(run(owned.as_str()), 3);
}
