use module_02::datarace::{datarace01,datarace02};

#[test]
fn test_datarace01() {
    let expected_output:i32 = 0;
    let output:i32 = datarace01();
    assert_eq!(output, expected_output);
}
#[test]
fn test_datarace02() {
    let expected_output:i32 = 0;
    let output:i32 = datarace02();
    assert_eq!(output, expected_output);
}
