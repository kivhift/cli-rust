use std::error::Error;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "r-cat";
const INPUT_DIR: &str = "tests/inputs";
const EXPECTED_DIR: &str = "tests/expected";
const TEST_INPUTS: &[&str] = &[
    "UTF-8-demo.txt",
    "empty.txt",
    "fox.txt",
    "spiders.txt",
    "the-bustle.txt",
];

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PROG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage: "));
    }

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = "__a_file_that_should_not_be_there__";
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PROG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}

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
        .join([base, ".out.stdin", &args.join("")].join(""));

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
        let _ = run(input, &["-b"])?;
        let _ = run(input, &["-n"])?;
    }

    Ok(())
}

#[test]
fn stdin_input() -> TestResult {
    for input in TEST_INPUTS {
        let _ = run_stdin(input, &[])?;
        let _ = run_stdin(input, &["-b"])?;
        let _ = run_stdin(input, &["-n"])?;
    }

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

    for arg in &["-b", "-n"] {
        let expected_path = PathBuf::from(EXPECTED_DIR)
            .join(["all.out", arg].join(""));

        Command::cargo_bin(PROG)?
            .arg(arg)
            .args(&inputs)
            .assert()
            .success()
            .stdout(predicate::path::eq_file(expected_path));
    }

    Ok(())
}
