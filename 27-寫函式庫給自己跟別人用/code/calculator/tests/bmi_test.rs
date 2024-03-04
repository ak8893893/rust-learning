extern crate calculator;

#[test]
fn bmi_test() {
    let result = calculator::bmi(180, 65);
    assert_eq!(result, Ok(20.1))
}

#[test]
fn bmi_test_invalid() {
    let result = calculator::bmi(0, 65);
    assert!(result.is_err());
}