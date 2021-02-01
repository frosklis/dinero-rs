#[test]
fn test_balance() {
    let args: Vec<String> = vec!["bal", "-f", "tests/demo.ledger"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let res = dinero::run_app(args);
    assert!(res.is_ok());
}
