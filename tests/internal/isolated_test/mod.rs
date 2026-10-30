// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::env;
use std::fs;
use std::process::Child;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const ISOLATED_TEST_ENV: &str = "QUBIT_FUNCTION_ISOLATED_TEST";
const ISOLATED_RESULT_ENV: &str = "QUBIT_FUNCTION_ISOLATED_RESULT";
const POLL_INTERVAL: Duration = Duration::from_millis(10);
const TEST_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) fn run_isolated(test_name: &str, body: impl FnOnce()) {
    run_isolated_with_timeout(test_name, TEST_TIMEOUT, body);
}

fn run_isolated_with_timeout(test_name: &str, timeout: Duration, body: impl FnOnce()) {
    if env::var(ISOLATED_TEST_ENV).as_deref() == Ok(test_name) {
        body();
        let result_path = env::var_os(ISOLATED_RESULT_ENV)
            .unwrap_or_else(|| panic!("isolated test {test_name}: result path is missing"));
        fs::write(result_path, test_name)
            .unwrap_or_else(|error| panic!("isolated test {test_name}: cannot write result marker: {error}"));
        return;
    }

    let result_dir = tempfile::Builder::new()
        .prefix("qubit-function-isolated-test-")
        .tempdir()
        .unwrap_or_else(|error| panic!("isolated test {test_name}: cannot create result directory: {error}"));
    let result_path = result_dir.path().join("completed-test");
    let mut child = ChildGuard::new(
        Command::new(
            env::current_exe()
                .unwrap_or_else(|error| panic!("isolated test {test_name}: cannot locate test executable: {error}")),
        )
        .args(["--exact", test_name, "--nocapture"])
        .env(ISOLATED_TEST_ENV, test_name)
        .env(ISOLATED_RESULT_ENV, &result_path)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap_or_else(|error| panic!("isolated test {test_name}: cannot start child process: {error}")),
    );
    let started = Instant::now();

    loop {
        match child
            .child_mut()
            .try_wait()
            .unwrap_or_else(|error| panic!("isolated test {test_name}: cannot check child process: {error}"))
        {
            Some(status) => {
                child.take();
                assert!(
                    status.success(),
                    "isolated test {test_name}: child exited with {status}"
                );
                let completed_name = fs::read_to_string(&result_path)
                    .unwrap_or_else(|error| panic!("isolated test {test_name}: completion marker is missing: {error}"));
                assert_eq!(
                    completed_name, test_name,
                    "isolated test {test_name}: wrong completion marker"
                );
                return;
            }
            None if started.elapsed() >= timeout => {
                child
                    .kill_and_wait()
                    .unwrap_or_else(|error| panic!("isolated test {test_name}: timeout cleanup failed: {error}"));
                panic!("isolated test {test_name}: child exceeded {timeout:?}");
            }
            None => thread::sleep(POLL_INTERVAL),
        }
    }
}

struct ChildGuard {
    child: Option<Child>,
}

impl ChildGuard {
    fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }

    fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("child process is present")
    }

    fn take(&mut self) {
        self.child.take();
    }

    fn kill_and_wait(&mut self) -> std::io::Result<()> {
        let Some(child) = self.child.as_mut() else {
            return Ok(());
        };
        if child.try_wait()?.is_some() {
            self.child.take();
            return Ok(());
        }
        let child = self.child.as_mut().expect("child process is present");
        child.kill()?;
        child.wait()?;
        self.child.take();
        Ok(())
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take()
            && child.try_wait().ok().flatten().is_none()
        {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::run_isolated;
    use super::run_isolated_with_timeout;

    fn exact_test_name(function_name: &str) -> String {
        let qualified = format!("{}::{function_name}", module_path!());
        let (_, exact_name) = qualified.split_once("::").expect("test path has crate prefix");
        exact_name.to_owned()
    }

    #[test]
    fn test_run_isolated_completes_and_marks_the_test() {
        run_isolated(
            &exact_test_name("test_run_isolated_completes_and_marks_the_test"),
            || {},
        );
    }

    #[test]
    fn test_run_isolated_reports_child_panic() {
        let test_name = exact_test_name("test_run_isolated_reports_child_panic");
        let result = std::panic::catch_unwind(|| {
            run_isolated(&test_name, || panic!("intentional isolated panic"));
        });
        assert!(result.is_err(), "child panic must fail the parent helper call");
    }

    #[test]
    fn test_run_isolated_rejects_unmatched_test_name() {
        let test_name = exact_test_name("test_run_isolated_rejects_unmatched_test_name");
        let invalid_name = format!("{test_name}::not-a-test");
        let result = std::panic::catch_unwind(|| {
            run_isolated_with_timeout(&invalid_name, Duration::from_secs(1), || {});
        });
        assert!(result.is_err(), "an unmatched test name must not pass with zero tests");
    }

    #[test]
    fn test_run_isolated_terminates_a_timed_out_child() {
        let test_name = exact_test_name("test_run_isolated_terminates_a_timed_out_child");
        let result = std::panic::catch_unwind(|| {
            run_isolated_with_timeout(&test_name, Duration::from_millis(100), || {
                loop {
                    std::thread::park();
                }
            });
        });
        assert!(result.is_err(), "a child exceeding its deadline must be terminated");
    }
}
