use module_03::dummy::dummy;

#[test]
fn test_dummy() {
    let result1: String = "dummy".to_string();
    let result2: String = dummy();
    assert_eq!(result1, result2);
}
