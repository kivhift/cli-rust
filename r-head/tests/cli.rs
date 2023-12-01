use std::error::Error;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "r-head";
const INPUT_DIR: &str = "tests/inputs";
const EXPECTED_DIR: &str = "tests/expected";
const TEST_INPUTS: &[&str] = &[
    "UTF-8-demo.txt",
    "empty.txt",
    "one.txt",
    "two.txt",
    "three.txt",
    "ten.txt",
];

fn run(base: &str, args: &[&str]) -> TestResult {
    let input_path = PathBuf::from(INPUT_DIR).join(base);
    let expected_path = PathBuf::from(EXPECTED_DIR)
        .join([base, ".out", &args.join("")].join(""));

    Command::cargo_bin(PROG)?
        .arg(input_path)
        .args(args)
        .assert()
        .success()
        .stdout(predicate::path::eq_file(expected_path));

    Ok(())
}

fn run_stdin(base: &str, args: &[&str]) -> TestResult {
    let input_path = PathBuf::from(INPUT_DIR).join(base);
    let expected_path = PathBuf::from(EXPECTED_DIR)
        .join([base, ".out", &args.join("")].join(""));

    Command::cargo_bin(PROG)?
        .pipe_stdin(input_path)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::path::eq_file(expected_path));

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
    let _ = run(TEST_INPUTS[0], &["-c162"])?;

    Ok(())
}

#[test]
fn all_input() -> TestResult {
    let mut inputs = vec![];
    for input in TEST_INPUTS {
        inputs.push(PathBuf::from(INPUT_DIR).join(input));
    }

    Command::cargo_bin(PROG)?
        .args(&inputs)
        .assert()
        .success()
        .stdout(predicate::path::eq_file(
            PathBuf::from(EXPECTED_DIR).join("all.out")
        ));

    Command::cargo_bin(PROG)?
        .args(&inputs)
        .arg("-c1")
        .assert()
        .success()
        .stdout(predicate::path::eq_file(
            PathBuf::from(EXPECTED_DIR).join("all.out-c1")
        ));

    for i in &[2, 4] {
        for a in &["c", "n"] {
            let arg = format!("-{}{}", a, i);

            Command::cargo_bin(PROG)?
                .args(&inputs)
                .arg(&arg)
                .assert()
                .success()
                .stdout(predicate::path::eq_file(
                    PathBuf::from(EXPECTED_DIR).join(format!("all.out{}", arg))
                ));
        }
    }

    Ok(())
}
