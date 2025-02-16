use module_01::collections::collections;

#[test]
fn test_collections() {
    let expected_output:i32 = 0;
    let output:i32 = collections();
    assert_eq!(output, expected_output);
}
