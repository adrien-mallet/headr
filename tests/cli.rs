use std::fs;

use assert_cmd::Command;

#[test]
fn simple_head_test() {
    let output = "tests/expected/simple.output";
    let resource = "tests/resources/lipsum";
    run_test(output, &[resource])
}

#[test]
fn double_head_test() {
    let output = "tests/expected/double.output";
    let resource = "tests/resources/lipsum";
    run_test(output, &[resource, resource])
}

#[test]
fn n_5_test() {
    let output = "tests/expected/n_5.output";
    let resource = "tests/resources/lipsum";
    run_test(output, &["-n", "5", resource])
}

#[test]
fn n_5_end_test() {
    let output = "tests/expected/n_-5.output";
    let resource = "tests/resources/lipsum";
    run_test(output, &["-n-5", resource])
}

#[test]
fn c_6_test() {
    let output = "tests/expected/c_6.output";
    let resource = "tests/resources/lipsum";
    run_test(output, &["-c", "6", resource])
}

#[test]
fn c_2_end_test() {
    let output = "tests/expected/c_-6.output";
    let resource = "tests/resources/lipsum";
    run_test(output, &["-c-6", resource])
}

fn run_test(expected: &str, args: &[&str]) {
    let mut cmd = Command::cargo_bin("headr").unwrap();
    let stdout = cmd.args(args).output().expect("Headr output error").stdout;
    assert_eq!(
        fs::read_to_string(expected).expect("Resource test file not found"),
        String::from_utf8(stdout).expect("Invalid utf8 string")
    );
}
