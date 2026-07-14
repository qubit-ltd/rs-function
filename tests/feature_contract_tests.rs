// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::{
    fmt::{
        Debug,
        Display,
    },
    fs,
    path::PathBuf,
    process::{
        Command,
        Output,
    },
    sync::atomic::{
        AtomicUsize,
        Ordering,
    },
};

#[cfg(feature = "stateful")]
use qubit_function::{
    ArcCallable,
    ArcCallableWith,
    ArcRunnableWith,
};
use qubit_function::{
    BoxCallable,
    BoxCallableWith,
    BoxRunnableWith,
};
#[cfg(feature = "rc")]
use qubit_function::{
    RcCallable,
    RcCallableWith,
    RcRunnableWith,
};

static NEXT_PROJECT_ID: AtomicUsize = AtomicUsize::new(0);

/// Asserts at compile time that T supports both formatting traits.
fn assert_debug_and_display<T: Debug + Display>() {}

/// Compiles a temporary consumer crate against this checkout.
///
/// The features argument selects path-dependency features, and source becomes
/// the fixture crate's main.rs. The returned output contains Cargo's status and
/// diagnostics.
fn compile_consumer(features: &[&str], source: &str) -> Output {
    let project_id = NEXT_PROJECT_ID.fetch_add(1, Ordering::Relaxed);
    let project_root = std::env::temp_dir().join(format!(
        "qubit-function-feature-contract-{}-{project_id}",
        std::process::id(),
    ));
    let source_root = project_root.join("src");
    fs::create_dir_all(&source_root)
        .expect("temporary consumer source directory should be created");

    let feature_list = features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let feature_clause = if feature_list.is_empty() {
        String::new()
    } else {
        format!(", features = [{feature_list}]")
    };
    let dependency_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = format!(
        "[package]\n\
         name = \"feature-contract-consumer\"\n\
         version = \"0.0.0\"\n\
         edition = \"2024\"\n\n\
         [dependencies]\n\
         qubit-function = {{ path = \"{}\", default-features = false{} }}\n\n\
         [workspace]\n",
        dependency_path.display(),
        feature_clause,
    );
    fs::write(project_root.join("Cargo.toml"), manifest)
        .expect("temporary consumer manifest should be written");
    fs::write(source_root.join("main.rs"), source)
        .expect("temporary consumer source should be written");

    let output = Command::new("cargo")
        .args(["+1.94.0", "check", "--offline", "--quiet", "--target-dir"])
        .arg(project_root.join("target"))
        .current_dir(&project_root)
        .output()
        .expect("temporary consumer should invoke Cargo");

    fs::remove_dir_all(&project_root)
        .expect("temporary consumer directory should be removed");
    output
}

