use assert_cmd::Command;
use predicates::prelude::*;

fn assert_cmd_success_from_string_with_expected_output(mut cmd: Command, input: String, expected_output: String) {
    cmd.arg(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected_output.as_str()));
}

fn assert_cmd_success_from_vector_with_expected_output(mut cmd: Command, input: Vec<&str>, expected_output: &str) {
    cmd.args(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected_output));
}

#[test]
fn echo_input_string_with_new_line() {
    let cmd = Command::cargo_bin("recho").unwrap();
    let string_input = String::from("example input");
    let expected_output = string_input.clone() + "\n";
    
    assert_cmd_success_from_string_with_expected_output(cmd, string_input, expected_output)
}

#[test]
fn echo_input_string_no_trailing_newline() {
    let cmd = Command::cargo_bin("recho").unwrap();
    let multiple_arguments_input = vec!["example", "input", "-n"];
    let expected_output = "example input";

    assert_cmd_success_from_vector_with_expected_output(cmd, multiple_arguments_input, expected_output)
}

#[test]
fn echo_input_string_mixed_argument_order() {
    let cmd = Command::cargo_bin("recho").unwrap();
    let multiple_arguments_input = vec!["-n", "example", "input"];
    let expected_output = "example input";

    assert_cmd_success_from_vector_with_expected_output(cmd, multiple_arguments_input, expected_output)
}

#[test]
fn empty_input_string() {
    let mut cmd = Command::cargo_bin("recho").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::eq("\n"));
}

#[test]
fn unexpected_argument() {
    let mut cmd = Command::cargo_bin("recho").unwrap();
    let unexpected_argument = "-q";

    cmd.arg(unexpected_argument)
        .assert()
        .failure()
        .stderr(predicate::str::contains("which wasn't expected"));
}
