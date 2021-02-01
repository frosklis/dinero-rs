#[test]
fn test_balance() {
    let args: Vec<String> = vec![
        "executable",
        "bal",
        "-f",
        "tests/demo.ledger",
        "--init-file",
        "tests/example_ledgerrc",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    let res = dinero::run_app(args);
    assert!(res.is_ok());
}
