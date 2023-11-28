use std::error::Error;
use std::fs;
use std::path::PathBuf;

use snapbox::cmd::{Command, cargo_bin};

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "r-head";
const INPUT_DIR: &str = "tests/inputs";
const EXPECTED_DIR: &str = "tests/expected";
const TEST_INPUTS: &[&str] = &[
    "empty.txt",
    "one.txt",
    "two.txt",
    "three.txt",
    "ten.txt",
    "UTF-8-demo.txt",
];

fn run(base: &str, args: &[&str]) -> TestResult {
    let input_path = PathBuf::from(INPUT_DIR).join(base);
    let expected_path = PathBuf::from(EXPECTED_DIR)
        .join([base, ".out", &args.join("")].join(""));

    Command::new(cargo_bin(PROG))
        .arg(input_path)
        .args(args)
        .assert()
        .success()
        .stdout_eq_path(expected_path);

    Ok(())
}

fn run_stdin(base: &str, args: &[&str]) -> TestResult {
    let input_path = PathBuf::from(INPUT_DIR).join(base);
    let expected_path = PathBuf::from(EXPECTED_DIR)
        .join([base, ".out", &args.join("")].join(""));
    let input_stdin = fs::read_to_string(input_path)?;

    Command::new(cargo_bin(PROG))
        .args(args)
        .stdin(input_stdin)
        .assert()
        .success()
        .stdout_eq_path(expected_path);

    Ok(())
}

#[test]
fn arg_input() -> TestResult {
    for input in TEST_INPUTS {
        let _ = run(input, &[])?;
        let _ = run(input, &["-c1"])?;
        for i in &[2, 4] {
            let _ = run(input, &[&format!("-c{}", i)])?;
            let _ = run(input, &[&format!("-n{}", i)])?;
        }
    }

    Ok(())
}

#[test]
fn stdin_input() -> TestResult {
    for input in TEST_INPUTS {
        let _ = run_stdin(input, &[])?;
        let _ = run_stdin(input, &["-c1"])?;
        for i in &[2, 4] {
            let _ = run_stdin(input, &[&format!("-c{}", i)])?;
            let _ = run_stdin(input, &[&format!("-n{}", i)])?;
        }
    }

    Ok(())
}

#[test]
fn utf8_split() -> TestResult {
    let _ = run(TEST_INPUTS[TEST_INPUTS.len() - 1], &["-c162"])?;

    Ok(())
}
