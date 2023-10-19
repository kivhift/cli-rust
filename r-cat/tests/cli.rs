use std::error::Error;
use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

const PROG: &str = "r-cat";
const INPUT_DIR: &str = "tests/inputs";
const EXPECTED_DIR: &str = "tests/expected";

const EMPTY: &str = "empty.txt";
const FOX: &str = "fox.txt";
const SPIDERS: &str = "spiders.txt";
const BUSTLE: &str = "the-bustle.txt";
const UTF_8_DEMO: &str = "UTF-8-demo.txt";

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
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}

fn run(args: &[&str], expected: &str) -> TestResult {
    let expected = PathBuf::from(EXPECTED_DIR).join(expected);
    let expected_stdout = fs::read_to_string(expected)?;

    let mut paths: Vec<PathBuf> = vec![];
    for arg in args {
        paths.push(
            if arg.starts_with("-") {
                PathBuf::from(arg)
            } else {
                PathBuf::from(INPUT_DIR).join(arg)
            }
        )
    }

    Command::cargo_bin(PROG)?
        .args(paths)
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

fn run_stdin(input: &str, args: &[&str], tag: &str) -> TestResult {
    let expected = PathBuf::from(EXPECTED_DIR)
        .join(&format!("{}{}.stdin.out", input, tag));
    let input_stdin = fs::read_to_string(PathBuf::from(INPUT_DIR).join(input))?;
    let expected_stdout = fs::read_to_string(expected)?;

    Command::cargo_bin(PROG)?
        .args(args)
        .write_stdin(input_stdin)
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], &format!("{}.out", EMPTY))
}

#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], &format!("{}.b.out", EMPTY))
}

#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], &format!("{}.n.out", EMPTY))
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], &format!("{}.out", FOX))
}

#[test]
fn fox_b() -> TestResult {
    run(&[FOX, "-b"], &format!("{}.b.out", FOX))
}

#[test]
fn fox_n() -> TestResult {
    run(&[FOX, "-n"], &format!("{}.n.out", FOX))
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], &format!("{}.out", SPIDERS))
}

#[test]
fn spiders_b() -> TestResult {
    run(&[SPIDERS, "-b"], &format!("{}.b.out", SPIDERS))
}

#[test]
fn spiders_n() -> TestResult {
    run(&[SPIDERS, "-n"], &format!("{}.n.out", SPIDERS))
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], &format!("{}.out", BUSTLE))
}

#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], &format!("{}.b.out", BUSTLE))
}

#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], &format!("{}.n.out", BUSTLE))
}

#[test]
fn utf_8_demo() -> TestResult {
    run(&[UTF_8_DEMO], &format!("{}.out", UTF_8_DEMO))
}

#[test]
fn utf_8_demo_b() -> TestResult {
    run(&[UTF_8_DEMO, "-b"], &format!("{}.b.out", UTF_8_DEMO))
}

#[test]
fn utf_8_demo_n() -> TestResult {
    run(&[UTF_8_DEMO, "-n"], &format!("{}.n.out", UTF_8_DEMO))
}

#[test]
fn all() -> TestResult {
    run(&[UTF_8_DEMO, EMPTY, FOX, SPIDERS, BUSTLE], "all.out")
}

#[test]
fn all_b() -> TestResult {
    run(&[UTF_8_DEMO, EMPTY, FOX, SPIDERS, BUSTLE, "-b"], "all.b.out")
}

#[test]
fn all_n() -> TestResult {
    run(&[UTF_8_DEMO, EMPTY, FOX, SPIDERS, BUSTLE, "-n"], "all.n.out")
}

#[test]
fn empty_stdin() -> TestResult {
    run_stdin(EMPTY, &[], "")
}

#[test]
fn empty_b_stdin() -> TestResult {
    run_stdin(EMPTY, &["-b"], ".b")
}

#[test]
fn empty_n_stdin() -> TestResult {
    run_stdin(EMPTY, &["-n"], ".n")
}

#[test]
fn fox_stdin() -> TestResult {
    run_stdin(FOX, &[], "")
}

#[test]
fn fox_b_stdin() -> TestResult {
    run_stdin(FOX, &["-b"], ".b")
}

#[test]
fn fox_n_stdin() -> TestResult {
    run_stdin(FOX, &["-n"], ".n")
}

#[test]
fn spiders_stdin() -> TestResult {
    run_stdin(SPIDERS, &[], "")
}

#[test]
fn spiders_b_stdin() -> TestResult {
    run_stdin(SPIDERS, &["-b"], ".b")
}

#[test]
fn spiders_n_stdin() -> TestResult {
    run_stdin(SPIDERS, &["-n"], ".n")
}

#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &[], "")
}

#[test]
fn bustle_b_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-b"], ".b")
}

#[test]
fn bustle_n_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-n"], ".n")
}

#[test]
fn utf_8_demo_stdin() -> TestResult {
    run_stdin(UTF_8_DEMO, &[], "")
}

#[test]
fn utf_8_demo_b_stdin() -> TestResult {
    run_stdin(UTF_8_DEMO, &["-b"], ".b")
}

#[test]
fn utf_8_demo_n_stdin() -> TestResult {
    run_stdin(UTF_8_DEMO, &["-n"], ".n")
}
