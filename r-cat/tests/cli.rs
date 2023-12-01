use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

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
fn usage() {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PROG).unwrap()
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage: "));
    }
}

#[test]
fn skips_bad_file() {
    let bad = "__a_file_that_should_not_be_there__";
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PROG).unwrap()
        .arg(bad)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected).unwrap());
}

fn run(base: &str, args: &[&str]) {
    let input_path = PathBuf::from(INPUT_DIR).join(base);
    let expected_path = PathBuf::from(EXPECTED_DIR)
        .join([base, ".out", &args.join("")].join(""));

    Command::cargo_bin(PROG).unwrap()
        .arg(input_path)
        .args(args)
        .assert()
        .success()
        .stdout(predicate::path::eq_file(expected_path));
}

fn run_stdin(base: &str, args: &[&str]) {
    let input_path = PathBuf::from(INPUT_DIR).join(base);
    let expected_path = PathBuf::from(EXPECTED_DIR)
        .join([base, ".out.stdin", &args.join("")].join(""));

    Command::cargo_bin(PROG).unwrap()
        .pipe_stdin(input_path).unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(predicate::path::eq_file(expected_path));
}

#[test]
fn arg_input() {
    for input in TEST_INPUTS {
        let _ = run(input, &[]);
        let _ = run(input, &["-b"]);
        let _ = run(input, &["-n"]);
    }
}

#[test]
fn stdin_input() {
    for input in TEST_INPUTS {
        let _ = run_stdin(input, &[]);
        let _ = run_stdin(input, &["-b"]);
        let _ = run_stdin(input, &["-n"]);
    }
}

#[test]
fn all_input() {
    let mut inputs = vec![];
    for input in TEST_INPUTS {
        inputs.push(PathBuf::from(INPUT_DIR).join(input));
    }

    Command::cargo_bin(PROG).unwrap()
        .args(&inputs)
        .assert()
        .success()
        .stdout(predicate::path::eq_file(
            PathBuf::from(EXPECTED_DIR).join("all.out")
        ));

    for arg in &["-b", "-n"] {
        let expected_path = PathBuf::from(EXPECTED_DIR)
            .join(["all.out", arg].join(""));

        Command::cargo_bin(PROG).unwrap()
            .arg(arg)
            .args(&inputs)
            .assert()
            .success()
            .stdout(predicate::path::eq_file(expected_path));
    }
}
