// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::fs;
use std::path::Path;
use std::process::Command;

mod support;

#[test]
fn test_compile_contract_cache_does_not_leave_temporary_targets() {
    let sandbox = tempfile::Builder::new()
        .prefix("qubit-function-cache-regression-")
        .tempdir()
        .expect("regression sandbox creation failed");
    let sentinel = sandbox.path().join("parent-sentinel");
    fs::write(&sentinel, b"keep").expect("sentinel creation failed");
    let binary = std::env::current_exe().expect("test executable path unavailable");

    for _ in 0..2 {
        let output = Command::new(&binary)
            .args([
                "--ignored",
                "--exact",
                "test_compile_contract_cache_probe",
                "--nocapture",
            ])
            .env("TMPDIR", sandbox.path())
            .env("TMP", sandbox.path())
            .env("TEMP", sandbox.path())
            .output()
            .expect("cache regression probe failed to start");
        assert!(
            output.status.success(),
            "probe failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        let entries = fs::read_dir(sandbox.path())
            .expect("sandbox listing failed")
            .map(|entry| entry.expect("sandbox entry read failed").path())
            .collect::<Vec<_>>();
        assert_eq!(entries, vec![sentinel.clone()], "child left temporary artifacts");
        assert_eq!(fs::read(&sentinel).expect("sentinel read failed"), b"keep");
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("target/compile-contract/no-features")
                .is_dir(),
            "the documented stable feature cache must exist",
        );
    }
}

#[test]
#[ignore = "invoked explicitly by the parent cache lifecycle regression"]
fn test_compile_contract_cache_probe() {
    let output = support::compile_consumer(&[], "fn main() {}\n");
    assert!(
        output.status.success(),
        "probe compilation failed: {}",
        String::from_utf8_lossy(&output.stderr),
    );
}
