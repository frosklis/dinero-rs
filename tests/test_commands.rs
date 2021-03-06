use assert_cmd::Command;
use common::{test_args, test_err};
mod common;
#[test]
fn date_filters() {
    let args1 = &["bal", "-f", "tests/example_files/demo.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args1).assert();
    let mut output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 17);
    test_args(args1);
    let args2 = &[
        "bal",
        "-f",
        "tests/example_files/demo.ledger",
        "-e",
        "2021-01-17",
        "-b",
        "2021-01-15",
        "--force-color",
    ];
    let assert_2 = Command::cargo_bin("dinero").unwrap().args(args2).assert();
    output = String::from_utf8(assert_2.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 12);

    test_args(args2);
}

/// A test for [issue 18](https://github.com/frosklis/dinero-rs/issues/18)
#[test]
fn exchange() {
    let mut outputs = Vec::new();
    for _ in 0..100 {
        let args = &[
            "bal",
            "-f",
            "tests/example_files/demo.ledger",
            "-X",
            "EUR",
            "--force-color",
        ];
        let assert = Command::cargo_bin("dinero").unwrap().args(args).assert();
        outputs.push(String::from_utf8(assert.get_output().to_owned().stdout).unwrap());
        test_args(args);
    }
    for i in 1..100 {
        assert_eq!(outputs[i], outputs[0], "output mismatch");
    }
}

/// A test for [issue 17](https://github.com/frosklis/dinero-rs/issues/17)
/// the aliases should not care about uppercase / lowercase
#[test]
fn commodity_alias() {
    let mut outputs = Vec::new();
    let aliases = vec!["EUR", "eur"];
    for alias in aliases {
        let args = &["bal", "-f", "tests/example_files/demo.ledger", "-X", alias];
        let assert = Command::cargo_bin("dinero").unwrap().args(args).assert();
        outputs.push(String::from_utf8(assert.get_output().to_owned().stdout).unwrap());
        test_args(args);
    }
    assert_eq!(outputs[0], outputs[1], "output mismatch");
}

#[test]
/// Check that the register report is showing virtual postings
fn virtual_postings() {
    let args = &["reg", "-f", "tests/example_files/virtual_postings.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    test_args(args);
    assert_eq!(output.lines().into_iter().count(), 7);
}

#[test]
/// Check that the virtual postings are being filtered out
fn real_filter() {
    let args = &[
        "reg",
        "-f",
        "tests/example_files/virtual_postings.ledger",
        "--real",
    ];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 4);

    test_args(args);
}

#[test]
/// Check that the tag filter works
fn tag_filter() {
    let args = &[
        "bal",
        "-f",
        "tests/example_files/demo.ledger",
        "--flat",
        "%fruit",
    ];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 4);

    test_args(args);
}

#[test]
/// Check that the tag filter works
fn depth_tree() {
    let args_1 = &[
        "bal",
        "-f",
        "tests/example_files/demo.ledger",
        "--depth",
        "1",
    ];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args_1).assert();
    let output_1 = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output_1.lines().into_iter().count(), 11);
    test_args(args_1);

    let args_2 = &[
        "bal",
        "-f",
        "tests/example_files/demo.ledger",
        "--flat",
        "--depth",
        "1",
    ];
    let assert_2 = Command::cargo_bin("dinero").unwrap().args(args_2).assert();
    let output_2 = String::from_utf8(assert_2.get_output().to_owned().stdout).unwrap();
    assert_eq!(output_2.lines().into_iter().count(), 11);

    test_args(args_2);
}

#[test]
/// Check that the tag filter works
fn account_filter() {
    let args = &[
        "bal",
        "-f",
        "tests/example_files/demo.ledger",
        "--flat",
        "travel",
    ];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 1);

    test_args(args);
}

#[test]
/// Check the accounts command
fn accounts_command() {
    let args = &["accounts", "-f", "tests/example_files/demo.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 7);

    test_args(args);
}

#[test]
/// Check the check command
fn check_command() {
    let args = &["check", "-f", "tests/example_files/demo.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 1);

    test_args(args);
}

#[test]
/// Check the check command
fn check_command_bad_file() {
    let args = &["check", "-f", "tests/example_files/demo_bad.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    let output_err = String::from_utf8(assert_1.get_output().to_owned().stderr).unwrap();
    assert_eq!(output.lines().into_iter().count(), 1);
    assert_eq!(output_err.lines().into_iter().count(), 4);
}

#[test]
#[should_panic]
/// Check the check command
fn test_command_bad_file() {
    let args = &["check", "-f", "tests/example_files/demo_bad.ledger"];
    test_args(args);
}

#[test]
/// Check the prices command
fn prices_command() {
    let args = &["prices", "-f", "tests/example_files/demo.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 7);

    test_args(args);
}

#[test]
/// Check the payees command
fn payees_command() {
    let args = &["payees", "-f", "tests/example_files/demo.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(
        output.lines().into_iter().count(),
        5,
        "Because of the aliases, there should be only 5 payees, not 6."
    );

    test_args(args);
}

#[test]
/// Check the commodities command
fn commodities_command() {
    let args = &["commodities", "-f", "tests/example_files/demo.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();

    assert_eq!(output.lines().into_iter().count(), 7);
    test_args(args);
}

#[test]
/// If this fails it means that it created an extra posting
fn automated_fail() {
    let args = &["reg", "-f", "tests/example_files/automated_fail.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output_err = String::from_utf8(assert_1.get_output().to_owned().stderr).unwrap();
    assert_eq!(output_err.lines().into_iter().count(), 5);

    test_err(args);
}

#[test]
fn automated_value_expression() {
    let args = &["reg", "-f", "tests/example_files/automated.ledger"];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 11);

    test_args(args);
}

#[test]
fn automated_add_tag() {
    let args = &[
        "reg",
        "-f",
        "tests/example_files/automated.ledger",
        "%yummy",
    ];
    let assert_1 = Command::cargo_bin("dinero").unwrap().args(args).assert();
    let output = String::from_utf8(assert_1.get_output().to_owned().stdout).unwrap();
    assert_eq!(output.lines().into_iter().count(), 2);

    test_args(args);
}