/// Formats Cargo diagnostics for an assertion failure.
fn cargo_diagnostics(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

/// Asserts that compilation fails for the intended API-contract reason.
fn assert_compile_failure(output: &Output, expected_diagnostic: &str) {
    let diagnostics = cargo_diagnostics(output);
    assert!(!output.status.success(), "{diagnostics}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(expected_diagnostic),
        "expected diagnostic containing {expected_diagnostic:?}\n{diagnostics}",
    );
}

#[test]
fn test_baseline_task_wrappers_implement_debug_and_display() {
    assert_debug_and_display::<BoxCallable<(), ()>>();
    assert_debug_and_display::<BoxCallableWith<(), (), ()>>();
    assert_debug_and_display::<BoxRunnableWith<(), ()>>();
}

#[cfg(feature = "rc")]
#[test]
fn test_rc_task_wrappers_implement_debug_and_display() {
    assert_debug_and_display::<RcCallable<(), ()>>();
    assert_debug_and_display::<RcCallableWith<(), (), ()>>();
    assert_debug_and_display::<RcRunnableWith<(), ()>>();
}

#[cfg(feature = "stateful")]
#[test]
fn test_arc_task_wrappers_implement_debug_and_display() {
    assert_debug_and_display::<ArcCallable<(), ()>>();
    assert_debug_and_display::<ArcCallableWith<(), (), ()>>();
    assert_debug_and_display::<ArcRunnableWith<(), ()>>();
}

#[test]
fn test_without_combinators_rejects_box_consumer_when() {
    let output = compile_consumer(
        &[],
        r#"
use qubit_function::BoxConsumer;

fn main() {
    let consumer = BoxConsumer::new(|_: &i32| {});
    let _conditional = consumer.when(|value: &i32| *value > 0);
}
"#,
    );

    assert_compile_failure(&output, "no method named `when`");
}

#[test]
fn test_without_combinators_rejects_deep_tester_ops_path() {
    let output = compile_consumer(
        &[],
        r#"
use qubit_function::testers::tester::fn_tester_ops::FnTesterOps;

fn main() {
    let _tester = (|| true).and(|| true);
}
"#,
    );

    assert_compile_failure(&output, "fn_tester_ops");
}

#[test]
fn test_without_combinators_rejects_box_runnable_then_callable() {
    let output = compile_consumer(
        &[],
        r#"
use qubit_function::BoxRunnable;

fn main() {
    let runnable = BoxRunnable::new(|| Ok::<(), ()>(()));
    let _callable = runnable.then_callable(|| Ok::<i32, ()>(42));
}
"#,
    );

    assert_compile_failure(&output, "no method named `then_callable`");
}

#[test]
fn test_without_combinators_rejects_box_runnable_with_then_callable() {
    let output = compile_consumer(
        &[],
        r#"
use qubit_function::BoxRunnableWith;

fn main() {
    let runnable = BoxRunnableWith::new(|_: &mut i32| Ok::<(), ()>(()));
    let _callable =
        runnable.then_callable_with(|value: &mut i32| Ok::<i32, ()>(*value));
}
"#,
    );

    assert_compile_failure(&output, "no method named `then_callable_with`");
}

#[test]
fn test_without_combinators_rejects_box_runnable_once_then_callable() {
    let output = compile_consumer(
        &["once"],
        r#"
use qubit_function::BoxRunnableOnce;

fn main() {
    let runnable = BoxRunnableOnce::new(|| Ok::<(), ()>(()));
    let _callable = runnable.then_callable(|| Ok::<i32, ()>(42));
}
"#,
    );

    assert_compile_failure(&output, "no method named `then_callable`");
}

#[test]
fn test_without_combinators_rejects_local_box_runnable_once_then_callable() {
    let output = compile_consumer(
        &["once"],
        r#"
use qubit_function::LocalBoxRunnableOnce;

fn main() {
    let runnable = LocalBoxRunnableOnce::new(|| Ok::<(), ()>(()));
    let _callable = runnable.then_callable(|| Ok::<i32, ()>(42));
}
"#,
    );

    assert_compile_failure(&output, "no method named `then_callable`");
}

#[test]
fn test_with_combinators_accepts_public_extension_apis() {
    let output = compile_consumer(
        &["combinators"],
        r#"
use qubit_function::{BoxConsumer, FnTesterOps};

fn main() {
    let consumer = BoxConsumer::new(|_: &i32| {});
    let _conditional = consumer.when(|value: &i32| *value > 0);
    let _tester = (|| true).and(|| true);
}
"#,
    );

    assert!(output.status.success(), "{}", cargo_diagnostics(&output));
}

#[test]
fn test_with_combinators_accepts_task_chaining_apis() {
    let output = compile_consumer(
        &["once", "combinators"],
        r#"
use qubit_function::{
    BoxRunnable,
    BoxRunnableOnce,
    BoxRunnableWith,
    LocalBoxRunnableOnce,
};

fn main() {
    let runnable = BoxRunnable::new(|| Ok::<(), ()>(()));
    let _callable = runnable.then_callable(|| Ok::<i32, ()>(42));

    let runnable = BoxRunnableWith::new(|_: &mut i32| Ok::<(), ()>(()));
    let _callable =
        runnable.then_callable_with(|value: &mut i32| Ok::<i32, ()>(*value));

    let runnable = BoxRunnableOnce::new(|| Ok::<(), ()>(()));
    let _callable = runnable.then_callable(|| Ok::<i32, ()>(42));

    let runnable = LocalBoxRunnableOnce::new(|| Ok::<(), ()>(()));
    let _callable = runnable.then_callable(|| Ok::<i32, ()>(42));
}
"#,
    );

    assert!(output.status.success(), "{}", cargo_diagnostics(&output));
}
