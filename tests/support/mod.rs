// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::fs;
use std::process::Command;
use std::process::Output;
use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use tempfile::TempDir;

const SUPPORTED_FEATURES: [&str; 4] = ["full", "once", "rc", "stateful"];
const FIXTURE_MANIFEST: &str = include_str!("fixture/Cargo.toml");
const FIXTURE_LOCK: &str = include_str!("fixture/Cargo.lock");
static TARGET_ROOT: OnceLock<TempDir> = OnceLock::new();
static NEXT_BIN_ID: AtomicUsize = AtomicUsize::new(0);

/// Compiles an isolated consumer source against this checkout.
pub(super) fn compile_consumer(features: &[&str], source: &str) -> Output {
    let normalized_features = normalize_features(features);
    let source_dir = tempfile::Builder::new()
        .prefix("qubit-function-contract-source-")
        .tempdir()
        .expect("compile-contract infrastructure: temporary source directory creation failed");
    let project_root = source_dir.path().to_owned();

    // The package name and lock stay fixed, but a distinct bin name gives
    // Cargo each concurrent source a separate root-target fingerprint while
    // retaining the shared dependency cache for this feature group.
    let bin_name = format!(
        "qubit-function-contract-fixture-{}",
        NEXT_BIN_ID.fetch_add(1, Ordering::Relaxed),
    );
    let bin_name_pattern = "[[bin]]\nname = \"qubit-function-contract-fixture\"";
    assert_eq!(FIXTURE_MANIFEST.matches(bin_name_pattern).count(), 1);
    assert_eq!(FIXTURE_MANIFEST.matches("path = \"../../..\"").count(), 1);
    let manifest = FIXTURE_MANIFEST
        .replace(bin_name_pattern, &format!("[[bin]]\nname = \"{bin_name}\""))
        .replace(
            "path = \"../../..\"",
            &format!("path = \"{}\"", toml_basic_string(env!("CARGO_MANIFEST_DIR"))),
        );
    fs::write(project_root.join("Cargo.toml"), manifest)
        .expect("compile-contract infrastructure: fixture manifest write failed");
    fs::write(project_root.join("Cargo.lock"), FIXTURE_LOCK)
        .expect("compile-contract infrastructure: fixture lock write failed");
    fs::write(project_root.join("main.rs"), source)
        .expect("compile-contract infrastructure: fixture source write failed");

    let target_root = TARGET_ROOT.get_or_init(|| {
        tempfile::Builder::new()
            .prefix("qubit-function-contract-target-")
            .tempdir()
            .expect("compile-contract infrastructure: shared target directory creation failed")
    });
    let feature_key = if normalized_features.is_empty() {
        "no-features".to_owned()
    } else {
        normalized_features.join("-")
    };
    let target_dir = target_root.path().join(feature_key);

    let mut command = Command::new("cargo");
    command
        .args([
            "+1.94.0",
            "check",
            "--offline",
            "--locked",
            "--quiet",
            "--no-default-features",
            "--manifest-path",
        ])
        .arg(project_root.join("Cargo.toml"))
        .args(["--target-dir"])
        .arg(target_dir);
    if !normalized_features.is_empty() {
        command.arg("--features").arg(normalized_features.join(","));
    }
    let output = command
        .current_dir(&project_root)
        .output()
        .expect("compile-contract infrastructure: failed to invoke cargo +1.94.0");
    assert_no_infrastructure_failure(&output);
    output
}

fn normalize_features(features: &[&str]) -> Vec<String> {
    let mut normalized = features
        .iter()
        .map(|feature| {
            assert!(
                SUPPORTED_FEATURES.contains(feature),
                "compile-contract infrastructure: unsupported feature {feature:?}; supported: {SUPPORTED_FEATURES:?}"
            );
            (*feature).to_owned()
        })
        .collect::<Vec<_>>();
    normalized.sort_unstable();
    normalized.dedup();
    normalized
}

fn toml_basic_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{09}' => escaped.push_str("\\t"),
            '\u{0a}' => escaped.push_str("\\n"),
            '\u{0c}' => escaped.push_str("\\f"),
            '\u{0d}' => escaped.push_str("\\r"),
            character if character.is_control() => {
                use std::fmt::Write as _;
                write!(escaped, "\\u{:04X}", character as u32).expect("writing to a String cannot fail");
            }
            character => escaped.push(character),
        }
    }
    escaped
}

fn assert_no_infrastructure_failure(output: &Output) {
    if output.status.success() {
        return;
    }
    let diagnostics = format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let infrastructure_markers = [
        "no matching package named",
        "failed to download",
        "failed to download from",
        "unable to update",
        "lock file needs to be updated",
        "lock file `",
        "is not installed",
        "toolchain '1.94.0'",
        "could not find requested toolchain",
    ];
    if infrastructure_markers.iter().any(|marker| diagnostics.contains(marker)) {
        panic!("compile-contract infrastructure failure; this is not a compiler contract result:\n{diagnostics}");
    }
}
