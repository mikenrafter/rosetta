use std::io::Write;
use std::process::{Command, Output, Stdio};

fn rosetta_binary() -> &'static str {
    env!("CARGO_BIN_EXE_rosetta")
}

fn run_with_stdin(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(rosetta_binary())
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rosetta");

    child
        .stdin
        .as_mut()
        .expect("stdin pipe should be available")
        .write_all(input)
        .expect("failed to write stdin");

    child.wait_with_output().expect("failed to wait for rosetta")
}

#[test]
fn rejects_stdin_over_cli_limit() {
    let output = run_with_stdin(&["--max-input-bytes", "8"], b"SGVsbG8=!");

    assert!(!output.status.success(), "oversized stdin should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("input exceeds maximum of 8 bytes"),
        "stderr should explain the configured limit, got: {stderr}"
    );
}

#[test]
fn accepts_stdin_at_cli_limit() {
    let output = run_with_stdin(&["--max-input-bytes", "8"], b"SGVsbG8=");

    assert!(
        output.status.success(),
        "boundary-sized stdin should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Base64"), "expected Base64 output, got: {stdout}");
}

#[test]
fn env_override_limits_stdin() {
    let mut child = Command::new(rosetta_binary())
        .env("ROSETTA_MAX_INPUT_BYTES", "8")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rosetta");

    child
        .stdin
        .as_mut()
        .expect("stdin pipe should be available")
        .write_all(b"SGVsbG8=!")
        .expect("failed to write stdin");

    let output = child.wait_with_output().expect("failed to wait for rosetta");

    assert!(!output.status.success(), "env-limited oversized stdin should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("input exceeds maximum of 8 bytes"),
        "stderr should explain the env limit, got: {stderr}"
    );
}

#[test]
fn flags_do_not_break_positional_input() {
    let output = Command::new(rosetta_binary())
        .args(["--max-input-bytes", "8", "SGVsbG8="])
        .output()
        .expect("failed to run rosetta");

    assert!(
        output.status.success(),
        "positional input should still succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Base64"), "expected Base64 output, got: {stdout}");
}
